pub mod world;
pub mod agent;
pub mod event;
pub mod institution;
pub mod site;
pub mod artifact;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};

use crate::gen::name_gen;
use crate::gen::prose_gen;
use crate::sim::agent::Agent;
use crate::sim::artifact::Artifact;
use crate::sim::event::{Event, EventType};
use crate::sim::institution::Institution;
use crate::sim::site::Site;
use crate::sim::world::World;

/// Maximum number of events kept in the log ring buffer.
const MAX_EVENTS: usize = 200;

/// Number of major events that must accumulate to trigger an era transition.
pub const ERA_THRESHOLD: u32 = 15;

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
    /// Inspecting a specific agent by index into the agents vec.
    InspectAgent(usize),
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
}

impl SimState {
    pub fn new(world: World, agents: Vec<Agent>, institutions: Vec<Institution>, sites: Vec<Site>, artifacts: Vec<Artifact>) -> Self {
        let mut rng = StdRng::seed_from_u64(world.seed.wrapping_add(1));
        let next_id = institutions.iter().map(|i| i.id + 1).max().unwrap_or(0);
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
            next_institution_id: next_id,
            follow_target: None,
            pre_pause_speed: None,
            annals: Vec::new(),
            current_era_name: first_era,
            current_era_start: 0,
            era_major_events: 0,
            era_notable_agents: Vec::new(),
            era_notable_institutions: Vec::new(),
            era_defining_event: None,
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
        }
    }

    /// Reconstruct a SimState from loaded save data.
    pub fn from_save_data(data: SaveData) -> Self {
        let mut rng = StdRng::seed_from_u64(data.rng_state_seed);
        let last_tick = data.world.tick;
        let next_id = data.institutions.iter().map(|i| i.id + 1).max().unwrap_or(0);
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
            next_institution_id: next_id,
            follow_target: data.follow_target,
            pre_pause_speed: None,
            annals: data.annals,
            current_era_name: era_name,
            current_era_start: data.current_era_start,
            era_major_events: data.era_major_events,
            era_notable_agents: Vec::new(),
            era_notable_institutions: Vec::new(),
            era_defining_event: None,
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

        // Build site positions for agent site-seeking.
        let site_positions: Vec<(u32, u32)> = self
            .sites
            .iter()
            .map(|s| (s.grid_x, s.grid_y))
            .collect();

        // Process all agent actions and collect resulting events.
        let mut new_events: Vec<Event> = Vec::new();

        for agent in &mut self.agents {
            let actions = agent.act(&mut self.rng, &self.world.terrain, &settlement_positions, &site_positions);

            for action in actions {
                let agent_name = agent.display_name();

                // For site events, use the site name instead of nearest settlement
                let description = match &action.event_type {
                    EventType::AgentEnteredSite | EventType::AgentLeftSite => {
                        // Find which site is at this position
                        let site_name = self.sites.iter()
                            .find(|s| s.grid_x == action.new_pos.0 && s.grid_y == action.new_pos.1)
                            .map(|s| s.name.as_str())
                            .unwrap_or("an unnamed site");
                        prose_gen::generate_site_description(
                            &action.event_type,
                            &agent_name,
                            site_name,
                            &mut self.rng,
                        )
                    }
                    _ => {
                        let loc_name = prose_gen::nearest_settlement_name(
                            action.new_pos.0,
                            action.new_pos.1,
                            &self.world,
                        );
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

        // Weather events — interval scaled by temporal_rate and ecological_volatility
        let weather_interval = (50.0 / (self.world.params.temporal_rate * (0.5 + self.world.params.ecological_volatility))).max(5.0) as u64;
        if tick % weather_interval == 0 && !self.world.settlements.is_empty() {
            let idx = self.rng.gen_range(0..self.world.settlements.len());
            let s = &self.world.settlements[idx];
            let loc_name = s.name.clone();
            let description = prose_gen::generate_description(
                &EventType::WeatherEvent,
                None,
                Some(&loc_name),
                tick,
                &mut self.rng,
                self.world.params.narrative_register,
                self.world.params.weirdness_coefficient,
            );
            new_events.push(Event {
                tick,
                event_type: EventType::WeatherEvent,
                subject_id: None,
                location: Some((s.x as u32, s.y as u32)),
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

        // --- Institutional simulation ---
        let mut inst_events = self.process_institutional_tick(tick);
        new_events.append(&mut inst_events);

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
                    let epithet = name_gen::generate_epithet(
                        &event.event_type,
                        loc_name.as_deref(),
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

                let inst_name = name_gen::generate_institution_name(&kind, &phonemes, people_id, &mut self.rng);
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
                    if agent.disposition.institutional_loyalty > 0.4 && self.rng.gen_bool(0.005) {
                        let inst_id = alive_institutions[self.rng.gen_range(0..alive_institutions.len())];
                        agent.current_goal = agent::Goal::JoinInstitution(inst_id);
                    }
                }
            }
        }

        // Periodic institutional events — interval scaled by temporal_rate and political_churn
        let inst_event_interval = (75.0 / (self.world.params.temporal_rate * (0.5 + self.world.params.political_churn))).max(5.0) as u64;
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

        // Relationship events — interval scaled by temporal_rate and political_churn
        let relation_interval = (150.0 / (self.world.params.temporal_rate * (0.5 + self.world.params.political_churn))).max(10.0) as u64;
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

        // Dissolve institutions with 0 members (check every 200 ticks)
        if tick % 200 == 0 {
            for inst in &mut self.institutions {
                if !inst.alive { continue; }
                // Remove dead agents from member lists
                let living_ids: std::collections::HashSet<u64> = self.agents.iter()
                    .filter(|a| a.alive)
                    .map(|a| a.id)
                    .collect();
                inst.member_ids.retain(|id| living_ids.contains(id));

                if inst.member_ids.is_empty() {
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
            let new_name = name_gen::generate_institution_name(&new_kind, phonemes, people_id, &mut self.rng);
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
        );

        Some(Event {
            tick,
            event_type,
            subject_id: Some(agent_id),
            location: Some((self.agents[ai].x, self.agents[ai].y)),
            description,
        })
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
