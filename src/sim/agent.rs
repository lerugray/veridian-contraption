use rand::rngs::StdRng;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::sim::world::{MAP_HEIGHT, MAP_WIDTH, Terrain};

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
}

impl Agent {
    /// Take one action for this tick, mutating position and goal as needed.
    pub fn act(
        &mut self,
        rng: &mut StdRng,
        terrain: &[Vec<Terrain>],
        settlements: &[(u32, u32)],
    ) {
        if !self.alive {
            return;
        }

        self.age += 1;

        // Die of old age at 36500 ticks (~100 years).
        if self.age > 36500 {
            self.alive = false;
            self.chronicle.push(format!(
                "Tick {}: Passed from this world at a venerable age.",
                self.age
            ));
            return;
        }

        match &self.current_goal {
            Goal::Wander => {
                self.wander(rng, terrain);
                self.maybe_change_goal(rng, settlements);
            }
            Goal::SeekSettlement(idx) => {
                let idx = *idx;
                if idx < settlements.len() {
                    let (sx, sy) = settlements[idx];
                    self.move_toward(sx, sy, terrain);
                    // Arrived?
                    if self.x == sx && self.y == sy {
                        self.current_goal = Goal::Rest(rng.gen_range(10..=50));
                    }
                } else {
                    // Invalid target, wander instead
                    self.current_goal = Goal::Wander;
                }
            }
            Goal::Rest(remaining) => {
                let remaining = *remaining;
                if remaining <= 1 {
                    self.maybe_change_goal(rng, settlements);
                } else {
                    self.current_goal = Goal::Rest(remaining - 1);
                }
            }
        }
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
        // Agents can walk on anything except deep water
        if t != Terrain::DeepWater {
            self.x = nx as u32;
            self.y = ny as u32;
        }
    }

    /// Possibly switch to a new goal based on disposition weights.
    fn maybe_change_goal(&mut self, rng: &mut StdRng, settlements: &[(u32, u32)]) {
        let roll: f32 = rng.gen();

        // Higher ambition = more likely to seek a settlement
        // Higher risk_tolerance = less likely to rest
        if roll < self.disposition.ambition * 0.3 && !settlements.is_empty() {
            let idx = rng.gen_range(0..settlements.len());
            self.current_goal = Goal::SeekSettlement(idx);
        } else if roll > 1.0 - (1.0 - self.disposition.risk_tolerance) * 0.3 {
            self.current_goal = Goal::Rest(rng.gen_range(5..=30));
        } else {
            self.current_goal = Goal::Wander;
        }
    }
}
