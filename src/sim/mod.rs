pub mod world;
pub mod agent;
pub mod event;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};

use crate::gen::prose_gen;
use crate::sim::agent::Agent;
use crate::sim::event::{Event, EventType};
use crate::sim::world::World;

/// Maximum number of events kept in the log ring buffer.
const MAX_EVENTS: usize = 200;

/// Simulation speed settings.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SimSpeed {
    Paused,
    Run1x,
    Run5x,
    Run20x,
}

impl SimSpeed {
    pub fn ticks_per_frame(self) -> u32 {
        match self {
            SimSpeed::Paused => 0,
            SimSpeed::Run1x => 1,
            SimSpeed::Run5x => 5,
            SimSpeed::Run20x => 20,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            SimSpeed::Paused => "PAUSED",
            SimSpeed::Run1x => "1x",
            SimSpeed::Run5x => "5x",
            SimSpeed::Run20x => "20x",
        }
    }
}

/// Which UI overlay is currently active (if any).
#[derive(Debug, Clone, PartialEq)]
pub enum Overlay {
    None,
    /// Inspecting a specific agent by index into the agents vec.
    InspectAgent(usize),
    /// Agent search: player is typing a name to find.
    AgentSearch(String),
    /// Export menu.
    ExportMenu,
    /// Export: player is typing a filename prefix.
    ExportInput(String),
}

/// The complete simulation state.
pub struct SimState {
    pub world: World,
    pub agents: Vec<Agent>,
    pub speed: SimSpeed,
    /// Event log (ring buffer, most recent at end).
    pub events: Vec<Event>,
    /// Scroll offset for the log pane (0 = pinned to bottom / auto-scroll).
    pub log_scroll: usize,
    /// Current UI overlay.
    pub overlay: Overlay,
    /// Temporary status message shown in the status bar (clears after a few frames).
    pub status_message: Option<(String, u32)>,
    /// The RNG used for all simulation randomness.
    rng: StdRng,
}

impl SimState {
    pub fn new(world: World, agents: Vec<Agent>) -> Self {
        let rng = StdRng::seed_from_u64(world.seed.wrapping_add(1));
        let genesis = Event {
            tick: 0,
            event_type: EventType::WorldGenesis,
            subject_id: None,
            location: None,
            description: "The world stirs into being. Somewhere, a ledger is opened.".to_string(),
        };
        Self {
            world,
            agents,
            speed: SimSpeed::Paused,
            events: vec![genesis],
            log_scroll: 0,
            overlay: Overlay::None,
            status_message: None,
            rng,
        }
    }

    /// Advance the simulation by one tick.
    pub fn tick(&mut self) {
        self.world.tick += 1;
        let tick = self.world.tick;

        // Build settlement positions for agent goal-seeking.
        let settlement_positions: Vec<(u32, u32)> = self
            .world
            .settlements
            .iter()
            .map(|s| (s.x as u32, s.y as u32))
            .collect();

        // Process all agent actions and collect resulting events.
        let mut new_events: Vec<Event> = Vec::new();

        for agent in &mut self.agents {
            let actions = agent.act(&mut self.rng, &self.world.terrain, &settlement_positions);

            for action in actions {
                let loc_name = prose_gen::nearest_settlement_name(
                    action.new_pos.0,
                    action.new_pos.1,
                    &self.world,
                );
                // Look up agent name by id (agent is currently borrowed mutably,
                // so we grab the name before the action or use the id to find it).
                // Since we're iterating agents, we can use the current agent's name.
                let agent_name = agent.name.clone();

                let description = prose_gen::generate_description(
                    &action.event_type,
                    Some(&agent_name),
                    Some(&loc_name),
                    tick,
                    &mut self.rng,
                );

                new_events.push(Event {
                    tick,
                    event_type: action.event_type,
                    subject_id: Some(action.agent_id),
                    location: Some(action.new_pos),
                    description,
                });
            }
        }

        // Weather events — roughly every 50 ticks, pick a random settlement
        if tick % 50 == 0 && !self.world.settlements.is_empty() {
            let idx = self.rng.gen_range(0..self.world.settlements.len());
            let s = &self.world.settlements[idx];
            let loc_name = s.name.clone();
            let description = prose_gen::generate_description(
                &EventType::WeatherEvent,
                None,
                Some(&loc_name),
                tick,
                &mut self.rng,
            );
            new_events.push(Event {
                tick,
                event_type: EventType::WeatherEvent,
                subject_id: None,
                location: Some((s.x as u32, s.y as u32)),
                description,
            });
        }

        // Settlement growth/shrinkage — roughly every 200 ticks
        if tick % 200 == 0 && !self.world.settlements.is_empty() {
            let idx = self.rng.gen_range(0..self.world.settlements.len());
            let s = &self.world.settlements[idx];
            let loc_name = s.name.clone();
            let grows = self.rng.gen_bool(0.6);
            let etype = if grows {
                EventType::SettlementGrew
            } else {
                EventType::SettlementShrank
            };
            let description = prose_gen::generate_description(
                &etype,
                None,
                Some(&loc_name),
                tick,
                &mut self.rng,
            );
            new_events.push(Event {
                tick,
                event_type: etype,
                subject_id: None,
                location: Some((s.x as u32, s.y as u32)),
                description,
            });
        }

        // Census report every 100 ticks
        if tick % 100 == 0 {
            let alive_count = self.agents.iter().filter(|a| a.alive).count();
            let description = format!(
                "The census records {} souls still accounted for. The registrar noted this figure without comment.",
                alive_count
            );
            new_events.push(Event {
                tick,
                event_type: EventType::CensusReport,
                subject_id: None,
                location: None,
                description,
            });
        }

        // If auto-scrolled (offset 0), stay pinned to bottom.
        // If user has scrolled up, don't move their view.
        let was_at_bottom = self.log_scroll == 0;

        // Add new events to the log
        self.events.extend(new_events);

        // Trim to ring buffer size
        if self.events.len() > MAX_EVENTS {
            let drain_count = self.events.len() - MAX_EVENTS;
            self.events.drain(..drain_count);
            // Adjust scroll offset so user's view doesn't jump
            if self.log_scroll > 0 {
                self.log_scroll = self.log_scroll.saturating_sub(drain_count);
            }
        }

        if was_at_bottom {
            self.log_scroll = 0;
        }
    }

    /// Run the appropriate number of ticks for the current speed setting.
    /// frame_count is used to throttle slower speeds (1x runs every 3rd frame).
    pub fn step_frame(&mut self, frame_count: u64) {
        match self.speed {
            SimSpeed::Paused => {}
            SimSpeed::Run1x => {
                // ~10 ticks/sec at 30 FPS
                if frame_count % 3 == 0 {
                    self.tick();
                }
            }
            SimSpeed::Run5x => {
                // ~50 ticks/sec at 30 FPS (actually ~2 per frame * 30 = 60)
                for _ in 0..2 {
                    self.tick();
                }
            }
            SimSpeed::Run20x => {
                for _ in 0..20 {
                    self.tick();
                }
            }
        }

        // Tick down status message timer
        if let Some((_, ref mut ttl)) = self.status_message {
            if *ttl == 0 {
                self.status_message = None;
            } else {
                *ttl -= 1;
            }
        }
    }

    /// Scroll the log up by a given number of lines.
    pub fn scroll_log_up(&mut self, amount: usize) {
        let max_scroll = self.events.len().saturating_sub(1);
        self.log_scroll = (self.log_scroll + amount).min(max_scroll);
    }

    /// Scroll the log down (toward present). 0 = pinned to bottom.
    pub fn scroll_log_down(&mut self, amount: usize) {
        self.log_scroll = self.log_scroll.saturating_sub(amount);
    }

    /// Set a temporary status bar message (shown for ~90 frames / ~3 seconds).
    pub fn set_status_message(&mut self, msg: String) {
        self.status_message = Some((msg, 90));
    }

    /// Find agents at a specific map position.
    pub fn agents_at(&self, x: u32, y: u32) -> Vec<usize> {
        self.agents
            .iter()
            .enumerate()
            .filter(|(_, a)| a.alive && a.x == x && a.y == y)
            .map(|(i, _)| i)
            .collect()
    }

    /// Search agents by name (case-insensitive substring match).
    pub fn search_agents(&self, query: &str) -> Vec<usize> {
        let q = query.to_lowercase();
        self.agents
            .iter()
            .enumerate()
            .filter(|(_, a)| a.alive && a.name.to_lowercase().contains(&q))
            .map(|(i, _)| i)
            .collect()
    }
}
