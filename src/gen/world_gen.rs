use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use std::collections::HashMap;

use crate::gen::{name_gen, dungeon_gen, artifact_gen};
use crate::sim::agent::{Agent, Disposition, Goal};
use crate::sim::artifact::Artifact;
use crate::sim::institution::{Institution, InstitutionKind};
use crate::sim::site::Site;
use crate::sim::world::*;

/// Flavor presets that bias world parameter generation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WorldFlavor {
    TheLongBureaucracy,
    TheBurningProvinces,
    TheDeepTaxonomy,
    TheConspiratorialAge,
    Unguided,
}

impl WorldFlavor {
    /// Get flavor by index (matches menu order).
    pub fn from_index(i: usize) -> Self {
        match i {
            0 => WorldFlavor::TheLongBureaucracy,
            1 => WorldFlavor::TheBurningProvinces,
            2 => WorldFlavor::TheDeepTaxonomy,
            3 => WorldFlavor::TheConspiratorialAge,
            _ => WorldFlavor::Unguided,
        }
    }
}

/// Generate world parameters based on flavor preset, with variance.
fn generate_params(flavor: WorldFlavor, rng: &mut StdRng) -> WorldParams {
    // Helper: generate a biased value within a range
    let biased = |center: f32, spread: f32, rng: &mut StdRng| -> f32 {
        let v = center + rng.gen_range(-spread..=spread);
        v.clamp(0.0, 1.0)
    };
    let biased_rate = |center: f32, spread: f32, rng: &mut StdRng| -> f32 {
        let v = center + rng.gen_range(-spread..=spread);
        v.clamp(0.1, 3.0)
    };

    match flavor {
        WorldFlavor::TheLongBureaucracy => WorldParams {
            temporal_rate: biased_rate(0.4, 0.15, rng),
            political_churn: biased(0.7, 0.15, rng),
            cosmological_density: biased(0.25, 0.15, rng),
            ecological_volatility: biased(0.2, 0.1, rng),
            narrative_register: NarrativeRegister::Bureaucratic,
            weirdness_coefficient: biased(0.4, 0.2, rng),
        },
        WorldFlavor::TheBurningProvinces => WorldParams {
            temporal_rate: biased_rate(2.0, 0.5, rng),
            political_churn: biased(0.8, 0.1, rng),
            cosmological_density: biased(0.4, 0.2, rng),
            ecological_volatility: biased(0.75, 0.15, rng),
            narrative_register: NarrativeRegister::Ominous,
            weirdness_coefficient: biased(0.5, 0.2, rng),
        },
        WorldFlavor::TheDeepTaxonomy => WorldParams {
            temporal_rate: biased_rate(1.0, 0.3, rng),
            political_churn: biased(0.4, 0.15, rng),
            cosmological_density: biased(0.5, 0.2, rng),
            ecological_volatility: biased(0.7, 0.15, rng),
            narrative_register: NarrativeRegister::Clinical,
            weirdness_coefficient: biased(0.7, 0.15, rng),
        },
        WorldFlavor::TheConspiratorialAge => WorldParams {
            temporal_rate: biased_rate(1.2, 0.3, rng),
            political_churn: biased(0.7, 0.15, rng),
            cosmological_density: biased(0.8, 0.1, rng),
            ecological_volatility: biased(0.3, 0.15, rng),
            narrative_register: NarrativeRegister::Conspiratorial,
            weirdness_coefficient: biased(0.6, 0.2, rng),
        },
        WorldFlavor::Unguided => WorldParams {
            temporal_rate: biased_rate(1.0, 0.9, rng),
            political_churn: rng.gen_range(0.1..=0.9),
            cosmological_density: rng.gen_range(0.05..=0.95),
            ecological_volatility: rng.gen_range(0.1..=0.9),
            narrative_register: match rng.gen_range(0..5) {
                0 => NarrativeRegister::Clinical,
                1 => NarrativeRegister::Lyrical,
                2 => NarrativeRegister::Bureaucratic,
                3 => NarrativeRegister::Ominous,
                _ => NarrativeRegister::Conspiratorial,
            },
            weirdness_coefficient: rng.gen_range(0.1..=0.9),
        },
    }
}

/// Generate a complete world from a seed and flavor preset.
pub fn generate_world(seed: u64, flavor: WorldFlavor) -> (World, Vec<Agent>, Vec<Institution>, Vec<Site>, Vec<Artifact>) {
    let mut rng = StdRng::seed_from_u64(seed);
    let phonemes = name_gen::load_phoneme_data();
    let params = generate_params(flavor, &mut rng);

    let name = name_gen::generate_world_name(&phonemes, &mut rng);
    let heightmap = generate_heightmap(&mut rng);
    let moisture = generate_heightmap(&mut rng); // second noise pass for moisture
    let terrain = classify_terrain(&heightmap, &moisture);
    let mut settlements = place_settlements(&terrain, &mut rng);
    let peoples = generate_peoples(&mut rng, &phonemes);

    // Name settlements and generate floor plans
    for settlement in &mut settlements {
        let people_id = rng.gen_range(0..peoples.len());
        settlement.name = name_gen::generate_settlement_name(
            &phonemes,
            peoples[people_id].phoneme_set,
            &mut rng,
        );
        settlement.floor = Some(dungeon_gen::generate_settlement_floor(&settlement.size, &mut rng));
    }

    let mut agents = generate_agents(&settlements, &peoples, &phonemes, &mut rng);

    // Generate 5-10 adventurer agents with adventurer dispositions
    let adventurer_count = rng.gen_range(5..=10);
    let next_id = agents.len() as u64;
    for i in 0..adventurer_count {
        let people_id = rng.gen_range(0..peoples.len());
        let name = name_gen::generate_personal_name(&phonemes, peoples[people_id].phoneme_set, &mut rng);
        let settlement_idx = rng.gen_range(0..settlements.len());
        let s = &settlements[settlement_idx];
        let age = rng.gen_range(1825..10950); // 5-30 years old

        agents.push(Agent {
            id: next_id + i as u64,
            name,
            people_id,
            x: s.x as u32,
            y: s.y as u32,
            health: rng.gen_range(70..=100),
            age,
            disposition: Disposition {
                risk_tolerance: rng.gen_range(0.7..=1.0),
                ambition: rng.gen_range(0.6..=1.0),
                institutional_loyalty: rng.gen_range(0.0..=0.3),
                paranoia: rng.gen_range(0.1..=0.6),
            },
            current_goal: Goal::Wander,
            chronicle: Vec::new(),
            alive: true,
            epithets: Vec::new(),
            last_epithet_tick: 0,
            institution_ids: Vec::new(),
            is_adventurer: true,
            held_artifacts: Vec::new(),
        });
    }

    let institutions = generate_institutions(&settlements, &peoples, &phonemes, &mut agents, &params, &mut rng);

    // Generate sites, passing institution info for controlling faction assignment
    let inst_info: Vec<(u64, String)> = institutions.iter().map(|i| (i.id, i.name.clone())).collect();
    let sites = dungeon_gen::generate_sites(&terrain, &phonemes, &inst_info, &mut rng);

    // Generate artifacts, distributed between sites and settlements
    let settlement_names: Vec<String> = settlements.iter().map(|s| s.name.clone()).collect();
    let site_names: Vec<String> = sites.iter().map(|s| s.name.clone()).collect();
    let artifacts = artifact_gen::generate_artifacts(
        sites.len(),
        settlements.len(),
        &settlement_names,
        &site_names,
        &inst_info,
        &phonemes,
        &mut rng,
    );

    // Record artifact IDs on the sites that hold them
    // (sites.artifacts vec is already empty, fill from artifact locations)
    // We can't mutate sites here since we need to return them, but we'll build
    // the artifact->site mapping and apply it.
    let mut site_artifacts: Vec<Vec<u64>> = vec![Vec::new(); sites.len()];
    for artifact in &artifacts {
        if let crate::sim::artifact::ArtifactLocation::InSite(idx) = &artifact.current_location {
            if *idx < sites.len() {
                site_artifacts[*idx].push(artifact.id);
            }
        }
    }
    let mut sites = sites;
    for (i, ids) in site_artifacts.into_iter().enumerate() {
        sites[i].artifacts = ids;
    }

    let world = World {
        seed,
        name,
        terrain,
        settlements,
        peoples,
        tick: 0,
        params,
    };

    (world, agents, institutions, sites, artifacts)
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

        // Placeholder name — replaced after peoples are generated
        let name = format!("Settlement_{}", settlements.len() + 1);

        settlements.push(Settlement { name, size, x, y, floor: None });
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
    phonemes: &[name_gen::PhonemeSet],
    rng: &mut StdRng,
) -> Vec<Agent> {
    let count = rng.gen_range(40..=80);
    let mut agents = Vec::with_capacity(count);

    for i in 0..count {
        // Assign to a random people — name uses that people's phoneme set
        let people_id = rng.gen_range(0..peoples.len());
        let name = name_gen::generate_personal_name(phonemes, peoples[people_id].phoneme_set, rng);

        // Assign to a random settlement
        let settlement_idx = rng.gen_range(0..settlements.len());
        let s = &settlements[settlement_idx];

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
            epithets: Vec::new(),
            last_epithet_tick: 0,
            institution_ids: Vec::new(),
            is_adventurer: false,
            held_artifacts: Vec::new(),
        });
    }

    agents
}

// ---------------------------------------------------------------------------
// Peoples generation
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Institution generation
// ---------------------------------------------------------------------------

/// Generate 4-8 starting institutions distributed across peoples.
fn generate_institutions(
    settlements: &[Settlement],
    peoples: &[People],
    phonemes: &[name_gen::PhonemeSet],
    agents: &mut [Agent],
    params: &WorldParams,
    rng: &mut StdRng,
) -> Vec<Institution> {
    // High political churn = more institutions at start
    let base_count = 4 + (params.political_churn * 6.0) as usize; // 4-10
    let count = rng.gen_range(base_count.saturating_sub(1)..=base_count.min(12));
    let mut institutions = Vec::new();

    for i in 0..count {
        let people_id = i % peoples.len();
        // High cosmological_density biases toward SecretSociety and Cult
        let kind = if params.cosmological_density > 0.6 && rng.gen_bool(0.4) {
            if rng.gen_bool(0.5) {
                InstitutionKind::SecretSociety
            } else {
                InstitutionKind::Cult
            }
        } else {
            let all_kinds = [
                InstitutionKind::Guild,
                InstitutionKind::Government,
                InstitutionKind::Cult,
                InstitutionKind::MercenaryCompany,
                InstitutionKind::RegulatoryBody,
                InstitutionKind::SecretSociety,
            ];
            all_kinds[rng.gen_range(0..all_kinds.len())].clone()
        };
        let name = name_gen::generate_institution_name(&kind, phonemes, people_id, rng);
        let charter = name_gen::generate_charter(&kind, rng);
        let actual_function = name_gen::generate_actual_function(&kind, rng);
        let doctrine = name_gen::generate_doctrines(&kind, rng);

        // Pick a settlement as base of operations
        let settlement_idx = rng.gen_range(0..settlements.len());
        let base = &settlements[settlement_idx];
        let territory = vec![(base.x as u32, base.y as u32)];

        // Assign 3-8 members from matching people (or any if not enough)
        let member_count = rng.gen_range(3..=8);
        let mut member_ids = Vec::new();

        // Prefer agents of the same people who aren't already in 2 institutions
        let mut candidates: Vec<usize> = agents.iter()
            .enumerate()
            .filter(|(_, a)| a.alive && a.people_id == people_id && a.institution_ids.len() < 2)
            .map(|(idx, _)| idx)
            .collect();

        // If not enough from same people, add any unaffiliated agents
        if candidates.len() < member_count {
            let extras: Vec<usize> = agents.iter()
                .enumerate()
                .filter(|(idx, a)| {
                    a.alive && a.institution_ids.is_empty() && !candidates.contains(idx)
                })
                .map(|(idx, _)| idx)
                .collect();
            candidates.extend(extras);
        }

        // Shuffle candidates and take up to member_count
        for _ in 0..candidates.len().min(100) {
            let a = rng.gen_range(0..candidates.len());
            let b = rng.gen_range(0..candidates.len());
            candidates.swap(a, b);
        }

        for &ci in candidates.iter().take(member_count) {
            member_ids.push(agents[ci].id);
            agents[ci].institution_ids.push(i as u64);
        }

        let chronicle_entry = format!("Founded at {}. Charter: {}", base.name, charter);

        institutions.push(Institution {
            id: i as u64,
            name,
            kind,
            charter,
            actual_function,
            power: rng.gen_range(10..=30),
            doctrine,
            member_ids,
            territory,
            founded_tick: 0,
            relationships: HashMap::new(),
            chronicle: vec![chronicle_entry],
            people_id,
            alive: true,
        });
    }

    institutions
}

// ---------------------------------------------------------------------------
// Peoples generation
// ---------------------------------------------------------------------------

/// Generate 3-4 peoples with phoneme-based names and terrain preferences.
fn generate_peoples(
    rng: &mut StdRng,
    phonemes: &[name_gen::PhonemeSet],
) -> Vec<People> {
    let count = rng.gen_range(3..=4);
    let mut peoples = Vec::new();

    let all_terrains = [
        Terrain::Plains,
        Terrain::Hills,
        Terrain::Forest,
        Terrain::Desert,
        Terrain::Mountains,
    ];

    for i in 0..count {
        // Assign each people a distinct phoneme set (wraps around if more peoples than sets)
        let phoneme_set = i % phonemes.len();
        let set = &phonemes[phoneme_set];

        // People name: 2-syllable name from their phoneme set
        let name = name_gen::generate_name_part_public(set, 2, 2, rng);

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
            phoneme_set,
        });
    }

    peoples
}
