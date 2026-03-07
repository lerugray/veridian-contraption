use rand::rngs::StdRng;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::sim::combat::{InjuryStatus, CombatHistoryEntry};
use crate::sim::event::EventType;
use crate::sim::world::{MAP_HEIGHT, MAP_WIDTH, Terrain};

/// The tone of a conversation between two agents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConversationTone {
    Warm,
    Tense,
    Cryptic,
    Mundane,
    Significant,
}

impl ConversationTone {
    pub fn display_color(&self) -> ratatui::style::Color {
        use ratatui::style::Color;
        match self {
            ConversationTone::Warm => Color::Rgb(100, 200, 140),
            ConversationTone::Tense => Color::Rgb(220, 100, 100),
            ConversationTone::Cryptic => Color::Rgb(220, 200, 100),
            ConversationTone::Mundane => Color::Rgb(150, 150, 150),
            ConversationTone::Significant => Color::Rgb(100, 200, 220),
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            ConversationTone::Warm => "Warm",
            ConversationTone::Tense => "Tense",
            ConversationTone::Cryptic => "Cryptic",
            ConversationTone::Mundane => "Mundane",
            ConversationTone::Significant => "Significant",
        }
    }
}

/// A conversation between two agents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub other_id: u64,
    pub tick: u64,
    pub line_a: String,
    pub line_b: String,
    pub tone: ConversationTone,
}

/// The kind of relationship between two agents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationshipKind {
    Friend,
    Rival,
    Partner,
    Mentor,
    Protege,
    Estranged,
}

impl RelationshipKind {
    pub fn label(&self) -> &'static str {
        match self {
            RelationshipKind::Friend => "Friend",
            RelationshipKind::Rival => "Rival",
            RelationshipKind::Partner => "Partner",
            RelationshipKind::Mentor => "Mentor",
            RelationshipKind::Protege => "Protégé",
            RelationshipKind::Estranged => "Estranged",
        }
    }

    pub fn display_color(&self) -> ratatui::style::Color {
        use ratatui::style::Color;
        match self {
            RelationshipKind::Friend => Color::Rgb(100, 200, 140),
            RelationshipKind::Rival => Color::Rgb(220, 100, 100),
            RelationshipKind::Partner => Color::Rgb(220, 160, 220),
            RelationshipKind::Mentor => Color::Rgb(180, 180, 240),
            RelationshipKind::Protege => Color::Rgb(140, 200, 240),
            RelationshipKind::Estranged => Color::Rgb(150, 150, 130),
        }
    }
}

/// A relationship between this agent and another.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    pub other_id: u64,
    pub kind: RelationshipKind,
    /// Intensity 1-3 (casual to deep).
    pub intensity: u8,
    /// Tick when this relationship was formed.
    pub formed_tick: u64,
}

/// Behavioral weights that shape how an agent acts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Disposition {
    pub risk_tolerance: f32,
    pub ambition: f32,
    pub institutional_loyalty: f32,
    pub paranoia: f32,
}

impl Disposition {
    /// Generate a random disposition.
    pub fn random(rng: &mut StdRng) -> Self {
        Self {
            risk_tolerance: rng.gen::<f32>(),
            ambition: rng.gen::<f32>(),
            institutional_loyalty: rng.gen::<f32>(),
            paranoia: rng.gen::<f32>(),
        }
    }
}

/// What an agent is currently trying to do.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Goal {
    /// Moving randomly through the world.
    Wander,
    /// Heading toward a specific settlement (index into World::settlements).
    SeekSettlement(usize),
    /// Resting in place (counts down ticks).
    Rest(u32),
    /// Seeking to join an institution (institution id).
    JoinInstitution(u64),
    /// Advancing within current institution (institution id).
    AdvanceInInstitution(u64),
    /// Founding a new institution (only high-ambition agents).
    FoundInstitution,
    /// Heading toward a site to explore it (site index).
    SeekSite(usize),
    /// Currently inside a site, resting/exploring (site index, ticks remaining).
    ExploreSite(usize, u32),
    /// Seeking to acquire an artifact from a site (artifact id, site index).
    AcquireArtifact(u64, usize),
    /// Returning an artifact to a settlement (artifact id, settlement index).
    ReturnArtifact(u64, usize),
    /// Seeking nearest settlement to recover from wounds (settlement index).
    SeekSettlementForHealing(usize),
}

/// An action result returned from Agent::act() to be turned into events by the sim.
pub struct AgentAction {
    pub agent_id: u64,
    pub event_type: EventType,
    pub old_pos: (u32, u32),
    pub new_pos: (u32, u32),
}

/// A single agent in the simulation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: u64,
    pub name: String,
    pub people_id: usize,
    pub x: u32,
    pub y: u32,
    pub health: u8,
    pub age: u32,
    pub disposition: Disposition,
    pub current_goal: Goal,
    pub chronicle: Vec<String>,
    /// Whether this agent is alive.
    pub alive: bool,
    /// Accumulated epithets (most recent last).
    #[serde(default)]
    pub epithets: Vec<String>,
    /// Tick when the last epithet was gained (prevents rapid accumulation).
    #[serde(default)]
    pub last_epithet_tick: u64,
    /// Institution IDs this agent belongs to (0-2).
    #[serde(default)]
    pub institution_ids: Vec<u64>,
    /// Whether this agent is an adventurer (high risk, seeks artifacts).
    #[serde(default)]
    pub is_adventurer: bool,
    /// Artifact IDs currently held by this agent.
    #[serde(default)]
    pub held_artifacts: Vec<u64>,
    /// Relationships with other agents.
    #[serde(default)]
    pub relationships: Vec<Relationship>,
    /// Recent conversations with other agents (capped at 20).
    #[serde(default)]
    pub conversations: Vec<Conversation>,
    /// Current injury status from combat.
    #[serde(default)]
    pub injury: InjuryStatus,
    /// Ticks remaining until injury is fully healed.
    #[serde(default)]
    pub recovery_remaining: u32,
    /// Total combats survived (drives experience tier).
    #[serde(default)]
    pub combats_survived: u16,
    /// Tick of last combat (prevents rapid re-engagement).
    #[serde(default)]
    pub last_combat_tick: u64,
    /// Recent combat history for inspect overlay (capped at 20, oldest dropped).
    #[serde(default)]
    pub combat_history: Vec<CombatHistoryEntry>,
}

impl Agent {
    /// Take one action for this tick. Returns any notable events that occurred.
    /// `site_positions` contains (grid_x, grid_y) for each site, indexed by site index.
    pub fn act(
        &mut self,
        rng: &mut StdRng,
        terrain: &[Vec<Terrain>],
        settlements: &[(u32, u32)],
        site_positions: &[(u32, u32)],
    ) -> Vec<AgentAction> {
        if !self.alive {
            return Vec::new();
        }

        let mut actions = Vec::new();
        let old_pos = (self.x, self.y);

        self.age += 1;

        // --- Injury recovery ---
        if self.recovery_remaining > 0 {
            self.recovery_remaining -= 1;
            if self.recovery_remaining == 0 {
                self.injury = InjuryStatus::Uninjured;
            }
        }

        // --- Gravely wounded death chance (each tick, until at settlement) ---
        if self.injury == InjuryStatus::GravelyWounded {
            let at_settlement = settlements.iter().any(|&(sx, sy)| self.x == sx && self.y == sy);
            if !at_settlement && rng.gen_bool(0.003) {
                self.alive = false;
                actions.push(AgentAction {
                    agent_id: self.id,
                    event_type: EventType::AgentDied,
                    old_pos,
                    new_pos: old_pos,
                });
                return actions;
            }
        }

        // Gradual mortality: chance of natural death increases with age.
        // Starts at age ~50 years (18250 ticks), ramps up significantly past ~70 (25550).
        // Hard cap at 36500 (~100 years).
        if self.age > 18250 {
            let age_factor = (self.age - 18250) as f64 / 18250.0; // 0.0 at 50yrs, 1.0 at 100yrs
            let death_chance = 0.00005 + age_factor * age_factor * 0.002; // ramps quadratically
            if self.age > 36500 || rng.gen_bool(death_chance.min(1.0)) {
                self.alive = false;
                actions.push(AgentAction {
                    agent_id: self.id,
                    event_type: EventType::NaturalDeath,
                    old_pos,
                    new_pos: old_pos,
                });
                return actions;
            }
        }

        // Age milestone events (every ~10 years = 3650 ticks)
        if self.age > 0 && self.age % 3650 == 0 {
            actions.push(AgentAction {
                agent_id: self.id,
                event_type: EventType::AgeEvent,
                old_pos,
                new_pos: old_pos,
            });
        }

        match &self.current_goal {
            Goal::Wander => {
                self.wander(rng, terrain);
                self.maybe_change_goal(rng, settlements, site_positions);
            }
            Goal::SeekSettlement(idx) => {
                let idx = *idx;
                if idx < settlements.len() {
                    let (sx, sy) = settlements[idx];
                    self.move_toward(sx, sy, terrain);
                    // Arrived at settlement?
                    if self.x == sx && self.y == sy && (old_pos.0 != sx || old_pos.1 != sy) {
                        actions.push(AgentAction {
                            agent_id: self.id,
                            event_type: EventType::AgentArrived,
                            old_pos,
                            new_pos: (self.x, self.y),
                        });
                        self.current_goal = Goal::Rest(rng.gen_range(10..=50));
                    }
                } else {
                    self.current_goal = Goal::Wander;
                }
            }
            Goal::Rest(remaining) => {
                let remaining = *remaining;
                if remaining <= 1 {
                    // About to leave — generate a departure event if at a settlement
                    let at_settlement = settlements.iter().any(|&(sx, sy)| self.x == sx && self.y == sy);
                    if at_settlement {
                        actions.push(AgentAction {
                            agent_id: self.id,
                            event_type: EventType::AgentDeparted,
                            old_pos,
                            new_pos: old_pos,
                        });
                    }
                    self.maybe_change_goal(rng, settlements, site_positions);
                } else {
                    self.current_goal = Goal::Rest(remaining - 1);
                }
            }
            Goal::SeekSite(idx) => {
                let idx = *idx;
                if idx < site_positions.len() {
                    let (sx, sy) = site_positions[idx];
                    self.move_toward(sx, sy, terrain);
                    // Arrived at site?
                    if self.x == sx && self.y == sy && (old_pos.0 != sx || old_pos.1 != sy) {
                        actions.push(AgentAction {
                            agent_id: self.id,
                            event_type: EventType::AgentEnteredSite,
                            old_pos,
                            new_pos: (self.x, self.y),
                        });
                        self.current_goal = Goal::ExploreSite(idx, rng.gen_range(20..=80));
                    }
                } else {
                    self.current_goal = Goal::Wander;
                }
            }
            Goal::ExploreSite(idx, remaining) => {
                let idx = *idx;
                let remaining = *remaining;
                if remaining <= 1 {
                    actions.push(AgentAction {
                        agent_id: self.id,
                        event_type: EventType::AgentLeftSite,
                        old_pos,
                        new_pos: old_pos,
                    });
                    self.maybe_change_goal(rng, settlements, site_positions);
                } else {
                    self.current_goal = Goal::ExploreSite(idx, remaining - 1);
                }
            }
            Goal::AcquireArtifact(_artifact_id, site_idx) => {
                let site_idx = *site_idx;
                if site_idx < site_positions.len() {
                    let (sx, sy) = site_positions[site_idx];
                    if self.x == sx && self.y == sy {
                        // At the site — acquisition is handled by sim tick
                        // Stay here, goal will be changed by sim
                    } else {
                        self.move_toward(sx, sy, terrain);
                        if self.x == sx && self.y == sy {
                            actions.push(AgentAction {
                                agent_id: self.id,
                                event_type: EventType::AgentEnteredSite,
                                old_pos,
                                new_pos: (self.x, self.y),
                            });
                        }
                    }
                } else {
                    self.current_goal = Goal::Wander;
                }
            }
            Goal::ReturnArtifact(_artifact_id, settlement_idx) => {
                let settlement_idx = *settlement_idx;
                if settlement_idx < settlements.len() {
                    let (sx, sy) = settlements[settlement_idx];
                    self.move_toward(sx, sy, terrain);
                    // Arrival is handled by sim tick
                } else {
                    self.current_goal = Goal::Wander;
                }
            }
            // Institutional goals resolve in the sim tick loop, not here.
            // Agent wanders while pursuing them.
            Goal::JoinInstitution(_) | Goal::AdvanceInInstitution(_) | Goal::FoundInstitution => {
                self.wander(rng, terrain);
                // These goals persist for a while; the sim tick handles resolution.
            }
            Goal::SeekSettlementForHealing(idx) => {
                let idx = *idx;
                if idx < settlements.len() {
                    let (sx, sy) = settlements[idx];
                    // Wounded agents move every other tick
                    let should_move = self.injury != InjuryStatus::Wounded || self.age % 2 == 0;
                    if should_move {
                        self.move_toward(sx, sy, terrain);
                    }
                    // Arrived at settlement?
                    if self.x == sx && self.y == sy {
                        if old_pos.0 != sx || old_pos.1 != sy {
                            actions.push(AgentAction {
                                agent_id: self.id,
                                event_type: EventType::AgentArrived,
                                old_pos,
                                new_pos: (self.x, self.y),
                            });
                        }
                        // Start recovering: rest until healed
                        let rest_ticks = self.recovery_remaining.max(20);
                        self.current_goal = Goal::Rest(rest_ticks);
                    }
                } else {
                    self.current_goal = Goal::Wander;
                }
            }
        }

        actions
    }

    /// Move one tile in a random walkable direction.
    fn wander(&mut self, rng: &mut StdRng, terrain: &[Vec<Terrain>]) {
        let dx: i32 = rng.gen_range(-1..=1);
        let dy: i32 = rng.gen_range(-1..=1);
        self.try_move(dx, dy, terrain);
    }

    /// Move one tile toward a target position.
    fn move_toward(&mut self, tx: u32, ty: u32, terrain: &[Vec<Terrain>]) {
        let dx = (tx as i32 - self.x as i32).signum();
        let dy = (ty as i32 - self.y as i32).signum();
        self.try_move(dx, dy, terrain);
    }

    /// Attempt to move by (dx, dy), checking bounds and terrain walkability.
    fn try_move(&mut self, dx: i32, dy: i32, terrain: &[Vec<Terrain>]) {
        let nx = self.x as i32 + dx;
        let ny = self.y as i32 + dy;

        if nx < 0 || ny < 0 || nx >= MAP_WIDTH as i32 || ny >= MAP_HEIGHT as i32 {
            return;
        }

        let t = terrain[ny as usize][nx as usize];
        if t != Terrain::DeepWater {
            self.x = nx as u32;
            self.y = ny as u32;
        }
    }

    /// Possibly switch to a new goal based on disposition weights.
    pub fn maybe_change_goal(&mut self, rng: &mut StdRng, settlements: &[(u32, u32)], site_positions: &[(u32, u32)]) {
        // Wounded agents seek settlement for healing after completing their current goal
        if self.injury == InjuryStatus::Wounded && !settlements.is_empty() {
            if let Some((nearest_idx, _)) = settlements.iter().enumerate()
                .min_by_key(|(_, &(sx, sy))| {
                    let dx = self.x as i32 - sx as i32;
                    let dy = self.y as i32 - sy as i32;
                    dx * dx + dy * dy
                })
            {
                self.current_goal = Goal::SeekSettlementForHealing(nearest_idx);
                return;
            }
        }

        let roll: f32 = rng.gen();

        // High-ambition agents with no institution may try to found one
        if self.institution_ids.is_empty()
            && self.disposition.ambition > 0.8
            && roll < 0.05
        {
            self.current_goal = Goal::FoundInstitution;
            return;
        }

        // Agents with high institutional loyalty may advance in their institution
        if !self.institution_ids.is_empty()
            && self.disposition.institutional_loyalty > 0.5
            && roll < self.disposition.institutional_loyalty * 0.15
        {
            let inst_id = self.institution_ids[rng.gen_range(0..self.institution_ids.len())];
            self.current_goal = Goal::AdvanceInInstitution(inst_id);
            return;
        }

        // Risk-tolerant agents may seek out a site to explore
        if !site_positions.is_empty()
            && self.disposition.risk_tolerance > 0.4
            && roll < self.disposition.risk_tolerance * 0.12
        {
            let idx = rng.gen_range(0..site_positions.len());
            self.current_goal = Goal::SeekSite(idx);
            return;
        }

        if roll < self.disposition.ambition * 0.3 && !settlements.is_empty() {
            let idx = rng.gen_range(0..settlements.len());
            self.current_goal = Goal::SeekSettlement(idx);
        } else if roll > 1.0 - (1.0 - self.disposition.risk_tolerance) * 0.3 {
            self.current_goal = Goal::Rest(rng.gen_range(5..=30));
        } else {
            self.current_goal = Goal::Wander;
        }
    }

    /// Name with primary epithet for log entries and display.
    pub fn display_name(&self) -> String {
        if let Some(epithet) = self.epithets.last() {
            format!("{} {}", self.name, epithet)
        } else {
            self.name.clone()
        }
    }
}
