pub mod world;
pub mod agent;
pub mod event;

use rand::rngs::StdRng;
use rand::SeedableRng;
use serde::{Deserialize, Serialize};

use crate::sim::agent::Agent;
use crate::sim::world::World;

/// Simulation speed settings.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SimSpeed {
    Paused,
    Run1x,
    Run5x,
    Run20x,
}

impl SimSpeed {
    /// How many ticks to run per frame update.
    pub fn ticks_per_frame(self) -> u32 {
        match self {
            SimSpeed::Paused => 0,
            SimSpeed::Run1x => 1,
            SimSpeed::Run5x => 5,
            SimSpeed::Run20x => 20,
        }
    }

    /// Label for the status bar.
    pub fn label(self) -> &'static str {
        match self {
            SimSpeed::Paused => "PAUSED",
            SimSpeed::Run1x => "1x",
            SimSpeed::Run5x => "5x",
            SimSpeed::Run20x => "20x",
        }
    }
}

/// The complete simulation state: world + agents + RNG.
pub struct SimState {
    pub world: World,
    pub agents: Vec<Agent>,
    pub speed: SimSpeed,
    /// Log entries displayed in the live log pane.
    pub log: Vec<String>,
    /// The RNG used for all simulation randomness (reconstructed from seed on load).
    rng: StdRng,
}

impl SimState {
    /// Create a new SimState from a generated world and agents.
    pub fn new(world: World, agents: Vec<Agent>) -> Self {
        let rng = StdRng::seed_from_u64(world.seed.wrapping_add(1));
        Self {
            world,
            agents,
            speed: SimSpeed::Paused,
            log: vec!["The world stirs into being.".to_string()],
            rng,
        }
    }

    /// Advance the simulation by one tick.
    pub fn tick(&mut self) {
        self.world.tick += 1;

        // Build a list of settlement positions for agent goal-seeking.
        let settlement_positions: Vec<(u32, u32)> = self
            .world
            .settlements
            .iter()
            .map(|s| (s.x as u32, s.y as u32))
            .collect();

        // Track deaths this tick for log entries.
        let mut deaths = Vec::new();

        for agent in &mut self.agents {
            let was_alive = agent.alive;
            agent.act(&mut self.rng, &self.world.terrain, &settlement_positions);
            if was_alive && !agent.alive {
                deaths.push(agent.name.clone());
            }
        }

        // Log deaths
        for name in deaths {
            let entry = format!(
                "[Tick {}] {} has departed, their administrative obligations at last concluded.",
                self.world.tick, name
            );
            self.log.push(entry);
        }

        // Periodic log entries to show the world is alive
        if self.world.tick % 100 == 0 {
            let alive_count = self.agents.iter().filter(|a| a.alive).count();
            let entry = format!(
                "[Tick {}] The census records {} souls still accounted for.",
                self.world.tick, alive_count
            );
            self.log.push(entry);
        }

        // Keep log from growing unbounded (retain last 200 entries)
        if self.log.len() > 200 {
            let drain_count = self.log.len() - 200;
            self.log.drain(..drain_count);
        }
    }

    /// Run the appropriate number of ticks for the current speed setting.
    pub fn step_frame(&mut self) {
        let ticks = self.speed.ticks_per_frame();
        for _ in 0..ticks {
            self.tick();
        }
    }
}
