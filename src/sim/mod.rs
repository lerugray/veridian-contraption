pub mod world;
pub mod agent;
pub mod event;
pub mod institution;
pub mod site;
pub mod artifact;
pub mod eschaton;

use std::collections::HashMap;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};

use crate::gen::name_gen;
use crate::gen::prose_gen;
use crate::gen::eschaton_gen;
use crate::sim::eschaton::{EschatonType, EschatonRecord, ESCHATON_COOLDOWN, TENSION_THRESHOLD, COSMO_THRESHOLD};
use crate::sim::agent::Agent;
use crate::sim::artifact::Artifact;
use crate::sim::event::{Event, EventType};
use crate::sim::institution::Institution;
use crate::sim::site::Site;
use crate::sim::world::{Season, World};

/// Maximum number of events kept in the log ring buffer.
const MAX_EVENTS: usize = 200;

/// Number of major events that must accumulate to trigger an era transition.
pub const ERA_THRESHOLD: u32 = 15;

/// Simulation speed settings.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SimSpeed {
    Paused,
    Run05x,
    Run1x,
    Run5x,
    Run10x,
}

impl SimSpeed {
    pub fn ticks_per_frame(self) -> u32 {
        match self {
            SimSpeed::Paused => 0,
            SimSpeed::Run05x => 1,
            SimSpeed::Run1x => 1,
            SimSpeed::Run5x => 5,
            SimSpeed::Run10x => 10,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            SimSpeed::Paused => "PAUSED",
            SimSpeed::Run05x => "0.5x",
            SimSpeed::Run1x => "1x",
            SimSpeed::Run5x => "5x",
            SimSpeed::Run10x => "10x",
        }
    }
}

/// What entity the player is following (persistent, not an overlay).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FollowTarget {
    Agent(u64),
    Institution(u64),
}

/// A single entry in the World Annals — one completed era.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnalsEntry {
    pub era_name: String,
    pub start_tick: u64,
    pub end_tick: u64,
    pub summary: String,
    pub notable_agents: Vec<String>,
    pub notable_institutions: Vec<String>,
    pub defining_event: String,
}

/// Which UI overlay is currently active (if any).
#[derive(Debug, Clone, PartialEq)]
pub enum Overlay {
    None,
    /// Inspecting a specific agent by index into the agents vec. (agent_idx, scroll_offset)
    InspectAgent(usize, usize),
    /// Agent search: player is typing a name to find. (query, selected match index)
    AgentSearch(String, usize),
    /// Browsable list of all living agents. (selected index into filtered list)
    AgentList(usize),
    /// Export menu.
    ExportMenu,
    /// Export: player is typing a filename prefix.
    ExportInput(String),
    /// Save: player is typing a save name (Ctrl+S).
    SaveNameInput(String),
    /// Faction list overlay. (selected index)
    FactionList(usize),
    /// Follow selection: pick agent or institution. (0=agent, 1=institution)
    FollowSelect(usize),
    /// Follow agent picker: browsable agent list. (selected index)
    FollowAgentPick(usize),
    /// Follow institution picker: browsable institution list. (selected index)
    FollowInstitutionPick(usize),
    /// Help screen showing all keybindings.
    Help,
    /// Map legend showing symbol meanings.
    MapLegend,
    /// Site list: browsable list of all sites. (selected index)
    SiteList(usize),
    /// Viewing a site interior. (site index, current floor index)
    SiteView(usize, usize),
    /// World Assessment Report (in-game, via W key). (scroll offset)
    WorldReport(usize),
    /// World Annals: scrollable era history. (scroll offset)
    Annals(usize),
    /// Quit confirm: return to main menu? (selected option: 0=save&return, 1=return, 2=cancel)
    QuitConfirm(usize),
    /// Eschaton confirmation screen. (selected option: 0=confirm, 1=cancel)
    EschatonConfirm(usize),
    /// Faction detail dossier. (institution index into institutions vec, scroll offset)
    FactionDetail(usize, usize),
    /// Viewing a settlement interior. (settlement index into world.settlements vec)
    SettlementView(usize),
}

/// Serializable snapshot of the simulation state for save/load.
#[derive(Serialize, Deserialize)]
pub struct SaveData {
    pub world: World,
    pub agents: Vec<Agent>,
    #[serde(default)]
    pub institutions: Vec<Institution>,
    #[serde(default)]
    pub sites: Vec<Site>,
    #[serde(default)]
    pub artifacts: Vec<Artifact>,
    pub speed: SimSpeed,
    pub events: Vec<Event>,
    pub save_name: Option<String>,
    /// Seed used to reconstruct the RNG on load.
    pub rng_state_seed: u64,
    #[serde(default)]
    pub follow_target: Option<FollowTarget>,
    /// World Annals — completed eras.
    #[serde(default)]
    pub annals: Vec<AnnalsEntry>,
    /// Name of the current (ongoing) era.
    #[serde(default)]
    pub current_era_name: Option<String>,
    /// Tick when the current era began.
    #[serde(default)]
    pub current_era_start: u64,
    /// Major event count accumulated in the current era.
    #[serde(default)]
    pub era_major_events: u32,
    /// History of past Eschaton events.
    #[serde(default)]
    pub eschaton_history: Vec<EschatonRecord>,
    /// Accumulated world tension (drives autonomous Eschaton trigger).
    #[serde(default)]
    pub tension: f32,
    /// Tick of the most recent Eschaton (0 = never).
    #[serde(default)]
    pub last_eschaton_tick: u64,
}

/// The complete simulation state.
pub struct SimState {
    pub world: World,
    pub agents: Vec<Agent>,
    pub institutions: Vec<Institution>,
    pub sites: Vec<Site>,
    pub artifacts: Vec<Artifact>,
    pub speed: SimSpeed,
    /// Event log (ring buffer, most recent at end).
    pub events: Vec<Event>,
    /// Scroll offset for the log pane (0 = pinned to bottom / auto-scroll).
    pub log_scroll: usize,
    /// When the player scrolls up, freeze the view at this event count.
    /// None = live mode (showing latest events).
    pub log_frozen_len: Option<usize>,
    /// Current UI overlay.
    pub overlay: Overlay,
    /// Temporary status message shown in the status bar (clears after a few frames).
    pub status_message: Option<(String, u32)>,
    /// The RNG used for all simulation randomness.
    rng: StdRng,
    /// Name of the current save file (None = unsaved new world).
    pub save_name: Option<String>,
    /// Tick at which last autosave fired (to avoid double-saving).
    pub last_autosave_tick: u64,
    /// Next institution ID to assign.
    pub next_institution_id: u64,
    /// Currently followed entity (None = not following).
    pub follow_target: Option<FollowTarget>,
    /// Speed before pausing (so unpause restores the previous speed).
    pub pre_pause_speed: Option<SimSpeed>,
    /// World Annals — completed eras.
    pub annals: Vec<AnnalsEntry>,
    /// Name of the current (ongoing) era.
    pub current_era_name: String,
    /// Tick when the current era began.
    pub current_era_start: u64,
    /// Major event count accumulated in the current era (triggers era transition at threshold).
    pub era_major_events: u32,
    /// Notable agent names collected during the current era (for annals summary).
    era_notable_agents: Vec<String>,
    /// Notable institution names collected during the current era.
    era_notable_institutions: Vec<String>,
    /// The most significant event description in the current era.
    era_defining_event: Option<String>,
    /// History of past Eschaton events.
    pub eschaton_history: Vec<EschatonRecord>,
    /// Accumulated world tension (drives autonomous Eschaton trigger).
    pub tension: f32,
    /// Tick of the most recent Eschaton (0 = never fired).
    pub last_eschaton_tick: u64,
    /// Frames remaining to show "THE ESCHATON HAS OCCURRED" flash in status bar.
    pub eschaton_flash: u32,
    /// Global frame counter for animations (not saved).
    pub frame_count: u64,
    /// Next agent ID to assign (monotonically increasing).
    pub next_agent_id: u64,
    /// Overlay to return to when closing a transient overlay (e.g. Help opened from SiteView).
    pub pre_overlay: Option<Box<Overlay>>,
    /// The season at the end of the previous tick (for detecting transitions).
    pub last_season: Season,
    /// Per-settlement weather template suppression: maps settlement index -> (last_template_id, tick).
    weather_template_history: HashMap<usize, (u8, u64)>,
    /// Per-settlement arrival template suppression: maps settlement index -> (last_template_id, tick).
    arrival_template_history: HashMap<usize, (u8, u64)>,
    /// Per-settlement departure template suppression: maps settlement index -> (last_template_id, tick).
    departure_template_history: HashMap<usize, (u8, u64)>,
}

impl SimState {
    pub fn new(world: World, agents: Vec<Agent>, institutions: Vec<Institution>, sites: Vec<Site>, artifacts: Vec<Artifact>) -> Self {
        let mut rng = StdRng::seed_from_u64(world.seed.wrapping_add(1));
        let next_inst_id = institutions.iter().map(|i| i.id + 1).max().unwrap_or(0);
        let next_agent_id = agents.iter().map(|a| a.id + 1).max().unwrap_or(0);
        let genesis = Event {
            tick: 0,
            event_type: EventType::WorldGenesis,
            subject_id: None,
            location: None,
            description: "The world stirs into being. Somewhere, a ledger is opened.".to_string(),
        };
        let first_era = name_gen::generate_era_name(0, &mut rng);
        Self {
            world,
            agents,
            institutions,
            sites,
            artifacts,
            speed: SimSpeed::Paused,
            events: vec![genesis],
            log_scroll: 0,
            log_frozen_len: None,
            overlay: Overlay::None,
            status_message: None,
            rng,
            save_name: None,
            last_autosave_tick: 0,
            next_institution_id: next_inst_id,
            follow_target: None,
            pre_pause_speed: None,
            annals: Vec::new(),
            current_era_name: first_era,
            current_era_start: 0,
            era_major_events: 0,
            era_notable_agents: Vec::new(),
            era_notable_institutions: Vec::new(),
            era_defining_event: None,
            eschaton_history: Vec::new(),
            tension: 0.0,
            last_eschaton_tick: 0,
            eschaton_flash: 0,
            frame_count: 0,
            next_agent_id,
            pre_overlay: None,
            last_season: Season::Spring,
            weather_template_history: HashMap::new(),
            arrival_template_history: HashMap::new(),
            departure_template_history: HashMap::new(),
        }
    }

    /// Create a serializable snapshot for saving.
    pub fn to_save_data(&self) -> SaveData {
        SaveData {
            world: self.world.clone(),
            agents: self.agents.clone(),
            institutions: self.institutions.clone(),
            sites: self.sites.clone(),
            artifacts: self.artifacts.clone(),
            speed: self.speed,
            events: self.events.clone(),
            save_name: self.save_name.clone(),
            rng_state_seed: self.world.seed.wrapping_add(self.world.tick),
            follow_target: self.follow_target.clone(),
            annals: self.annals.clone(),
            current_era_name: Some(self.current_era_name.clone()),
            current_era_start: self.current_era_start,
            era_major_events: self.era_major_events,
            eschaton_history: self.eschaton_history.clone(),
            tension: self.tension,
            last_eschaton_tick: self.last_eschaton_tick,
        }
    }

    /// Reconstruct a SimState from loaded save data.
    pub fn from_save_data(data: SaveData) -> Self {
        let loaded_season = {
            let cycle = (400.0 / data.world.params.temporal_rate).max(40.0) as u64;
            let (s, _, _) = Season::from_tick(data.world.tick, cycle);
            s
        };
        let mut rng = StdRng::seed_from_u64(data.rng_state_seed);
        let last_tick = data.world.tick;
        let next_inst_id = data.institutions.iter().map(|i| i.id + 1).max().unwrap_or(0);
        let next_agent_id = data.agents.iter().map(|a| a.id + 1).max().unwrap_or(0);
        let era_name = data.current_era_name
            .unwrap_or_else(|| name_gen::generate_era_name(data.annals.len() as u32, &mut rng));
        Self {
            world: data.world,
            agents: data.agents,
            institutions: data.institutions,
            sites: data.sites,
            artifacts: data.artifacts,
            speed: data.speed,
            events: data.events,
            log_scroll: 0,
            log_frozen_len: None,
            overlay: Overlay::None,
            status_message: None,
            rng,
            save_name: data.save_name,
            last_autosave_tick: last_tick,
            next_institution_id: next_inst_id,
            follow_target: data.follow_target,
            pre_pause_speed: None,
            annals: data.annals,
            current_era_name: era_name,
            current_era_start: data.current_era_start,
            era_major_events: data.era_major_events,
            era_notable_agents: Vec::new(),
            era_notable_institutions: Vec::new(),
            era_defining_event: None,
            eschaton_history: data.eschaton_history,
            tension: data.tension,
            last_eschaton_tick: data.last_eschaton_tick,
            eschaton_flash: 0,
            frame_count: 0,
            next_agent_id,
            pre_overlay: None,
            last_season: loaded_season,
            weather_template_history: HashMap::new(),
            arrival_template_history: HashMap::new(),
            departure_template_history: HashMap::new(),
        }
    }

    /// Returns (season, progress 0.0–1.0, ticks_into_season, season_length).
    pub fn season_info(&self) -> (Season, f32, u64, u64) {
        let cycle = self.world.season_cycle_length();
        let (s, p, t) = self.world.current_season();
        (s, p, t, cycle / 4)
    }

    /// Advance the simulation by one tick.
    pub fn tick(&mut self) {
        self.world.tick += 1;
        let tick = self.world.tick;

        // --- Seasonal transition check ---
        let (current_season, _, _) = self.world.current_season();
        if current_season != self.last_season {
            let description = prose_gen::generate_seasonal_transition(
                current_season,
                &mut self.rng,
                self.world.params.narrative_register,
                self.world.params.weirdness_coefficient,
            );
            self.events.push(Event {
                tick,
                event_type: EventType::SeasonalTransition,
                subject_id: None,
                location: None,
                description,
            });
            self.last_season = current_season;
        }

        // Build settlement positions for agent goal-seeking.
        let settlement_positions: Vec<(u32, u32)> = self
            .world
            .settlements
            .iter()
            .map(|s| (s.x as u32, s.y as u32))
            .collect();

        // Build site positions for agent site-seeking.
        let site_positions: Vec<(u32, u32)> = self
            .sites
            .iter()
            .map(|s| (s.grid_x, s.grid_y))
            .collect();

        // Process all agent actions and collect resulting events.
        let mut new_events: Vec<Event> = Vec::new();

        // Winter: agents move less frequently (skip some agents' ticks)
        let winter_skip_chance = if current_season == Season::Winter {
            0.15 * self.world.params.ecological_volatility as f64 // up to 15% of agents skip per tick
        } else {
            0.0
        };

        for agent in &mut self.agents {
            // Winter movement slowdown: some agents skip their turn
            if winter_skip_chance > 0.0 && agent.alive && self.rng.gen_bool(winter_skip_chance.min(0.3)) {
                continue;
            }
            let actions = agent.act(&mut self.rng, &self.world.terrain, &settlement_positions, &site_positions);

            for action in actions {
                let agent_name = agent.display_name();

                // For site events, use the site name instead of nearest settlement
                let description = match &action.event_type {
                    EventType::AgentEnteredSite | EventType::AgentLeftSite => {
                        // Find which site is at this position
                        let site_info = self.sites.iter()
                            .find(|s| s.grid_x == action.new_pos.0 && s.grid_y == action.new_pos.1);
                        let site_name = site_info.map(|s| s.name.as_str()).unwrap_or("an unnamed site");
                        // Pick a room purpose from the first floor if available
                        let room_purpose = site_info.and_then(|s| {
                            s.floors.first().and_then(|f| {
                                if f.rooms.is_empty() { None }
                                else {
                                    let ri = (action.agent_id as usize) % f.rooms.len();
                                    Some(f.rooms[ri].purpose.label())
                                }
                            })
                        });
                        prose_gen::generate_site_description_with_room(
                            &action.event_type,
                            &agent_name,
                            site_name,
                            room_purpose,
                            &mut self.rng,
                            self.world.params.narrative_register,
                            self.world.params.weirdness_coefficient,
                        )
                    }
                    _ => {
                        let loc_name = prose_gen::nearest_settlement_name(
                            action.new_pos.0,
                            action.new_pos.1,
                            &self.world,
                        );
                        // Find nearest settlement index for arrival/departure suppression
                        let nearest_sidx = self.world.settlements.iter().enumerate()
                            .min_by_key(|(_, s)| {
                                let dx = s.x as i32 - action.new_pos.0 as i32;
                                let dy = s.y as i32 - action.new_pos.1 as i32;
                                dx * dx + dy * dy
                            })
                            .map(|(i, _)| i);
                        match action.event_type {
                            EventType::AgentArrived => {
                                let exclude = nearest_sidx.and_then(|si| {
                                    self.arrival_template_history.get(&si)
                                        .and_then(|(tmpl, last_tick)| if tick.saturating_sub(*last_tick) < 50 { Some(*tmpl) } else { None })
                                });
                                let (tmpl_idx, text) = prose_gen::gen_agent_arrived_indexed(
                                    &agent_name, &loc_name,
                                    self.world.params.narrative_register,
                                    self.world.params.weirdness_coefficient,
                                    &mut self.rng, exclude,
                                );
                                if let Some(si) = nearest_sidx {
                                    self.arrival_template_history.insert(si, (tmpl_idx, tick));
                                }
                                text
                            }
                            EventType::AgentDeparted => {
                                let exclude = nearest_sidx.and_then(|si| {
                                    self.departure_template_history.get(&si)
                                        .and_then(|(tmpl, last_tick)| if tick.saturating_sub(*last_tick) < 50 { Some(*tmpl) } else { None })
                                });
                                let (tmpl_idx, text) = prose_gen::gen_agent_departed_indexed(
                                    &agent_name, &loc_name,
                                    self.world.params.narrative_register,
                                    self.world.params.weirdness_coefficient,
                                    &mut self.rng, exclude,
                                );
                                if let Some(si) = nearest_sidx {
                                    self.departure_template_history.insert(si, (tmpl_idx, tick));
                                }
                                text
                            }
                            _ => {
                                prose_gen::generate_description(
                                    &action.event_type,
                                    Some(&agent_name),
                                    Some(&loc_name),
                                    tick,
                                    &mut self.rng,
                                    self.world.params.narrative_register,
                                    self.world.params.weirdness_coefficient,
                                )
                            }
                        }
                    }
                };

                new_events.push(Event {
                    tick,
                    event_type: action.event_type,
                    subject_id: Some(action.agent_id),
                    location: Some(action.new_pos),
                    description,
                });
            }
        }

        // Sync site populations from agent goals.
        // Clear all site populations, then rebuild from agents currently exploring.
        for site in &mut self.sites {
            site.population.clear();
        }
        for agent in &self.agents {
            if !agent.alive { continue; }
            if let agent::Goal::ExploreSite(site_idx, _) = &agent.current_goal {
                if *site_idx < self.sites.len() {
                    self.sites[*site_idx].population.push(agent.id);
                }
            }
        }

        // --- Adventurer artifact simulation ---
        let mut artifact_events = self.process_adventurer_tick(tick);
        new_events.append(&mut artifact_events);

        // --- Seasonal simulation effects ---
        {
            let (season, _, _) = self.world.current_season();
            let eco_vol = self.world.params.ecological_volatility;

            // Winter: extra death chance for older/weaker agents (every 20 ticks)
            if season == Season::Winter && tick % 20 == 0 {
                let extra_death_chance = 0.002 * eco_vol as f64;
                let mut winter_deaths = Vec::new();
                for (idx, agent) in self.agents.iter().enumerate() {
                    if !agent.alive { continue; }
                    // Older agents and those with low health are vulnerable
                    let vulnerability = (agent.age as f64 / 36500.0).min(1.0) * 0.5
                        + (1.0 - agent.health as f64 / 100.0) * 0.5;
                    if self.rng.gen_bool((extra_death_chance * vulnerability).min(0.02)) {
                        winter_deaths.push(idx);
                    }
                }
                for idx in winter_deaths {
                    let agent = &mut self.agents[idx];
                    if !agent.alive { continue; }
                    agent.alive = false;
                    let agent_name = agent.display_name();
                    let agent_id = agent.id;
                    let pos = (agent.x, agent.y);
                    let loc_name = prose_gen::nearest_settlement_name(pos.0, pos.1, &self.world);
                    let description = prose_gen::generate_description(
                        &EventType::NaturalDeath,
                        Some(&agent_name),
                        Some(&loc_name),
                        tick,
                        &mut self.rng,
                        self.world.params.narrative_register,
                        self.world.params.weirdness_coefficient,
                    );
                    new_events.push(Event {
                        tick,
                        event_type: EventType::NaturalDeath,
                        subject_id: Some(agent_id),
                        location: Some(pos),
                        description,
                    });
                }
            }

            // Summer: boost adventurer activity — more agents become adventurers
            if season == Season::Summer && tick % 50 == 0 {
                let boost_chance = 0.03 * eco_vol as f64;
                for agent in &mut self.agents {
                    if !agent.alive || agent.is_adventurer { continue; }
                    if agent.disposition.risk_tolerance > 0.5
                        && self.rng.gen_bool(boost_chance.min(0.1))
                    {
                        agent.is_adventurer = true;
                    }
                }
            }
        }

        // Weather events — interval scaled by temporal_rate and ecological_volatility
        let weather_interval = (50.0 / (self.world.params.temporal_rate * (0.5 + self.world.params.ecological_volatility))).max(5.0) as u64;
        if tick % weather_interval == 0 && !self.world.settlements.is_empty() {
            let sidx = self.rng.gen_range(0..self.world.settlements.len());
            let s = &self.world.settlements[sidx];
            let loc_name = s.name.clone();
            let loc_xy = (s.x as u32, s.y as u32);
            // Near-duplicate suppression: avoid repeating the same template within 50 ticks
            let exclude = self.weather_template_history.get(&sidx)
                .and_then(|(tmpl, last_tick)| if tick.saturating_sub(*last_tick) < 50 { Some(*tmpl) } else { None });
            let (tmpl_idx, description) = prose_gen::gen_weather_indexed(
                &loc_name,
                self.world.params.narrative_register,
                self.world.params.weirdness_coefficient,
                &mut self.rng,
                exclude,
            );
            self.weather_template_history.insert(sidx, (tmpl_idx, tick));
            new_events.push(Event {
                tick,
                event_type: EventType::WeatherEvent,
                subject_id: None,
                location: Some(loc_xy),
                description,
            });
        }

        // Settlement growth/shrinkage — interval scaled by temporal_rate
        let settlement_interval = (200.0 / self.world.params.temporal_rate).max(20.0) as u64;
        if tick % settlement_interval == 0 && !self.world.settlements.is_empty() {
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
                self.world.params.narrative_register,
                self.world.params.weirdness_coefficient,
            );
            new_events.push(Event {
                tick,
                event_type: etype,
                subject_id: None,
                location: Some((s.x as u32, s.y as u32)),
                description,
            });
        }

        // Census report — interval scaled by temporal_rate
        let census_interval = (100.0 / self.world.params.temporal_rate).max(10.0) as u64;
        if tick % census_interval == 0 {
            let alive_count = self.agents.iter().filter(|a| a.alive).count();
            let description = prose_gen::generate_census_with_count(
                alive_count,
                &mut self.rng,
                self.world.params.narrative_register,
            );
            new_events.push(Event {
                tick,
                event_type: EventType::CensusReport,
                subject_id: None,
                location: None,
                description,
            });
        }

        // --- Demographic simulation (births, emigration, immigration) ---
        let mut demo_events = self.process_demographic_tick(tick);
        new_events.append(&mut demo_events);

        // --- Institutional simulation ---
        let mut inst_events = self.process_institutional_tick(tick);
        new_events.append(&mut inst_events);

        // --- Relationship simulation ---
        let mut rel_events = self.process_relationship_tick(tick);
        new_events.append(&mut rel_events);

        // --- Conversation simulation ---
        let mut conv_events = self.process_conversation_tick(tick);
        new_events.append(&mut conv_events);

        // Generate epithets for agents who had notable events this tick.
        // Each agent can gain at most one epithet, and only if 50+ ticks since the last.
        for event in &new_events {
            if let Some(agent_id) = event.subject_id {
                let min_gap: u64 = self.rng.gen_range(50..100);
                let eligible = self.agents.iter().any(|a| {
                    a.id == agent_id
                        && a.alive
                        && tick.saturating_sub(a.last_epithet_tick) >= min_gap
                });

                if eligible {
                    let loc_name = event.location.map(|(x, y)| {
                        prose_gen::nearest_settlement_name(x, y, &self.world)
                    });
                    let epithet = name_gen::generate_epithet_with_weirdness(
                        &event.event_type,
                        loc_name.as_deref(),
                        self.world.params.weirdness_coefficient,
                        &mut self.rng,
                    );
                    if let Some(agent) = self.agents.iter_mut().find(|a| a.id == agent_id) {
                        agent.epithets.push(epithet);
                        agent.last_epithet_tick = tick;
                    }
                }
            }
        }

        // Track major events for era transitions
        for event in &new_events {
            let is_major = matches!(
                event.event_type,
                EventType::AgentDied
                    | EventType::InstitutionFounded
                    | EventType::InstitutionDissolved
                    | EventType::SchismOccurred
                    | EventType::AllianceFormed
                    | EventType::RivalryDeclared
                    | EventType::ArtifactAcquired
                    | EventType::ArtifactDelivered
                    | EventType::AdventurerDiedInSite
                    | EventType::FactionDisbanded
                    | EventType::EschatonFired
            );
            if is_major {
                self.era_major_events += 1;
                // Track notable agents
                if let Some(agent_id) = event.subject_id {
                    if let Some(agent) = self.agents.iter().find(|a| a.id == agent_id) {
                        let name = agent.display_name();
                        if !self.era_notable_agents.contains(&name) && self.era_notable_agents.len() < 8 {
                            self.era_notable_agents.push(name);
                        }
                    }
                }
                // Track notable institutions (from institutional events)
                if matches!(
                    event.event_type,
                    EventType::InstitutionFounded
                        | EventType::InstitutionDissolved
                        | EventType::SchismOccurred
                ) {
                    // Extract institution name from description (first capitalized phrase)
                    for inst in &self.institutions {
                        if event.description.contains(&inst.name)
                            && !self.era_notable_institutions.contains(&inst.name)
                            && self.era_notable_institutions.len() < 6
                        {
                            self.era_notable_institutions.push(inst.name.clone());
                        }
                    }
                }
                // Track defining event (the most recent major event wins)
                self.era_defining_event = Some(event.description.clone());
            }
        }

        // --- Tension accumulation ---
        // Each major event type adds tension; slow decay per tick
        for event in &new_events {
            match event.event_type {
                EventType::AgentDied | EventType::NaturalDeath => self.tension += 0.02,
                EventType::InstitutionDissolved => self.tension += 0.05,
                EventType::SchismOccurred => self.tension += 0.03,
                EventType::RivalryDeclared | EventType::AllianceStrained => self.tension += 0.02,
                EventType::AdventurerDiedInSite => self.tension += 0.01,
                _ => {}
            }
        }
        // Slow decay
        self.tension = (self.tension - 0.001).max(0.0);

        // --- Autonomous Eschaton check (every 50 ticks) ---
        if tick % 50 == 0
            && self.world.params.cosmological_density > COSMO_THRESHOLD
            && self.tension > TENSION_THRESHOLD
            && (self.last_eschaton_tick == 0 || tick - self.last_eschaton_tick >= ESCHATON_COOLDOWN)
        {
            let trigger_chance = (self.tension * 0.1) as f64;
            if self.rng.gen_bool(trigger_chance.min(0.5)) {
                let eschaton_events = self.execute_eschaton(tick);
                new_events.extend(eschaton_events);
            }
        }

        // Check for era transition
        if self.era_major_events >= ERA_THRESHOLD {
            self.transition_era(tick);
        }

        // Add new events to the log
        self.events.extend(new_events);

        // Trim to ring buffer size
        if self.events.len() > MAX_EVENTS {
            let drain_count = self.events.len() - MAX_EVENTS;
            self.events.drain(..drain_count);
            // Adjust frozen view anchor so it still points at the same events
            if let Some(ref mut frozen) = self.log_frozen_len {
                *frozen = frozen.saturating_sub(drain_count);
            }
        }
    }

    /// Transition to a new era — close the current era and begin the next.
    fn transition_era(&mut self, tick: u64) {
        let defining = self.era_defining_event.take()
            .unwrap_or_else(|| "No single event could be identified as definitive.".to_string());

        // Generate a summary paragraph for the completed era
        let summary = self.generate_era_summary(tick, &defining);

        let entry = AnnalsEntry {
            era_name: self.current_era_name.clone(),
            start_tick: self.current_era_start,
            end_tick: tick,
            summary,
            notable_agents: self.era_notable_agents.clone(),
            notable_institutions: self.era_notable_institutions.clone(),
            defining_event: defining,
        };

        self.annals.push(entry);

        // Begin the next era
        let era_number = self.annals.len() as u32;
        self.current_era_name = name_gen::generate_era_name(era_number, &mut self.rng);
        self.current_era_start = tick;
        self.era_major_events = 0;
        self.era_notable_agents.clear();
        self.era_notable_institutions.clear();
        self.era_defining_event = None;

        // Log the era transition
        let era_event = Event {
            tick,
            event_type: EventType::CensusReport, // closest existing type for world-level
            subject_id: None,
            location: None,
            description: format!(
                "A new era has been declared. The records office has filed the previous period under '{}' and opened a fresh ledger for '{}'.",
                self.annals.last().map(|a| a.era_name.as_str()).unwrap_or("Unknown"),
                self.current_era_name
            ),
        };
        self.events.push(era_event);
    }

    /// Generate a prose summary for a completed era.
    fn generate_era_summary(&mut self, end_tick: u64, _defining_event: &str) -> String {
        let alive = self.agents.iter().filter(|a| a.alive).count();
        let dead = self.agents.iter().filter(|a| !a.alive).count();
        let living_inst = self.institutions.iter().filter(|i| i.alive).count();

        let duration = end_tick - self.current_era_start;

        let openings = [
            "This era spanned",
            "The period encompassed",
            "Across the breadth of",
            "Over the course of",
            "During the",
        ];
        let opening = openings[self.rng.gen_range(0..openings.len())];

        let closings = [
            "The registrar noted the transition without comment.",
            "The new era was declared with the customary lack of ceremony.",
            "Several filing cabinets were requisitioned for the archived records.",
            "The transition was observed by those few who had been paying attention.",
            "A minor clerical adjustment was made to the calendar.",
        ];
        let closing = closings[self.rng.gen_range(0..closings.len())];

        let mut parts = Vec::new();
        parts.push(format!(
            "{} {} ticks of recorded history, during which {} souls were accounted for and {} were filed as concluded.",
            opening, duration, alive + dead, dead
        ));

        if living_inst > 0 {
            let inst_phrases = [
                format!("{} institutions persisted through the period.", living_inst),
                format!("The bureaucratic landscape supported {} active organizations.", living_inst),
                format!("{} bodies of varying legitimacy remained operational.", living_inst),
            ];
            parts.push(inst_phrases[self.rng.gen_range(0..inst_phrases.len())].clone());
        }

        if !self.era_notable_agents.is_empty() {
            let count = self.era_notable_agents.len().min(3);
            let names: Vec<&str> = self.era_notable_agents.iter().take(count).map(|s| s.as_str()).collect();
            parts.push(format!(
                "Among those of note: {}.",
                names.join(", ")
            ));
        }

        parts.push(closing.to_string());
        parts.join(" ")
    }

    /// Execute an Eschaton event — the core world-altering function.
    /// Called by both autonomous trigger and player trigger.
    /// Returns the generated log events.
    pub fn execute_eschaton(&mut self, tick: u64) -> Vec<Event> {
        let eschaton_type = EschatonType::random(&mut self.rng);
        let register = self.world.params.narrative_register;
        let weirdness = self.world.params.weirdness_coefficient;
        let phonemes = name_gen::load_phoneme_data();

        // Generate prose events
        let events = eschaton_gen::generate_eschaton_prose(
            &eschaton_type, tick, register, weirdness, &mut self.rng,
        );

        // Apply mechanical effects based on type
        match eschaton_type {
            EschatonType::TheReckoningOfDebts => {
                eschaton_gen::execute_reckoning(
                    &mut self.institutions, &mut self.agents,
                    &mut self.next_institution_id, &phonemes, weirdness, &mut self.rng,
                );
                // Set founded_tick on new institutions
                for inst in &mut self.institutions {
                    if inst.founded_tick == 0 && inst.alive {
                        inst.founded_tick = tick;
                    }
                }
            }
            EschatonType::TheTaxonomicCorrection => {
                eschaton_gen::execute_taxonomic_correction(
                    &mut self.agents, &mut self.world.settlements,
                    &phonemes, &mut self.rng,
                );
            }
            EschatonType::TheAdministrativeSingularity => {
                eschaton_gen::execute_singularity(
                    &mut self.institutions, &mut self.agents,
                    &mut self.next_institution_id, &phonemes, weirdness, &mut self.rng,
                );
                for inst in &mut self.institutions {
                    if inst.founded_tick == 0 && inst.alive {
                        inst.founded_tick = tick;
                    }
                }
            }
            EschatonType::TheGeologicalArgument => {
                eschaton_gen::execute_geological_argument(
                    &mut self.world.terrain, &mut self.world.settlements,
                    &phonemes, &mut self.rng,
                );
            }
            EschatonType::TheDoctrinalCascade => {
                eschaton_gen::execute_doctrinal_cascade(
                    &mut self.institutions, &mut self.agents,
                    &mut self.next_institution_id, &phonemes, weirdness, &mut self.rng,
                );
                for inst in &mut self.institutions {
                    if inst.founded_tick == 0 && inst.alive {
                        inst.founded_tick = tick;
                    }
                }
            }
            EschatonType::TheArrivalOfSomethingOwed => {
                eschaton_gen::execute_arrival(
                    &mut self.agents, &self.world.settlements,
                    self.world.peoples.len(), &phonemes, self.next_agent_id, &mut self.rng,
                );
                // Update next_agent_id to account for any agents added by the eschaton
                self.next_agent_id = self.agents.iter().map(|a| a.id + 1).max().unwrap_or(self.next_agent_id);
            }
        }

        // Record the eschaton
        let era_before = self.current_era_name.clone();

        // Force an era transition
        self.era_defining_event = Some(format!("{} has occurred.", eschaton_type.label()));
        self.transition_era(tick);

        let era_after = self.current_era_name.clone();

        self.eschaton_history.push(EschatonRecord {
            eschaton_type,
            tick,
            era_name_before: era_before,
            era_name_after: era_after,
        });

        // Reset tension and reduce cosmological density
        self.tension = 0.1;
        self.last_eschaton_tick = tick;
        self.world.params.cosmological_density = (self.world.params.cosmological_density - 0.2).max(0.1);

        // Shift other world parameters slightly
        self.world.params.political_churn = (self.world.params.political_churn + self.rng.gen_range(-0.1..0.15)).clamp(0.05, 0.95);
        self.world.params.weirdness_coefficient = (self.world.params.weirdness_coefficient + self.rng.gen_range(-0.05..0.1)).clamp(0.05, 0.95);

        // Set the status bar flash
        self.eschaton_flash = 150; // ~5 seconds at 30fps

        events
    }

    /// Check whether the Eschaton can fire (cooldown check).
    pub fn can_eschaton(&self) -> bool {
        self.last_eschaton_tick == 0 || self.world.tick - self.last_eschaton_tick >= ESCHATON_COOLDOWN
    }

    /// Process demographic events: births, emigration, immigration.
    /// Runs every 10 ticks to reduce overhead. Rates scaled by temporal_rate.
    fn process_demographic_tick(&mut self, tick: u64) -> Vec<Event> {
        let mut events = Vec::new();
        if tick % 10 != 0 || self.world.settlements.is_empty() {
            return events;
        }

        let phonemes = name_gen::load_phoneme_data();
        let register = self.world.params.narrative_register;
        let weirdness = self.world.params.weirdness_coefficient;
        let temporal_rate = self.world.params.temporal_rate;
        let (season, _, _) = self.world.current_season();
        let eco_vol = self.world.params.ecological_volatility;

        // Seasonal modifiers scale with ecological_volatility
        let birth_modifier = match season {
            Season::Spring => 1.0 + 0.3 * eco_vol,  // Spring: increased births
            Season::Winter => 1.0 - 0.2 * eco_vol,   // Winter: fewer births
            _ => 1.0,
        };
        let emigration_modifier = match season {
            Season::Winter => 1.0 + 0.2 * eco_vol,   // Winter: more likely to leave
            _ => 1.0,
        };

        // --- BIRTHS ---
        // For each settlement, chance of a birth proportional to how many agents are nearby.
        // Base chance per settlement per 10-tick check: ~2% * temporal_rate, scaled by local pop.
        let settlement_positions: Vec<(u32, u32)> = self.world.settlements.iter()
            .map(|s| (s.x as u32, s.y as u32))
            .collect();

        // Count agents near each settlement (within 3 tiles)
        let mut settlement_pops: Vec<u32> = vec![0; self.world.settlements.len()];
        for agent in &self.agents {
            if !agent.alive { continue; }
            for (si, &(sx, sy)) in settlement_positions.iter().enumerate() {
                let dx = (agent.x as i32 - sx as i32).unsigned_abs();
                let dy = (agent.y as i32 - sy as i32).unsigned_abs();
                if dx <= 3 && dy <= 3 {
                    settlement_pops[si] += 1;
                    break;
                }
            }
        }

        let alive_count = self.agents.iter().filter(|a| a.alive).count();
        let mut births_this_tick: Vec<(String, usize, u32, u32)> = Vec::new(); // (name, people_id, x, y)

        for (si, pop) in settlement_pops.iter().enumerate() {
            if *pop == 0 { continue; }
            // Birth chance scales with local pop, temporal_rate, and season.
            let birth_chance = (0.003 * temporal_rate as f64 * birth_modifier as f64 * (*pop as f64).sqrt()).min(0.15);
            if self.rng.gen_bool(birth_chance) {
                let people_id = if !self.world.peoples.is_empty() {
                    self.rng.gen_range(0..self.world.peoples.len())
                } else { 0 };
                let name = name_gen::generate_personal_name(&phonemes,
                    if !self.world.peoples.is_empty() { self.world.peoples[people_id].phoneme_set } else { 0 },
                    &mut self.rng);
                let (sx, sy) = settlement_positions[si];
                births_this_tick.push((name, people_id, sx, sy));
            }
        }

        for (name, people_id, sx, sy) in births_this_tick {
            let loc_name = prose_gen::nearest_settlement_name(sx, sy, &self.world);
            let description = prose_gen::generate_description(
                &EventType::AgentBorn,
                Some(&name),
                Some(&loc_name),
                tick,
                &mut self.rng,
                register,
                weirdness,
            );

            let agent_id = self.next_agent_id;
            self.next_agent_id += 1;

            self.agents.push(Agent {
                id: agent_id,
                name: name.clone(),
                people_id,
                x: sx,
                y: sy,
                health: self.rng.gen_range(70..=100),
                age: 0,
                disposition: agent::Disposition::random(&mut self.rng),
                current_goal: agent::Goal::Rest(self.rng.gen_range(20..=60)),
                chronicle: Vec::new(),
                alive: true,
                epithets: Vec::new(),
                last_epithet_tick: 0,
                institution_ids: Vec::new(),
                is_adventurer: false,
                held_artifacts: Vec::new(),
                relationships: Vec::new(),
            conversations: Vec::new(),
            });

            events.push(Event {
                tick,
                event_type: EventType::AgentBorn,
                subject_id: Some(agent_id),
                location: Some((sx, sy)),
                description,
            });
        }

        // --- EMIGRATION ---
        // Agents with high risk_tolerance and low institutional_loyalty are more likely to leave.
        // Base chance per agent per 10 ticks: ~0.05% * temporal_rate, boosted by disposition.
        // Only check a sample of agents to reduce overhead.
        let sample_size = (alive_count / 5).max(1).min(20);
        let mut emigration_indices: Vec<usize> = Vec::new();
        for _ in 0..sample_size {
            let idx = self.rng.gen_range(0..self.agents.len());
            let agent = &self.agents[idx];
            if !agent.alive { continue; }
            // Wanderers and dissidents emigrate more
            let disposition_factor = agent.disposition.risk_tolerance * 1.5
                + (1.0 - agent.disposition.institutional_loyalty) * 0.5
                + agent.disposition.paranoia * 0.3;
            let emigration_chance = 0.001 * temporal_rate as f64 * disposition_factor as f64 * emigration_modifier as f64;
            if self.rng.gen_bool(emigration_chance.min(0.05)) {
                emigration_indices.push(idx);
            }
        }

        for idx in emigration_indices {
            let agent = &mut self.agents[idx];
            if !agent.alive { continue; }
            let agent_name = agent.display_name();
            let agent_id = agent.id;
            let pos = (agent.x, agent.y);
            agent.alive = false;

            let loc_name = prose_gen::nearest_settlement_name(pos.0, pos.1, &self.world);
            let description = prose_gen::generate_emigration(
                &agent_name,
                &loc_name,
                register,
                weirdness,
                &mut self.rng,
            );

            events.push(Event {
                tick,
                event_type: EventType::AgentEmigrated,
                subject_id: Some(agent_id),
                location: Some(pos),
                description,
            });
        }

        // --- IMMIGRATION ---
        // New agents arrive from outside the known world.
        // Base chance per 10-tick check: ~1.5% * temporal_rate.
        let immigration_chance = 0.015 * temporal_rate as f64;
        if self.rng.gen_bool(immigration_chance.min(0.10)) {
            // Arrive at a border settlement (pick a random one)
            let si = self.rng.gen_range(0..self.world.settlements.len());
            let (sx, sy) = settlement_positions[si];
            let people_id = if !self.world.peoples.is_empty() {
                self.rng.gen_range(0..self.world.peoples.len())
            } else { 0 };
            let name = name_gen::generate_personal_name(&phonemes,
                if !self.world.peoples.is_empty() { self.world.peoples[people_id].phoneme_set } else { 0 },
                &mut self.rng);

            let agent_id = self.next_agent_id;
            self.next_agent_id += 1;

            // Immigrants arrive as adults of varying age
            let age = self.rng.gen_range(3650..18250); // ~10-50 years

            let loc_name = prose_gen::nearest_settlement_name(sx, sy, &self.world);
            let description = prose_gen::generate_immigration(
                &name,
                &loc_name,
                register,
                weirdness,
                &mut self.rng,
            );

            self.agents.push(Agent {
                id: agent_id,
                name: name.clone(),
                people_id,
                x: sx,
                y: sy,
                health: self.rng.gen_range(50..=90),
                age,
                disposition: agent::Disposition::random(&mut self.rng),
                current_goal: agent::Goal::Rest(self.rng.gen_range(10..=40)),
                chronicle: Vec::new(),
                alive: true,
                epithets: Vec::new(),
                last_epithet_tick: 0,
                institution_ids: Vec::new(),
                is_adventurer: self.rng.gen_bool(0.1), // small chance of arriving as adventurer
                held_artifacts: Vec::new(),
                relationships: Vec::new(),
            conversations: Vec::new(),
            });

            events.push(Event {
                tick,
                event_type: EventType::AgentImmigrated,
                subject_id: Some(agent_id),
                location: Some((sx, sy)),
                description,
            });
        }

        events
    }

    /// Process institutional events for one tick.
    fn process_institutional_tick(&mut self, tick: u64) -> Vec<Event> {
        let mut events = Vec::new();
        let phonemes = name_gen::load_phoneme_data();

        // Agent goals: JoinInstitution, AdvanceInInstitution, FoundInstitution
        // Check every 10 ticks to reduce per-tick overhead.
        if tick % 10 == 0 {
            let mut founding_agents: Vec<usize> = Vec::new();
            let mut joining_agents: Vec<(usize, u64)> = Vec::new();

            for (ai, agent) in self.agents.iter().enumerate() {
                if !agent.alive { continue; }
                match &agent.current_goal {
                    agent::Goal::FoundInstitution => {
                        if agent.institution_ids.len() < 2 {
                            founding_agents.push(ai);
                        }
                    }
                    agent::Goal::JoinInstitution(inst_id) => {
                        joining_agents.push((ai, *inst_id));
                    }
                    _ => {}
                }
            }

            // Process foundings
            for ai in founding_agents {
                let agent = &self.agents[ai];
                let people_id = agent.people_id;
                let agent_name = agent.display_name();
                let agent_id = agent.id;
                let loc = (agent.x, agent.y);

                let kind = match self.rng.gen_range(0..6) {
                    0 => institution::InstitutionKind::Guild,
                    1 => institution::InstitutionKind::Government,
                    2 => institution::InstitutionKind::Cult,
                    3 => institution::InstitutionKind::MercenaryCompany,
                    4 => institution::InstitutionKind::RegulatoryBody,
                    _ => institution::InstitutionKind::SecretSociety,
                };

                let inst_name = name_gen::generate_institution_name_with_weirdness(&kind, &phonemes, people_id, self.world.params.weirdness_coefficient, &mut self.rng);
                let charter = name_gen::generate_charter(&kind, &mut self.rng);
                let actual_function = name_gen::generate_actual_function(&kind, &mut self.rng);
                let doctrine = name_gen::generate_doctrines(&kind, &mut self.rng);

                let inst_id = self.next_institution_id;
                self.next_institution_id += 1;

                let loc_name = prose_gen::nearest_settlement_name(loc.0, loc.1, &self.world);
                let chronicle_entry = format!(
                    "Founded in {} by {} for the purpose of: {}",
                    loc_name, agent_name, charter
                );

                let inst = Institution {
                    id: inst_id,
                    name: inst_name.clone(),
                    kind,
                    charter,
                    actual_function,
                    power: self.rng.gen_range(5..=20),
                    doctrine,
                    member_ids: vec![agent_id],
                    territory: vec![loc],
                    founded_tick: tick,
                    relationships: std::collections::HashMap::new(),
                    chronicle: vec![chronicle_entry],
                    people_id,
                    alive: true,
                };
                self.institutions.push(inst);

                self.agents[ai].institution_ids.push(inst_id);
                self.agents[ai].current_goal = agent::Goal::Wander;

                let description = prose_gen::generate_institutional_description(
                    &EventType::InstitutionFounded,
                    Some(&agent_name),
                    Some(&inst_name),
                    Some(&loc_name),
                    &mut self.rng,
                    self.world.params.narrative_register,
                    self.world.params.weirdness_coefficient,
                );
                events.push(Event {
                    tick,
                    event_type: EventType::InstitutionFounded,
                    subject_id: Some(agent_id),
                    location: Some(loc),
                    description,
                });
            }

            // Process joins
            for (ai, inst_id) in joining_agents {
                let inst_alive = self.institutions.iter().any(|i| i.id == inst_id && i.alive);
                if !inst_alive || self.agents[ai].institution_ids.len() >= 2 {
                    self.agents[ai].current_goal = agent::Goal::Wander;
                    continue;
                }
                let agent_name = self.agents[ai].display_name();
                let agent_id = self.agents[ai].id;
                let loc = (self.agents[ai].x, self.agents[ai].y);

                if let Some(inst) = self.institutions.iter_mut().find(|i| i.id == inst_id) {
                    if !inst.member_ids.contains(&agent_id) {
                        inst.member_ids.push(agent_id);
                        let loc_name = prose_gen::nearest_settlement_name(loc.0, loc.1, &self.world);
                        inst.chronicle.push(format!("{} joined near {}", agent_name, loc_name));

                        let inst_name = inst.name.clone();
                        self.agents[ai].institution_ids.push(inst_id);

                        let description = prose_gen::generate_institutional_description(
                            &EventType::MemberJoined,
                            Some(&agent_name),
                            Some(&inst_name),
                            None,
                            &mut self.rng,
                            self.world.params.narrative_register,
                            self.world.params.weirdness_coefficient,
                        );
                        events.push(Event {
                            tick,
                            event_type: EventType::MemberJoined,
                            subject_id: Some(agent_id),
                            location: Some(loc),
                            description,
                        });
                    }
                }
                self.agents[ai].current_goal = agent::Goal::Wander;
            }
        }

        // Unaffiliated agents with moderate+ loyalty sometimes seek to join an institution.
        // Check a few agents per tick to spread the load.
        if !self.institutions.is_empty() {
            let alive_institutions: Vec<u64> = self.institutions.iter()
                .filter(|i| i.alive)
                .map(|i| i.id)
                .collect();

            if !alive_institutions.is_empty() {
                for agent in &mut self.agents {
                    if !agent.alive || !agent.institution_ids.is_empty() { continue; }
                    if !matches!(agent.current_goal, agent::Goal::Wander) { continue; }
                    // Protégés get a boosted join chance (mentor smooths the path)
                    let has_mentor = agent.relationships.iter().any(|r| {
                        r.kind == agent::RelationshipKind::Protege
                    });
                    let join_chance = if has_mentor { 0.015 } else { 0.005 };
                    if agent.disposition.institutional_loyalty > 0.4 && self.rng.gen_bool(join_chance) {
                        let inst_id = alive_institutions[self.rng.gen_range(0..alive_institutions.len())];
                        agent.current_goal = agent::Goal::JoinInstitution(inst_id);
                    }
                }
            }
        }

        // Periodic institutional events — interval scaled by temporal_rate, political_churn, and season
        let (season, _, _) = self.world.current_season();
        let eco_vol = self.world.params.ecological_volatility;
        let inst_season_multiplier = match season {
            Season::Autumn => 1.0 + 0.4 * eco_vol, // Autumn: more political activity
            Season::Spring => 1.0 + 0.2 * eco_vol,  // Spring: slight boost to formation
            _ => 1.0,
        };
        let inst_event_interval = (75.0 / (self.world.params.temporal_rate * (0.5 + self.world.params.political_churn) * inst_season_multiplier)).max(5.0) as u64;
        if tick % inst_event_interval == 0 && !self.institutions.is_empty() {
            let alive_indices: Vec<usize> = self.institutions.iter()
                .enumerate()
                .filter(|(_, i)| i.alive)
                .map(|(idx, _)| idx)
                .collect();

            if !alive_indices.is_empty() {
                let inst_idx = alive_indices[self.rng.gen_range(0..alive_indices.len())];
                let event = self.generate_institutional_event(inst_idx, tick, &phonemes);
                if let Some(e) = event {
                    events.push(e);
                }
            }
        }

        // Relationship events — interval scaled by temporal_rate, political_churn, and season
        let relation_interval = (150.0 / (self.world.params.temporal_rate * (0.5 + self.world.params.political_churn) * inst_season_multiplier)).max(10.0) as u64;
        if tick % relation_interval == 0 {
            let alive_ids: Vec<u64> = self.institutions.iter()
                .filter(|i| i.alive)
                .map(|i| i.id)
                .collect();

            if alive_ids.len() >= 2 {
                let a_id = alive_ids[self.rng.gen_range(0..alive_ids.len())];
                let mut b_id = alive_ids[self.rng.gen_range(0..alive_ids.len())];
                let mut attempts = 0;
                while b_id == a_id && attempts < 10 {
                    b_id = alive_ids[self.rng.gen_range(0..alive_ids.len())];
                    attempts += 1;
                }
                if a_id != b_id {
                    if let Some(e) = self.generate_relationship_event(a_id, b_id, tick) {
                        events.push(e);
                    }
                }
            }
        }

        // Member departure/expulsion — interval scaled by temporal_rate and political_churn
        let departure_interval = (80.0 / (self.world.params.temporal_rate * (0.5 + self.world.params.political_churn))).max(5.0) as u64;
        if tick % departure_interval == 0 {
            if let Some(e) = self.process_member_departure(tick) {
                events.push(e);
            }
        }

        // Clean up institutions: remove dead agents from member lists and disband empty/powerless factions
        if tick % 200 == 0 {
            let living_ids: std::collections::HashSet<u64> = self.agents.iter()
                .filter(|a| a.alive)
                .map(|a| a.id)
                .collect();
            for inst in &mut self.institutions {
                if !inst.alive { continue; }
                inst.member_ids.retain(|id| living_ids.contains(id));

                // Factions with 0 members and power < 5 are disbanded
                if inst.member_ids.is_empty() && inst.power < 5 {
                    inst.alive = false;
                    let inst_name = inst.name.clone();
                    inst.chronicle.push(format!("Disbanded at tick {}. Zero members, insufficient resources.", tick));
                    let description = prose_gen::generate_faction_disbanded(
                        &inst_name,
                        &mut self.rng,
                        self.world.params.narrative_register,
                        self.world.params.weirdness_coefficient,
                    );
                    events.push(Event {
                        tick,
                        event_type: EventType::FactionDisbanded,
                        subject_id: None,
                        location: None,
                        description,
                    });
                } else if inst.member_ids.is_empty() {
                    // 0 members but still has power — dissolve with existing prose
                    inst.alive = false;
                    let inst_name = inst.name.clone();
                    inst.chronicle.push("Dissolved due to lack of members.".to_string());
                    events.push(Event {
                        tick,
                        event_type: EventType::InstitutionDissolved,
                        subject_id: None,
                        location: None,
                        description: prose_gen::generate_institutional_description(
                            &EventType::InstitutionDissolved,
                            None,
                            Some(&inst_name),
                            None,
                            &mut self.rng,
                            self.world.params.narrative_register,
                            self.world.params.weirdness_coefficient,
                        ),
                    });
                }
            }
        }

        events
    }

    /// Generate a periodic institutional event (schism, doctrine shift, etc.)
    fn generate_institutional_event(
        &mut self,
        inst_idx: usize,
        tick: u64,
        phonemes: &[name_gen::PhonemeSet],
    ) -> Option<Event> {
        let inst = &self.institutions[inst_idx];
        let inst_name = inst.name.clone();
        let people_id = inst.people_id;
        let member_count = inst.member_ids.len();

        let roll: f32 = self.rng.gen();

        // Schism — only if institution has 4+ members
        if roll < 0.15 && member_count >= 4 {
            // Split: create a new institution with half the members
            let split_count = member_count / 2;
            let split_members: Vec<u64> = inst.member_ids[..split_count].to_vec();
            let remaining: Vec<u64> = inst.member_ids[split_count..].to_vec();

            let new_kind = inst.kind.clone();
            let new_name = name_gen::generate_institution_name_with_weirdness(&new_kind, phonemes, people_id, self.world.params.weirdness_coefficient, &mut self.rng);
            let new_charter = name_gen::generate_charter(&new_kind, &mut self.rng);
            let new_doctrines = name_gen::generate_doctrines(&new_kind, &mut self.rng);

            let new_id = self.next_institution_id;
            self.next_institution_id += 1;

            let mut relationships = std::collections::HashMap::new();
            relationships.insert(inst.id, institution::InstitutionRelationship::Rival);

            let new_inst = Institution {
                id: new_id,
                name: new_name.clone(),
                kind: new_kind,
                charter: new_charter,
                actual_function: name_gen::generate_actual_function(&inst.kind, &mut self.rng),
                power: inst.power / 2,
                doctrine: new_doctrines,
                member_ids: split_members.clone(),
                territory: inst.territory.clone(),
                founded_tick: tick,
                relationships,
                chronicle: vec![format!("Split from {} over doctrinal differences", inst_name)],
                people_id,
                alive: true,
            };

            // Update old institution
            let inst = &mut self.institutions[inst_idx];
            inst.member_ids = remaining;
            inst.power = inst.power / 2 + 1;
            inst.relationships.insert(new_id, institution::InstitutionRelationship::Rival);
            inst.chronicle.push(format!("Suffered a schism; {} departed to form {}", split_count, new_name));

            // Update split members' affiliations
            for &aid in &split_members {
                if let Some(agent) = self.agents.iter_mut().find(|a| a.id == aid) {
                    agent.institution_ids.retain(|&id| id != inst.id);
                    agent.institution_ids.push(new_id);
                }
            }

            self.institutions.push(new_inst);

            let description = prose_gen::generate_institutional_description(
                &EventType::SchismOccurred,
                None,
                Some(&inst_name),
                None,
                &mut self.rng,
                self.world.params.narrative_register,
                self.world.params.weirdness_coefficient,
            );
            return Some(Event {
                tick,
                event_type: EventType::SchismOccurred,
                subject_id: None,
                location: None,
                description: format!("{} The dissenting faction reconstituted as {}.", description, new_name),
            });
        }

        // Doctrine shift
        if roll < 0.45 && !inst.doctrine.is_empty() {
            let old_idx = self.rng.gen_range(0..inst.doctrine.len());
            let old_doctrine = self.institutions[inst_idx].doctrine[old_idx].clone();
            let new_doctrines = name_gen::generate_doctrines(&inst.kind, &mut self.rng);
            if let Some(new_d) = new_doctrines.into_iter().find(|d| d != &old_doctrine) {
                self.institutions[inst_idx].doctrine[old_idx] = new_d.clone();
                self.institutions[inst_idx].chronicle.push(
                    format!("Officially revised position: '{}' replaced by '{}'", old_doctrine, new_d)
                );

                let description = prose_gen::generate_institutional_description(
                    &EventType::DoctrineShifted,
                    None,
                    Some(&inst_name),
                    None,
                    &mut self.rng,
                    self.world.params.narrative_register,
                    self.world.params.weirdness_coefficient,
                );
                return Some(Event {
                    tick,
                    event_type: EventType::DoctrineShifted,
                    subject_id: None,
                    location: None,
                    description,
                });
            }
        }

        // Power shift — institution gains or loses influence
        if roll < 0.7 {
            let change: i32 = self.rng.gen_range(-3..=5);
            let inst = &mut self.institutions[inst_idx];
            inst.power = (inst.power as i32 + change).max(1) as u32;
            // Not interesting enough for a log event on its own
            return None;
        }

        None
    }

    /// Generate a relationship event between two institutions.
    fn generate_relationship_event(&mut self, a_id: u64, b_id: u64, tick: u64) -> Option<Event> {
        let a_idx = self.institutions.iter().position(|i| i.id == a_id)?;
        let b_idx = self.institutions.iter().position(|i| i.id == b_id)?;

        let a_name = self.institutions[a_idx].name.clone();
        let b_name = self.institutions[b_idx].name.clone();

        let roll: f32 = self.rng.gen();
        let (event_type, relationship, description_extra) = if roll < 0.3 {
            (
                EventType::AllianceFormed,
                institution::InstitutionRelationship::Allied,
                format!("{} and {} have entered into a formal alliance.", a_name, b_name),
            )
        } else if roll < 0.55 {
            (
                EventType::RivalryDeclared,
                institution::InstitutionRelationship::Rival,
                format!("{} has declared {} a rival organization.", a_name, b_name),
            )
        } else if roll < 0.75 {
            let disputes = [
                "a boundary matter", "a question of precedence", "an unpaid obligation",
                "a doctrinal disagreement", "a personnel dispute", "a jurisdictional claim",
            ];
            let reason = disputes[self.rng.gen_range(0..disputes.len())];
            (
                EventType::AllianceStrained,
                institution::InstitutionRelationship::Disputed(reason.to_string()),
                format!("Relations between {} and {} have become strained over {}.", a_name, b_name, reason),
            )
        } else {
            return None; // No change
        };

        let mirror = match &relationship {
            institution::InstitutionRelationship::Allied => institution::InstitutionRelationship::Allied,
            institution::InstitutionRelationship::Rival => institution::InstitutionRelationship::Rival,
            institution::InstitutionRelationship::Disputed(r) => institution::InstitutionRelationship::Disputed(r.clone()),
            institution::InstitutionRelationship::Neutral => institution::InstitutionRelationship::Neutral,
        };

        self.institutions[a_idx].relationships.insert(b_id, relationship);
        self.institutions[a_idx].chronicle.push(description_extra.clone());
        self.institutions[b_idx].relationships.insert(a_id, mirror);
        self.institutions[b_idx].chronicle.push(description_extra.clone());

        let description = prose_gen::generate_institutional_description(
            &event_type,
            None,
            Some(&a_name),
            Some(&b_name),
            &mut self.rng,
            self.world.params.narrative_register,
            self.world.params.weirdness_coefficient,
        );

        Some(Event {
            tick,
            event_type,
            subject_id: None,
            location: None,
            description,
        })
    }

    /// Process a member departure or expulsion.
    fn process_member_departure(&mut self, tick: u64) -> Option<Event> {
        // Find an agent who might leave their institution
        let candidates: Vec<usize> = self.agents.iter()
            .enumerate()
            .filter(|(_, a)| a.alive && !a.institution_ids.is_empty())
            .map(|(i, _)| i)
            .collect();

        if candidates.is_empty() { return None; }

        let ai = candidates[self.rng.gen_range(0..candidates.len())];
        let agent = &self.agents[ai];

        // Low loyalty + random chance = departure
        if agent.disposition.institutional_loyalty > 0.3 || !self.rng.gen_bool(0.15) {
            return None;
        }

        let inst_id = agent.institution_ids[0];
        let agent_name = agent.display_name();
        let agent_id = agent.id;

        let inst_name = self.institutions.iter()
            .find(|i| i.id == inst_id)
            .map(|i| i.name.clone())
            .unwrap_or_else(|| "an unnamed body".to_string());

        // Determine if it's a departure or expulsion
        let expelled = self.rng.gen_bool(0.3);
        let event_type = if expelled { EventType::MemberExpelled } else { EventType::MemberDeparted };

        // Remove from institution
        if let Some(inst) = self.institutions.iter_mut().find(|i| i.id == inst_id) {
            inst.member_ids.retain(|&id| id != agent_id);
            let verb = if expelled { "expelled" } else { "departed" };
            inst.chronicle.push(format!("{} {} from the organization", agent_name, verb));
        }

        self.agents[ai].institution_ids.retain(|&id| id != inst_id);
        self.agents[ai].current_goal = agent::Goal::Wander;

        let description = prose_gen::generate_institutional_description(
            &event_type,
            Some(&agent_name),
            Some(&inst_name),
            None,
            &mut self.rng,
            self.world.params.narrative_register,
            self.world.params.weirdness_coefficient,
        );

        Some(Event {
            tick,
            event_type,
            subject_id: Some(agent_id),
            location: Some((self.agents[ai].x, self.agents[ai].y)),
            description,
        })
    }

    /// Process relationship formation, evolution, and behavioral effects.
    /// Runs every 20 ticks to reduce overhead.
    fn process_relationship_tick(&mut self, tick: u64) -> Vec<Event> {
        let mut events = Vec::new();
        if tick % 20 != 0 {
            return events;
        }

        let register = self.world.params.narrative_register;
        let weirdness = self.world.params.weirdness_coefficient;

        // Build a map of (grid position) -> list of alive agent indices for proximity checks.
        let mut agents_at: std::collections::HashMap<(u32, u32), Vec<usize>> = std::collections::HashMap::new();
        for (i, a) in self.agents.iter().enumerate() {
            if a.alive {
                agents_at.entry((a.x, a.y)).or_default().push(i);
            }
        }

        // Collect proximity pairs (agents at the same tile).
        let mut proximity_pairs: Vec<(usize, usize)> = Vec::new();
        for indices in agents_at.values() {
            if indices.len() < 2 { continue; }
            // Sample a limited number of pairs to avoid quadratic blowup.
            let limit = indices.len().min(6);
            for i in 0..limit {
                for j in (i + 1)..limit {
                    proximity_pairs.push((indices[i], indices[j]));
                }
            }
        }

        // --- Formation ---
        // Friendship: co-located agents with compatible dispositions
        for &(ai, bi) in &proximity_pairs {
            let a = &self.agents[ai];
            let b = &self.agents[bi];

            // Count recent conversation tones between this pair (last 10)
            let recent_convos: Vec<agent::ConversationTone> = a.conversations.iter()
                .rev().take(10)
                .filter(|c| c.other_id == b.id)
                .map(|c| c.tone)
                .collect();
            let warm_count = recent_convos.iter().filter(|t| **t == agent::ConversationTone::Warm).count();
            let significant_count = recent_convos.iter().filter(|t| **t == agent::ConversationTone::Significant).count();

            // Base ~1.5% chance, boosted by warm conversations (+50% if 4+ warm)
            let base_skip = if warm_count >= 4 { 0.9775 } else { 0.985 }; // 2.25% vs 1.5%
            if self.rng.gen_bool(base_skip) { continue; }

            // Skip if they already have a relationship
            if a.relationships.iter().any(|r| r.other_id == b.id) { continue; }
            // Disposition compatibility: similar loyalty and ambition
            let compat = 1.0 - ((a.disposition.institutional_loyalty - b.disposition.institutional_loyalty).abs()
                + (a.disposition.ambition - b.disposition.ambition).abs()) / 2.0;
            if compat < 0.4 { continue; }

            // Significant conversations count as notable relationship events
            let _ = significant_count; // used below in notability check

            let a_name = a.display_name();
            let b_name = b.display_name();
            let a_id = a.id;
            let b_id = b.id;
            let loc = (a.x, a.y);

            // Determine relationship type
            let kind = if self.is_mentor_candidate(ai, bi) {
                // Mentor/Protégé
                (agent::RelationshipKind::Mentor, agent::RelationshipKind::Protege)
            } else if compat > 0.7 && self.rng.gen_bool(0.15) {
                // Romantic partner (rare, requires high compatibility)
                (agent::RelationshipKind::Partner, agent::RelationshipKind::Partner)
            } else {
                (agent::RelationshipKind::Friend, agent::RelationshipKind::Friend)
            };

            self.agents[ai].relationships.push(agent::Relationship {
                other_id: b_id, kind: kind.0, intensity: 1, formed_tick: tick,
            });
            self.agents[bi].relationships.push(agent::Relationship {
                other_id: a_id, kind: kind.1, intensity: 1, formed_tick: tick,
            });

            // Only log notable relationship events (agents with institutional power or many epithets)
            // Significant conversations also trigger notability
            let notable = self.agents[ai].institution_ids.len() >= 2
                || self.agents[bi].institution_ids.len() >= 2
                || self.agents[ai].epithets.len() >= 2
                || self.agents[bi].epithets.len() >= 2
                || significant_count >= 1;
            if notable {
                let inst_a = self.agents[ai].institution_ids.first()
                    .and_then(|id| self.institutions.iter().find(|i| i.id == *id))
                    .map(|i| i.name.clone());
                let inst_b = self.agents[bi].institution_ids.first()
                    .and_then(|id| self.institutions.iter().find(|i| i.id == *id))
                    .map(|i| i.name.clone());
                let description = prose_gen::generate_relationship_event(
                    &a_name, &b_name, kind.0.label(), true,
                    &mut self.rng, register, weirdness,
                    inst_a.as_deref(), inst_b.as_deref(),
                );
                events.push(Event {
                    tick,
                    event_type: EventType::RelationshipFormed,
                    subject_id: Some(a_id),
                    location: Some(loc),
                    description,
                });
            }
        }

        // Rivalry: agents in competing institutions or conflicting dispositions
        for &(ai, bi) in &proximity_pairs {
            let a = &self.agents[ai];
            let b = &self.agents[bi];

            // Count tense conversations between this pair (last 10)
            let tense_count = a.conversations.iter()
                .rev().take(10)
                .filter(|c| c.other_id == b.id && c.tone == agent::ConversationTone::Tense)
                .count();

            // Base ~1% chance, boosted by tense conversations (+50% if 4+ tense)
            let base_skip = if tense_count >= 4 { 0.985 } else { 0.99 }; // 1.5% vs 1%
            if self.rng.gen_bool(base_skip) { continue; }

            if a.relationships.iter().any(|r| r.other_id == b.id) { continue; }

            // Conflicting institutions or very different dispositions
            let inst_conflict = !a.institution_ids.is_empty() && !b.institution_ids.is_empty()
                && a.institution_ids.iter().all(|id| !b.institution_ids.contains(id));
            let disp_conflict = (a.disposition.ambition - b.disposition.ambition).abs() > 0.5
                || (a.disposition.paranoia - b.disposition.paranoia).abs() > 0.5;

            if !inst_conflict && !disp_conflict { continue; }

            let a_name = a.display_name();
            let b_name = b.display_name();
            let a_id = a.id;
            let b_id = b.id;
            let loc = (a.x, a.y);

            self.agents[ai].relationships.push(agent::Relationship {
                other_id: b_id, kind: agent::RelationshipKind::Rival, intensity: 1, formed_tick: tick,
            });
            self.agents[bi].relationships.push(agent::Relationship {
                other_id: a_id, kind: agent::RelationshipKind::Rival, intensity: 1, formed_tick: tick,
            });

            let notable = self.agents[ai].institution_ids.len() >= 2
                || self.agents[bi].institution_ids.len() >= 2
                || self.agents[ai].epithets.len() >= 2
                || self.agents[bi].epithets.len() >= 2;
            if notable {
                let inst_a = self.agents[ai].institution_ids.first()
                    .and_then(|id| self.institutions.iter().find(|i| i.id == *id))
                    .map(|i| i.name.clone());
                let inst_b = self.agents[bi].institution_ids.first()
                    .and_then(|id| self.institutions.iter().find(|i| i.id == *id))
                    .map(|i| i.name.clone());
                let description = prose_gen::generate_relationship_event(
                    &a_name, &b_name, "Rival", true,
                    &mut self.rng, register, weirdness,
                    inst_a.as_deref(), inst_b.as_deref(),
                );
                events.push(Event {
                    tick,
                    event_type: EventType::RelationshipFormed,
                    subject_id: Some(a_id),
                    location: Some(loc),
                    description,
                });
            }
        }

        // --- Evolution: intensity changes and type transitions ---
        // Run every 100 ticks for performance
        if tick % 100 == 0 {
            // Collect changes to apply
            let mut changes: Vec<(usize, usize, Option<agent::RelationshipKind>, i8)> = Vec::new(); // (agent_idx, rel_idx, new_kind, intensity_delta)

            for (ai, agent) in self.agents.iter().enumerate() {
                if !agent.alive { continue; }
                for (ri, rel) in agent.relationships.iter().enumerate() {
                    // Only process if the other agent is alive
                    let other_alive = self.agents.iter().any(|a| a.id == rel.other_id && a.alive);
                    if !other_alive {
                        continue;
                    }

                    let age = tick.saturating_sub(rel.formed_tick);

                    match rel.kind {
                        agent::RelationshipKind::Friend => {
                            // Friends can deepen over time
                            if age > 200 && rel.intensity < 3 && self.rng.gen_bool(0.05) {
                                changes.push((ai, ri, None, 1));
                            }
                            // Friends can cool or become estranged if not co-located long enough
                            if age > 500 && rel.intensity == 1 && self.rng.gen_bool(0.03) {
                                changes.push((ai, ri, Some(agent::RelationshipKind::Estranged), 0));
                            }
                        }
                        agent::RelationshipKind::Partner => {
                            // Partners can deepen
                            if age > 300 && rel.intensity < 3 && self.rng.gen_bool(0.04) {
                                changes.push((ai, ri, None, 1));
                            }
                            // Partners can dissolve into estrangement
                            if age > 600 && self.rng.gen_bool(0.01) {
                                changes.push((ai, ri, Some(agent::RelationshipKind::Estranged), 0));
                            }
                        }
                        agent::RelationshipKind::Rival => {
                            // Rivals can intensify
                            if age > 200 && rel.intensity < 3 && self.rng.gen_bool(0.04) {
                                changes.push((ai, ri, None, 1));
                            }
                            // Rivals can reconcile into friendship (rare)
                            if age > 500 && self.rng.gen_bool(0.015) {
                                changes.push((ai, ri, Some(agent::RelationshipKind::Friend), 0));
                            }
                        }
                        agent::RelationshipKind::Mentor | agent::RelationshipKind::Protege => {
                            // Can deepen
                            if age > 300 && rel.intensity < 3 && self.rng.gen_bool(0.03) {
                                changes.push((ai, ri, None, 1));
                            }
                        }
                        agent::RelationshipKind::Estranged => {
                            // Can slowly cool in intensity
                            if age > 400 && rel.intensity > 1 && self.rng.gen_bool(0.02) {
                                changes.push((ai, ri, None, -1));
                            }
                        }
                    }
                }
            }

            // Apply changes
            for (ai, ri, new_kind, intensity_delta) in changes {
                if ri >= self.agents[ai].relationships.len() { continue; }
                let other_id = self.agents[ai].relationships[ri].other_id;
                let agent_id = self.agents[ai].id;

                if let Some(kind) = new_kind {
                    self.agents[ai].relationships[ri].kind = kind;
                    self.agents[ai].relationships[ri].intensity = 1;

                    // Update the reciprocal relationship
                    let reciprocal_kind = match kind {
                        agent::RelationshipKind::Friend => agent::RelationshipKind::Friend,
                        agent::RelationshipKind::Estranged => agent::RelationshipKind::Estranged,
                        other => other,
                    };
                    if let Some(oi) = self.agents.iter().position(|a| a.id == other_id) {
                        if oi != ai {
                            if let Some(other_rel) = self.agents[oi].relationships.iter_mut().find(|r| r.other_id == agent_id) {
                                other_rel.kind = reciprocal_kind;
                                other_rel.intensity = 1;
                            }
                        }
                    }

                    // Log notable changes
                    let a_notable = self.agents[ai].institution_ids.len() >= 2
                        || self.agents[ai].epithets.len() >= 2;
                    if a_notable {
                        let a_name = self.agents[ai].display_name();
                        let b_name = self.agents.iter().find(|a| a.id == other_id)
                            .map(|a| a.display_name()).unwrap_or_else(|| "unknown".to_string());
                        let inst_a = self.agents[ai].institution_ids.first()
                            .and_then(|id| self.institutions.iter().find(|i| i.id == *id))
                            .map(|i| i.name.clone());
                        let oi_opt = self.agents.iter().position(|a| a.id == other_id);
                        let inst_b = oi_opt.and_then(|oi| self.agents[oi].institution_ids.first())
                            .and_then(|id| self.institutions.iter().find(|i| i.id == *id))
                            .map(|i| i.name.clone());
                        let description = prose_gen::generate_relationship_event(
                            &a_name, &b_name, kind.label(), false,
                            &mut self.rng, register, weirdness,
                            inst_a.as_deref(), inst_b.as_deref(),
                        );
                        events.push(Event {
                            tick,
                            event_type: EventType::RelationshipChanged,
                            subject_id: Some(agent_id),
                            location: Some((self.agents[ai].x, self.agents[ai].y)),
                            description,
                        });
                    }
                } else {
                    let new_intensity = (self.agents[ai].relationships[ri].intensity as i8 + intensity_delta).clamp(1, 3) as u8;
                    self.agents[ai].relationships[ri].intensity = new_intensity;
                    // Update reciprocal intensity
                    if let Some(oi) = self.agents.iter().position(|a| a.id == other_id) {
                        if oi != ai {
                            if let Some(other_rel) = self.agents[oi].relationships.iter_mut().find(|r| r.other_id == agent_id) {
                                other_rel.intensity = new_intensity;
                            }
                        }
                    }
                }
            }
        }

        // --- Behavioral effects ---

        // Estranged agents avoid each other: if at the same settlement, chance to emigrate
        if tick % 50 == 0 {
            let mut flee_agents: Vec<usize> = Vec::new();
            for (ai, agent) in self.agents.iter().enumerate() {
                if !agent.alive { continue; }
                for rel in &agent.relationships {
                    if rel.kind != agent::RelationshipKind::Estranged { continue; }
                    if let Some(other) = self.agents.iter().find(|a| a.id == rel.other_id && a.alive) {
                        if agent.x == other.x && agent.y == other.y && self.rng.gen_bool(0.1) {
                            flee_agents.push(ai);
                            break;
                        }
                    }
                }
            }
            let settlement_positions: Vec<(u32, u32)> = self.world.settlements
                .iter().map(|s| (s.x as u32, s.y as u32)).collect();
            for ai in flee_agents {
                if !settlement_positions.is_empty() {
                    let idx = self.rng.gen_range(0..settlement_positions.len());
                    self.agents[ai].current_goal = agent::Goal::SeekSettlement(idx);
                }
            }
        }

        // Friends accompany each other: if a friend is heading to a site, chance to join
        if tick % 30 == 0 {
            let mut accompany: Vec<(usize, usize)> = Vec::new(); // (follower_idx, site_idx)
            for (ai, agent) in self.agents.iter().enumerate() {
                if !agent.alive { continue; }
                if !matches!(agent.current_goal, agent::Goal::Wander | agent::Goal::Rest(_)) { continue; }
                for rel in &agent.relationships {
                    if rel.kind != agent::RelationshipKind::Friend { continue; }
                    if let Some(friend) = self.agents.iter().find(|a| a.id == rel.other_id && a.alive) {
                        if let agent::Goal::SeekSite(site_idx) = friend.current_goal {
                            // Only if nearby (within 3 tiles)
                            let dx = (agent.x as i32 - friend.x as i32).abs();
                            let dy = (agent.y as i32 - friend.y as i32).abs();
                            if dx <= 3 && dy <= 3 && self.rng.gen_bool(0.15 * rel.intensity as f64 / 3.0) {
                                accompany.push((ai, site_idx));
                                break;
                            }
                        }
                    }
                }
            }
            for (ai, site_idx) in accompany {
                self.agents[ai].current_goal = agent::Goal::SeekSite(site_idx);
            }
        }

        // Partners: higher adventuring chance if partner is in danger (exploring a site)
        if tick % 50 == 0 {
            for ai in 0..self.agents.len() {
                let agent = &self.agents[ai];
                if !agent.alive || agent.is_adventurer { continue; }
                let has_endangered_partner = agent.relationships.iter().any(|rel| {
                    rel.kind == agent::RelationshipKind::Partner
                        && self.agents.iter().any(|a| {
                            a.id == rel.other_id && a.alive
                                && matches!(a.current_goal, agent::Goal::ExploreSite(_, _))
                        })
                });
                if has_endangered_partner && agent.disposition.risk_tolerance > 0.3
                    && self.rng.gen_bool(0.08)
                {
                    self.agents[ai].is_adventurer = true;
                }
            }
        }

        // Mentors smooth path for protégés: boost join chance
        // (Handled inline in process_institutional_tick by checking relationships)

        events
    }

    /// Check if agent at index `ai` could be a mentor to agent at `bi`.
    fn is_mentor_candidate(&self, ai: usize, bi: usize) -> bool {
        let a = &self.agents[ai];
        let b = &self.agents[bi];
        // Mentor must be significantly older and share an institution
        let age_gap = a.age.saturating_sub(b.age);
        if age_gap < 3650 { return false; } // at least ~10 years older
        // Must share at least one institution
        a.institution_ids.iter().any(|id| b.institution_ids.contains(id))
    }

    /// Count total active relationships in the world.
    pub fn relationship_count(&self) -> usize {
        // Each relationship is stored on both sides, so divide by 2
        self.agents.iter()
            .filter(|a| a.alive)
            .map(|a| a.relationships.iter()
                .filter(|r| self.agents.iter().any(|o| o.id == r.other_id && o.alive))
                .count())
            .sum::<usize>() / 2
    }

    /// Process conversation generation between co-located agents.
    /// Runs every 30 ticks.
    fn process_conversation_tick(&mut self, tick: u64) -> Vec<Event> {
        let mut events = Vec::new();
        if tick % 30 != 0 {
            return events;
        }

        // Build co-location map
        let mut agents_at: std::collections::HashMap<(u32, u32), Vec<usize>> = std::collections::HashMap::new();
        for (i, a) in self.agents.iter().enumerate() {
            if a.alive {
                agents_at.entry((a.x, a.y)).or_default().push(i);
            }
        }

        // Collect pairs to converse
        let mut conversation_pairs: Vec<(usize, usize)> = Vec::new();
        for indices in agents_at.values() {
            if indices.len() < 2 { continue; }
            let limit = indices.len().min(6);
            for i in 0..limit {
                for j in (i + 1)..limit {
                    // ~3% chance per pair
                    if self.rng.gen_bool(0.03) {
                        conversation_pairs.push((indices[i], indices[j]));
                    }
                }
            }
        }

        for (ai, bi) in conversation_pairs {
            // Extract data before mutable borrow
            let rel_kind = self.agents[ai].relationships.iter()
                .find(|r| r.other_id == self.agents[bi].id)
                .map(|r| r.kind);
            let a_name = self.agents[ai].display_name();
            let b_name = self.agents[bi].display_name();
            let a_id = self.agents[ai].id;
            let b_id = self.agents[bi].id;
            let loc = (self.agents[ai].x, self.agents[ai].y);

            // Determine tone weighted by relationship
            let tone = self.pick_conversation_tone(rel_kind);

            let (line_a, line_b) = prose_gen::generate_conversation(
                &a_name, &b_name, tone, &mut self.rng,
            );

            let conv_a = agent::Conversation {
                other_id: b_id, tick, line_a: line_a.clone(), line_b: line_b.clone(), tone,
            };
            let conv_b = agent::Conversation {
                other_id: a_id, tick, line_a: line_a.clone(), line_b: line_b.clone(), tone,
            };

            // Push and cap at 20
            self.agents[ai].conversations.push(conv_a);
            if self.agents[ai].conversations.len() > 20 {
                self.agents[ai].conversations.remove(0);
            }
            self.agents[bi].conversations.push(conv_b);
            if self.agents[bi].conversations.len() > 20 {
                self.agents[bi].conversations.remove(0);
            }

            // Only log Significant conversations involving notable agents
            if tone == agent::ConversationTone::Significant {
                let a_notable = self.agents[ai].institution_ids.len() >= 2
                    || self.agents[ai].epithets.len() >= 2;
                let b_notable = self.agents[bi].institution_ids.len() >= 2
                    || self.agents[bi].epithets.len() >= 2;
                if a_notable || b_notable {
                    events.push(Event {
                        tick,
                        event_type: EventType::ConversationOccurred,
                        subject_id: Some(a_id),
                        location: Some(loc),
                        description: format!("{} {}", line_a, line_b),
                    });
                }
            }
        }

        events
    }

    /// Pick a conversation tone weighted by relationship kind.
    fn pick_conversation_tone(&mut self, rel_kind: Option<agent::RelationshipKind>) -> agent::ConversationTone {
        use agent::ConversationTone::*;
        use agent::RelationshipKind;

        // Weights: [Mundane, Warm, Tense, Cryptic, Significant]
        let weights: [f32; 5] = match rel_kind {
            Some(RelationshipKind::Friend) | Some(RelationshipKind::Partner) => [0.15, 0.45, 0.05, 0.15, 0.20],
            Some(RelationshipKind::Mentor) | Some(RelationshipKind::Protege) => [0.20, 0.30, 0.05, 0.20, 0.25],
            Some(RelationshipKind::Rival) => [0.10, 0.05, 0.50, 0.20, 0.15],
            Some(RelationshipKind::Estranged) => [0.15, 0.05, 0.40, 0.30, 0.10],
            None => [0.40, 0.10, 0.10, 0.30, 0.10],
        };

        let roll: f32 = self.rng.gen();
        let mut cumulative = 0.0;
        let tones = [Mundane, Warm, Tense, Cryptic, Significant];
        for (i, &w) in weights.iter().enumerate() {
            cumulative += w;
            if roll < cumulative {
                return tones[i];
            }
        }
        Mundane
    }

    /// Process adventurer artifact-related actions for one tick.
    fn process_adventurer_tick(&mut self, tick: u64) -> Vec<Event> {
        let mut events = Vec::new();

        let settlement_positions: Vec<(u32, u32)> = self.world.settlements
            .iter().map(|s| (s.x as u32, s.y as u32)).collect();
        let site_positions: Vec<(u32, u32)> = self.sites
            .iter().map(|s| (s.grid_x, s.grid_y)).collect();

        let agent_count = self.agents.len();
        for ai in 0..agent_count {
            if !self.agents[ai].alive { continue; }

            let goal = self.agents[ai].current_goal.clone();
            match goal {
                agent::Goal::AcquireArtifact(artifact_id, site_idx) => {
                    if site_idx >= site_positions.len() {
                        self.agents[ai].current_goal = agent::Goal::Wander;
                        continue;
                    }
                    let (sx, sy) = site_positions[site_idx];
                    if self.agents[ai].x != sx || self.agents[ai].y != sy {
                        continue; // still traveling
                    }

                    // At the site — chance of death
                    if self.rng.gen_bool(0.03) {
                        let agent_name = self.agents[ai].display_name();
                        let site_name = self.sites.get(site_idx)
                            .map(|s| s.name.clone()).unwrap_or_else(|| "an unnamed site".to_string());
                        self.agents[ai].alive = false;
                        let description = prose_gen::generate_adventurer_death(
                            &agent_name, &site_name, &mut self.rng,
                            self.world.params.narrative_register,
                            self.world.params.weirdness_coefficient,
                        );
                        events.push(Event {
                            tick,
                            event_type: EventType::AdventurerDiedInSite,
                            subject_id: Some(self.agents[ai].id),
                            location: Some((sx, sy)),
                            description,
                        });
                        // Drop any held artifacts back into the site
                        let held = self.agents[ai].held_artifacts.clone();
                        for art_id in held {
                            if let Some(art) = self.artifacts.iter_mut().find(|a| a.id == art_id) {
                                art.current_location = artifact::ArtifactLocation::InSite(site_idx);
                                art.history.push(format!(
                                    "Returned to {} following the demise of its bearer.", site_name
                                ));
                            }
                            if site_idx < self.sites.len() {
                                self.sites[site_idx].artifacts.push(art_id);
                            }
                        }
                        self.agents[ai].held_artifacts.clear();
                        continue;
                    }

                    // Try to acquire
                    let still_here = self.artifacts.iter().any(|a| {
                        a.id == artifact_id
                            && matches!(a.current_location, artifact::ArtifactLocation::InSite(si) if si == site_idx)
                    });
                    if !still_here {
                        self.agents[ai].current_goal = agent::Goal::Wander;
                        continue;
                    }

                    // Acquire the artifact
                    let agent_name = self.agents[ai].display_name();
                    let agent_id = self.agents[ai].id;
                    let site_name = self.sites.get(site_idx)
                        .map(|s| s.name.clone()).unwrap_or_else(|| "an unnamed site".to_string());

                    if let Some(art) = self.artifacts.iter_mut().find(|a| a.id == artifact_id) {
                        art.current_location = artifact::ArtifactLocation::HeldByAgent(agent_id);
                        art.history.push(format!(
                            "Acquired by {} from {}.", agent_name, site_name
                        ));
                        let art_name = art.name.clone();

                        // Remove from site's artifact list
                        if site_idx < self.sites.len() {
                            self.sites[site_idx].artifacts.retain(|&id| id != artifact_id);
                        }

                        self.agents[ai].held_artifacts.push(artifact_id);

                        let description = prose_gen::generate_artifact_event(
                            &EventType::ArtifactAcquired,
                            &agent_name,
                            &art_name,
                            &site_name,
                            &mut self.rng,
                            self.world.params.narrative_register,
                            self.world.params.weirdness_coefficient,
                        );
                        events.push(Event {
                            tick,
                            event_type: EventType::ArtifactAcquired,
                            subject_id: Some(agent_id),
                            location: Some((sx, sy)),
                            description,
                        });

                        // Now set goal to return to nearest settlement
                        let nearest = settlement_positions.iter().enumerate()
                            .min_by_key(|(_, (px, py))| {
                                let dx = (*px as i32 - sx as i32).unsigned_abs();
                                let dy = (*py as i32 - sy as i32).unsigned_abs();
                                dx + dy
                            })
                            .map(|(idx, _)| idx)
                            .unwrap_or(0);
                        self.agents[ai].current_goal = agent::Goal::ReturnArtifact(artifact_id, nearest);
                    }
                }
                agent::Goal::ReturnArtifact(artifact_id, settlement_idx) => {
                    if settlement_idx >= settlement_positions.len() {
                        self.agents[ai].current_goal = agent::Goal::Wander;
                        continue;
                    }
                    let (sx, sy) = settlement_positions[settlement_idx];
                    if self.agents[ai].x != sx || self.agents[ai].y != sy {
                        continue; // still traveling
                    }

                    // Deliver the artifact
                    let agent_name = self.agents[ai].display_name();
                    let agent_id = self.agents[ai].id;
                    let settlement_name = self.world.settlements.get(settlement_idx)
                        .map(|s| s.name.clone()).unwrap_or_else(|| "an unnamed settlement".to_string());

                    if let Some(art) = self.artifacts.iter_mut().find(|a| a.id == artifact_id) {
                        art.current_location = artifact::ArtifactLocation::InSettlement(settlement_idx);
                        art.history.push(format!(
                            "Delivered to {} by {}.", settlement_name, agent_name
                        ));
                        let art_name = art.name.clone();

                        self.agents[ai].held_artifacts.retain(|&id| id != artifact_id);

                        let description = prose_gen::generate_artifact_event(
                            &EventType::ArtifactDelivered,
                            &agent_name,
                            &art_name,
                            &settlement_name,
                            &mut self.rng,
                            self.world.params.narrative_register,
                            self.world.params.weirdness_coefficient,
                        );
                        events.push(Event {
                            tick,
                            event_type: EventType::ArtifactDelivered,
                            subject_id: Some(agent_id),
                            location: Some((sx, sy)),
                            description,
                        });
                    }

                    self.agents[ai].current_goal = agent::Goal::Rest(self.rng.gen_range(10..=30));
                }
                _ => {
                    // Adventurers idle: seek artifacts
                    if self.agents[ai].is_adventurer
                        && matches!(goal, agent::Goal::Wander | agent::Goal::Rest(_))
                        && self.rng.gen_bool(0.02)
                    {
                        let site_artifacts: Vec<(u64, usize)> = self.artifacts.iter()
                            .filter_map(|a| {
                                if let artifact::ArtifactLocation::InSite(si) = &a.current_location {
                                    Some((a.id, *si))
                                } else {
                                    None
                                }
                            })
                            .collect();
                        if !site_artifacts.is_empty() {
                            let (aid, si) = site_artifacts[self.rng.gen_range(0..site_artifacts.len())];
                            self.agents[ai].current_goal = agent::Goal::AcquireArtifact(aid, si);
                        }
                    }
                }
            }
        }

        // --- Inhabitant interactions ---
        // Check agents currently at sites for interactions with inhabitants
        if tick % 5 == 0 {
            for si in 0..self.sites.len() {
                let site_pop: Vec<u64> = self.sites[si].population.clone();
                if site_pop.is_empty() || self.sites[si].inhabitants.is_empty() {
                    continue;
                }
                // ~15% chance per agent per 5-tick check
                for &agent_id in &site_pop {
                    if !self.rng.gen_bool(0.15) { continue; }
                    let agent_idx = match self.agents.iter().position(|a| a.id == agent_id && a.alive) {
                        Some(idx) => idx,
                        None => continue,
                    };
                    let inhab_idx = self.rng.gen_range(0..self.sites[si].inhabitants.len());
                    let inhab_name = self.sites[si].inhabitants[inhab_idx].name.clone();
                    let inhab_desc = self.sites[si].inhabitants[inhab_idx].description.clone();
                    let inhab_floor = self.sites[si].inhabitants[inhab_idx].floor;
                    let site_name = self.sites[si].name.clone();
                    let agent_name = self.agents[agent_idx].display_name();

                    // Get room purpose for the inhabitant's location
                    let room_purpose = self.sites[si].floors.get(inhab_floor).and_then(|f| {
                        let ix = self.sites[si].inhabitants[inhab_idx].x;
                        let iy = self.sites[si].inhabitants[inhab_idx].y;
                        f.rooms.iter().find(|r| ix >= r.x && ix < r.x + r.w && iy >= r.y && iy < r.y + r.h)
                            .map(|r| r.purpose.label())
                    });

                    let description = prose_gen::generate_inhabitant_interaction(
                        &agent_name,
                        &inhab_name,
                        &inhab_desc,
                        &site_name,
                        room_purpose,
                        &mut self.rng,
                        self.world.params.narrative_register,
                        self.world.params.weirdness_coefficient,
                    );
                    let (gx, gy) = (self.sites[si].grid_x, self.sites[si].grid_y);
                    events.push(Event {
                        tick,
                        event_type: EventType::InhabitantInteraction,
                        subject_id: Some(agent_id),
                        location: Some((gx, gy)),
                        description,
                    });
                    // Add to site history
                    let history_entry = format!("Tick {}: {} encountered {} within the site.", tick, agent_name, inhab_name);
                    self.sites[si].history.push(history_entry);
                    break; // one interaction per site per check
                }
            }
        }

        events
    }

    /// Get indices of all living institutions.
    pub fn living_institution_indices(&self) -> Vec<usize> {
        self.institutions.iter()
            .enumerate()
            .filter(|(_, i)| i.alive)
            .map(|(idx, _)| idx)
            .collect()
    }

    /// Get the display name of the currently followed entity.
    pub fn follow_label(&self) -> Option<String> {
        match &self.follow_target {
            Some(FollowTarget::Agent(id)) => {
                self.agents.iter().find(|a| a.id == *id).map(|a| a.display_name())
            }
            Some(FollowTarget::Institution(id)) => {
                self.institutions.iter().find(|i| i.id == *id).map(|i| i.name.clone())
            }
            None => None,
        }
    }

    /// Get events relevant to the followed entity.
    pub fn follow_events(&self) -> Vec<&Event> {
        match &self.follow_target {
            Some(FollowTarget::Agent(id)) => {
                self.events.iter().filter(|e| e.subject_id == Some(*id)).collect()
            }
            Some(FollowTarget::Institution(id)) => {
                // For institutions, show events that mention the institution name
                if let Some(inst) = self.institutions.iter().find(|i| i.id == *id) {
                    let name = &inst.name;
                    self.events.iter().filter(|e| e.description.contains(name.as_str())).collect()
                } else {
                    Vec::new()
                }
            }
            None => Vec::new(),
        }
    }

    /// Get the map position of a followed agent (if following an agent).
    pub fn follow_agent_pos(&self) -> Option<(u32, u32)> {
        if let Some(FollowTarget::Agent(id)) = &self.follow_target {
            self.agents.iter().find(|a| a.id == *id && a.alive).map(|a| (a.x, a.y))
        } else {
            None
        }
    }

    /// Get institution name by ID.
    pub fn institution_name(&self, id: u64) -> Option<&str> {
        self.institutions.iter()
            .find(|i| i.id == id)
            .map(|i| i.name.as_str())
    }

    /// Run the appropriate number of ticks for the current speed setting.
    pub fn step_frame(&mut self, frame_count: u64) {
        match self.speed {
            SimSpeed::Paused => {}
            SimSpeed::Run05x => {
                // ~2.5 ticks/sec at 30 FPS — half speed
                if frame_count % 12 == 0 {
                    self.tick();
                }
            }
            SimSpeed::Run1x => {
                // ~5 ticks/sec at 30 FPS — slow enough to read log entries
                if frame_count % 6 == 0 {
                    self.tick();
                }
            }
            SimSpeed::Run5x => {
                for _ in 0..2 {
                    self.tick();
                }
            }
            SimSpeed::Run10x => {
                for _ in 0..10 {
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

        // Tick down eschaton flash
        if self.eschaton_flash > 0 {
            self.eschaton_flash -= 1;
        }
    }

    /// Scroll the log up by a given number of lines.
    /// On first scroll, freezes the view so new events don't affect it.
    pub fn scroll_log_up(&mut self, amount: usize) {
        if self.log_frozen_len.is_none() {
            self.log_frozen_len = Some(self.events.len());
        }
        let pool = self.log_frozen_len.unwrap();
        let max_scroll = pool.saturating_sub(1);
        self.log_scroll = (self.log_scroll + amount).min(max_scroll);
    }

    /// Scroll the log down (toward present). When scroll reaches 0, resume live mode.
    pub fn scroll_log_down(&mut self, amount: usize) {
        self.log_scroll = self.log_scroll.saturating_sub(amount);
        if self.log_scroll == 0 {
            self.log_frozen_len = None;
        }
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

    /// Get indices of all living agents, sorted by name.
    pub fn living_agent_indices(&self) -> Vec<usize> {
        let mut indices: Vec<usize> = self.agents
            .iter()
            .enumerate()
            .filter(|(_, a)| a.alive)
            .map(|(i, _)| i)
            .collect();
        indices.sort_by(|&a, &b| self.agents[a].name.cmp(&self.agents[b].name));
        indices
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
