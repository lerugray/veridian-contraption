use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use crate::sim::agent::{Agent, Disposition, Goal};
use crate::sim::world::*;

/// Generate a complete world from a seed.
pub fn generate_world(seed: u64) -> (World, Vec<Agent>) {
    let mut rng = StdRng::seed_from_u64(seed);

    let name = generate_world_name(&mut rng);
    let heightmap = generate_heightmap(&mut rng);
    let moisture = generate_heightmap(&mut rng); // second noise pass for moisture
    let terrain = classify_terrain(&heightmap, &moisture);
    let settlements = place_settlements(&terrain, &mut rng);
    let peoples = generate_peoples(&mut rng);
    let agents = generate_agents(&settlements, &peoples, &mut rng);

    let world = World {
        seed,
        name,
        terrain,
        settlements,
        peoples,
        tick: 0,
    };

    (world, agents)
}

/// Placeholder world name generator — will be replaced by phoneme-based gen later.
fn generate_world_name(rng: &mut StdRng) -> String {
    let prefixes = ["Vel", "Gor", "Pelmr", "Anqu", "Drev", "Ossir", "Thal", "Krev"];
    let suffixes = ["wick", "oth", "andria", "ium", "ent", "aska", "is", "orn"];
    let p = prefixes[rng.gen_range(0..prefixes.len())];
    let s = suffixes[rng.gen_range(0..suffixes.len())];
    format!("{}{}", p, s)
}

// ---------------------------------------------------------------------------
// Heightmap generation — value noise with interpolation
// ---------------------------------------------------------------------------

/// Generate a heightmap using value noise at multiple octaves.
/// Returns a MAP_HEIGHT x MAP_WIDTH grid of f64 values in [0.0, 1.0].
fn generate_heightmap(rng: &mut StdRng) -> Vec<Vec<f64>> {
    // Base grid of random values at coarse resolution, then interpolate.
    // Two octaves: coarse (8x4 grid) and fine (16x8 grid), blended.
    let coarse = noise_octave(rng, 8, 4);
    let fine = noise_octave(rng, 16, 8);

    let mut map = vec![vec![0.0; MAP_WIDTH]; MAP_HEIGHT];
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            // Blend octaves: 70% coarse, 30% fine
            let val = coarse[y][x] * 0.7 + fine[y][x] * 0.3;
            map[y][x] = val;
        }
    }

    // Normalize to [0, 1]
    let mut min = f64::MAX;
    let mut max = f64::MIN;
    for row in &map {
        for &v in row {
            if v < min { min = v; }
            if v > max { max = v; }
        }
    }
    let range = (max - min).max(0.001);
    for row in &mut map {
        for v in row {
            *v = (*v - min) / range;
        }
    }

    map
}

/// Generate one octave of value noise: create a grid_w x grid_h lattice
/// of random values, then bilinearly interpolate to MAP_WIDTH x MAP_HEIGHT.
fn noise_octave(rng: &mut StdRng, grid_w: usize, grid_h: usize) -> Vec<Vec<f64>> {
    // Create lattice
    let mut lattice = vec![vec![0.0f64; grid_w + 1]; grid_h + 1];
    for row in &mut lattice {
        for v in row.iter_mut() {
            *v = rng.gen::<f64>();
        }
    }

    let mut result = vec![vec![0.0; MAP_WIDTH]; MAP_HEIGHT];

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            // Map pixel coords to lattice coords
            let lx = (x as f64 / MAP_WIDTH as f64) * grid_w as f64;
            let ly = (y as f64 / MAP_HEIGHT as f64) * grid_h as f64;

            let x0 = lx.floor() as usize;
            let y0 = ly.floor() as usize;
            let x1 = (x0 + 1).min(grid_w);
            let y1 = (y0 + 1).min(grid_h);

            let fx = lx - x0 as f64;
            let fy = ly - y0 as f64;

            // Smoothstep for less blocky results
            let sx = fx * fx * (3.0 - 2.0 * fx);
            let sy = fy * fy * (3.0 - 2.0 * fy);

            // Bilinear interpolation
            let top = lattice[y0][x0] * (1.0 - sx) + lattice[y0][x1] * sx;
            let bot = lattice[y1][x0] * (1.0 - sx) + lattice[y1][x1] * sx;
            result[y][x] = top * (1.0 - sy) + bot * sy;
        }
    }

    result
}

// ---------------------------------------------------------------------------
// Terrain classification
// ---------------------------------------------------------------------------

/// Convert heightmap + moisture values to terrain types.
fn classify_terrain(height: &[Vec<f64>], moisture: &[Vec<f64>]) -> Vec<Vec<Terrain>> {
    let mut terrain = vec![vec![Terrain::Plains; MAP_WIDTH]; MAP_HEIGHT];

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let h = height[y][x];
            let m = moisture[y][x];

            terrain[y][x] = if h < 0.20 {
                Terrain::DeepWater
            } else if h < 0.30 {
                Terrain::ShallowWater
            } else if h > 0.80 {
                Terrain::Mountains
            } else if h > 0.65 {
                Terrain::Hills
            } else if m < 0.30 {
                // Low moisture + mid elevation = desert
                Terrain::Desert
            } else if m > 0.60 {
                // High moisture + mid elevation = forest
                Terrain::Forest
            } else {
                Terrain::Plains
            };
        }
    }

    terrain
}

// ---------------------------------------------------------------------------
// Settlement placement
// ---------------------------------------------------------------------------

/// Place 6-12 settlements on habitable terrain, spaced apart.
fn place_settlements(terrain: &[Vec<Terrain>], rng: &mut StdRng) -> Vec<Settlement> {
    let count = rng.gen_range(6..=12);
    let mut settlements = Vec::new();

    // Collect habitable positions
    let mut candidates: Vec<(usize, usize)> = Vec::new();
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            if terrain[y][x].is_habitable() {
                candidates.push((x, y));
            }
        }
    }

    // Minimum distance between settlements (Manhattan distance)
    let min_dist = 5;

    let mut attempts = 0;
    while settlements.len() < count && attempts < 500 {
        attempts += 1;
        let idx = rng.gen_range(0..candidates.len());
        let (x, y) = candidates[idx];

        // Check distance from existing settlements
        let too_close = settlements.iter().any(|s: &Settlement| {
            let dx = (s.x as i32 - x as i32).unsigned_abs() as usize;
            let dy = (s.y as i32 - y as i32).unsigned_abs() as usize;
            dx + dy < min_dist
        });

        if too_close {
            continue;
        }

        let size = match rng.gen_range(0..10) {
            0..=1 => SettlementSize::City,
            2..=5 => SettlementSize::Town,
            _ => SettlementSize::Hamlet,
        };

        let n = settlements.len() + 1;
        let name = format!("Settlement_{}", n);

        settlements.push(Settlement { name, size, x, y });
    }

    settlements
}

// ---------------------------------------------------------------------------
// Peoples generation
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Agent generation
// ---------------------------------------------------------------------------

/// Generate 40-80 agents distributed across settlements.
fn generate_agents(
    settlements: &[Settlement],
    peoples: &[People],
    rng: &mut StdRng,
) -> Vec<Agent> {
    let count = rng.gen_range(40..=80);
    let mut agents = Vec::with_capacity(count);

    // Agent name parts — placeholder until phoneme-based gen in Phase 2
    let first_parts = [
        "Whelm", "Orrith", "Gask", "Pelm", "Thren", "Duvv", "Quor", "Anx",
        "Brevv", "Ilt", "Noch", "Vrem", "Solk", "Jurr", "Ersk", "Tobb",
        "Krev", "Mund", "Plix", "Zarr", "Felk", "Grint", "Hoss", "Lebb",
    ];
    let last_parts = [
        "Durr-Anquist", "the Appointed", "of the Reach", "Velmson",
        "Greywick", "Pallmark", "the Noted", "Inkster",
        "Thornwise", "Quillbent", "the Enumerated", "of Pelm",
        "Drossward", "the Provisional", "Axleworth", "Stumpkin",
    ];

    for i in 0..count {
        let first = first_parts[rng.gen_range(0..first_parts.len())];
        let last = last_parts[rng.gen_range(0..last_parts.len())];
        let name = format!("{} {}", first, last);

        // Assign to a random settlement
        let settlement_idx = rng.gen_range(0..settlements.len());
        let s = &settlements[settlement_idx];

        // Assign to a random people
        let people_id = rng.gen_range(0..peoples.len());

        // Start with a random age (in ticks) — 0 to ~50 years
        let age = rng.gen_range(0..18250);

        agents.push(Agent {
            id: i as u64,
            name,
            people_id,
            x: s.x as u32,
            y: s.y as u32,
            health: rng.gen_range(60..=100),
            age,
            disposition: Disposition::random(rng),
            current_goal: Goal::Wander,
            chronicle: Vec::new(),
            alive: true,
        });
    }

    agents
}

// ---------------------------------------------------------------------------
// Peoples generation
// ---------------------------------------------------------------------------

/// Generate 3-4 peoples with placeholder names and terrain preferences.
fn generate_peoples(rng: &mut StdRng) -> Vec<People> {
    let count = rng.gen_range(3..=4);
    let mut peoples = Vec::new();

    let all_terrains = [
        Terrain::Plains,
        Terrain::Hills,
        Terrain::Forest,
        Terrain::Desert,
        Terrain::Mountains,
    ];

    let name_parts = [
        ("Grev", "vin"),
        ("Thal", "ori"),
        ("Pelm", "ack"),
        ("Oss", "ren"),
    ];

    for i in 0..count {
        let (prefix, suffix) = name_parts[i];
        let name = format!("{}{}", prefix, suffix);

        // Each people prefers 2-3 terrain types
        let pref_count = rng.gen_range(2..=3);
        let mut preferred = Vec::new();
        let mut used = [false; 5];
        while preferred.len() < pref_count {
            let idx = rng.gen_range(0..all_terrains.len());
            if !used[idx] {
                used[idx] = true;
                preferred.push(all_terrains[idx]);
            }
        }

        let population = rng.gen_range(200..=2000);

        peoples.push(People {
            name,
            preferred_terrain: preferred,
            population,
        });
    }

    peoples
}
