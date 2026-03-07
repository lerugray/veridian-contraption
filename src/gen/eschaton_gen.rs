use rand::rngs::StdRng;
use rand::Rng;

use crate::sim::eschaton::EschatonType;
use crate::sim::event::{Event, EventType};
use crate::sim::institution::{Institution, InstitutionKind, InstitutionRelationship};
use crate::sim::world::{NarrativeRegister, Terrain, Settlement, SettlementSize, MAP_WIDTH, MAP_HEIGHT};
use crate::sim::agent::{Agent, Disposition};
use crate::gen::name_gen;

/// Generate the sequence of log events describing an Eschaton.
/// Returns 8-12 events that narrate what is happening.
pub fn generate_eschaton_prose(
    eschaton_type: &EschatonType,
    tick: u64,
    register: NarrativeRegister,
    _weirdness: f32,
    rng: &mut StdRng,
) -> Vec<Event> {
    let mut events = Vec::new();

    // Opening announcement — always the same ominous pattern
    events.push(Event {
        tick,
        event_type: EventType::EschatonFired,
        subject_id: None,
        location: None,
        description: format!(
            "{}. The registrars have been advised to step back from their desks.",
            eschaton_type.label().to_uppercase()
        ),
    });

    // Type-specific narration
    match eschaton_type {
        EschatonType::TheReckoningOfDebts => {
            let lines = reckoning_prose(register, rng);
            for line in lines {
                events.push(Event {
                    tick,
                    event_type: EventType::EschatonFired,
                    subject_id: None,
                    location: None,
                    description: line,
                });
            }
        }
        EschatonType::TheTaxonomicCorrection => {
            let lines = taxonomic_prose(register, rng);
            for line in lines {
                events.push(Event {
                    tick,
                    event_type: EventType::EschatonFired,
                    subject_id: None,
                    location: None,
                    description: line,
                });
            }
        }
        EschatonType::TheAdministrativeSingularity => {
            let lines = singularity_prose(register, rng);
            for line in lines {
                events.push(Event {
                    tick,
                    event_type: EventType::EschatonFired,
                    subject_id: None,
                    location: None,
                    description: line,
                });
            }
        }
        EschatonType::TheGeologicalArgument => {
            let lines = geological_prose(register, rng);
            for line in lines {
                events.push(Event {
                    tick,
                    event_type: EventType::EschatonFired,
                    subject_id: None,
                    location: None,
                    description: line,
                });
            }
        }
        EschatonType::TheDoctrinalCascade => {
            let lines = doctrinal_prose(register, rng);
            for line in lines {
                events.push(Event {
                    tick,
                    event_type: EventType::EschatonFired,
                    subject_id: None,
                    location: None,
                    description: line,
                });
            }
        }
        EschatonType::TheArrivalOfSomethingOwed => {
            let lines = arrival_prose(register, rng);
            for line in lines {
                events.push(Event {
                    tick,
                    event_type: EventType::EschatonFired,
                    subject_id: None,
                    location: None,
                    description: line,
                });
            }
        }
    }

    // Closing event
    events.push(Event {
        tick,
        event_type: EventType::EschatonFired,
        subject_id: None,
        location: None,
        description: "The Eschaton has concluded. A new era begins. The filing cabinets have been requisitioned.".to_string(),
    });

    events
}

fn reckoning_prose(register: NarrativeRegister, rng: &mut StdRng) -> Vec<String> {
    let mut lines = Vec::new();
    let pool = match register {
        NarrativeRegister::Bureaucratic => vec![
            "All outstanding inter-institutional disputes have been marked 'RESOLVED (CATASTROPHICALLY)' in the central ledger.",
            "The Office of Adjudication reports that every pending claim has been simultaneously settled. The terms are unclear.",
            "Institutional boundaries have been declared void. The cartographic department has requested additional ink.",
            "Several organizations have discovered they were, in fact, the same organization. Others have discovered they were not.",
            "The accumulated weight of unresolved grievances has exceeded the structural capacity of the political landscape.",
            "All debts have been called in at once. The resulting cascade of obligation has inverted several hierarchies.",
            "The registry of disputes now reads, in its entirety: 'Concluded.' The registrar has requested leave.",
        ],
        NarrativeRegister::Ominous => vec![
            "Every debt has come due at once, as was always going to happen.",
            "The institutional order has collapsed under the weight of its own contradictions. This was overdue.",
            "All that was owed has been collected. The collectors did not explain their methods.",
            "The political landscape has been razed to its foundations. Something is already growing in the rubble.",
            "Every alliance and every enmity has resolved into a single, clarifying act of mutual annihilation.",
            "The ledgers are balanced. The cost of balancing them has not yet been tallied.",
            "What was borrowed has been returned, violently and without ceremony.",
        ],
        _ => vec![
            "All institutional disputes have resolved simultaneously. The results are not what anyone expected.",
            "The accumulated tension between organizations has discharged in a single, comprehensive rearrangement.",
            "Every pending grievance, claim, and counter-claim has been settled in one convulsive adjustment.",
            "The political order has undergone a complete restructuring. Former allies are now adversaries, and vice versa.",
            "All debts, obligations, and treaties have been voided and redrawn. The new terms are incomprehensible.",
            "The institutional landscape has been leveled. What emerges from the wreckage remains to be seen.",
            "Every organization has simultaneously discovered the true cost of its outstanding commitments.",
        ],
    };
    let count = rng.gen_range(7..=9);
    let mut used = Vec::new();
    for _ in 0..count {
        let idx = rng.gen_range(0..pool.len());
        if !used.contains(&idx) {
            lines.push(pool[idx].to_string());
            used.push(idx);
        }
    }
    lines
}

fn taxonomic_prose(register: NarrativeRegister, rng: &mut StdRng) -> Vec<String> {
    let mut lines = Vec::new();
    let pool = match register {
        NarrativeRegister::Clinical => vec![
            "A comprehensive reclassification of all registered persons has been initiated without warning.",
            "All existing epithets have been revoked pending taxonomic review.",
            "The census bureau reports that several settlements no longer correspond to their registered names.",
            "Population categories have been reassigned according to criteria that were not publicly disclosed.",
            "The official record now reflects a reality that differs, in certain respects, from the previous version.",
            "Several peoples have been reclassified. Their representatives were not consulted.",
            "The taxonomic authority has issued corrections to the fundamental registry. All previous entries are void.",
        ],
        _ => vec![
            "The world has decided that everyone's title was wrong. New epithets are being assigned by committee.",
            "All peoples have been reclassified. The new categories bear little resemblance to the old ones.",
            "The census has been corrected to reflect a reality that no one remembers agreeing to.",
            "Settlements have been renamed. The residents have not been informed of the reasons.",
            "Every registered name and title has been revoked. The replacement process has already begun.",
            "The taxonomic authorities have determined that the previous classification system was, in its entirety, provisional.",
            "A correction has been applied to the fundamental categories of existence. Filing has been adjusted accordingly.",
        ],
    };
    let count = rng.gen_range(7..=9);
    let mut used = Vec::new();
    for _ in 0..count {
        let idx = rng.gen_range(0..pool.len());
        if !used.contains(&idx) {
            lines.push(pool[idx].to_string());
            used.push(idx);
        }
    }
    lines
}

fn singularity_prose(register: NarrativeRegister, rng: &mut StdRng) -> Vec<String> {
    let mut lines = Vec::new();
    let pool = match register {
        NarrativeRegister::Bureaucratic => vec![
            "All institutions have merged into a single entity of staggering administrative complexity.",
            "The unified body has already begun to issue contradictory directives to itself.",
            "Seventeen sub-committees have formed to manage the merger. Each claims primacy.",
            "The combined charter is four hundred pages long and internally contradictory on page three.",
            "The singular institution briefly achieved total administrative control of all affairs, everywhere.",
            "It then collapsed. The collapse took approximately as long as the merger.",
            "The resulting successor bodies are already disputing the inheritance.",
            "A committee has been formed to determine how many committees now exist. Its findings are pending.",
            "The unified institution's first and only act was to file a complaint against itself.",
        ],
        _ => vec![
            "Every organization has merged into one. The resulting entity is already tearing itself apart.",
            "For a single moment, all administrative power was concentrated in one body. Then it fractured.",
            "The singular institution existed for precisely long enough to realize it could not possibly function.",
            "The bureaucratic singularity has collapsed into a bureaucratic plurality. The plural is contested.",
            "All charters have been combined. The resulting document is logically impossible.",
            "The unified body has schismed into more organizations than originally existed.",
            "Administrative control was briefly total and immediately unsustainable.",
            "The merger produced a structure so complex it could only be understood by the structure itself, which declined.",
            "Seventeen successor bodies have emerged from the wreckage. Each claims to be the original.",
        ],
    };
    let count = rng.gen_range(8..=9);
    let mut used = Vec::new();
    for _ in 0..count {
        let idx = rng.gen_range(0..pool.len());
        if !used.contains(&idx) {
            lines.push(pool[idx].to_string());
            used.push(idx);
        }
    }
    lines
}

fn geological_prose(register: NarrativeRegister, rng: &mut StdRng) -> Vec<String> {
    let mut lines = Vec::new();
    let pool = match register {
        NarrativeRegister::Ominous => vec![
            "The ground has expressed an opinion. It disagrees with the current arrangement.",
            "Mountains have appeared where there were none. Others have declined to continue existing.",
            "The coastline has revised itself without consulting the harbormasters.",
            "Several settlements have discovered they are now located in terrain they did not previously occupy.",
            "Rivers have changed course. The rivers have not explained their reasoning.",
            "The geological record has been amended. The amendment is retroactive.",
            "The land has rearranged itself in a manner that suggests intent, though no intent has been formally claimed.",
            "Forests have migrated. The trees were not observed in transit.",
        ],
        _ => vec![
            "Geography has undergone a sudden and comprehensive revision.",
            "Several mountain ranges have relocated. Affected settlements are adjusting.",
            "The terrain has restructured itself. Cartographers have been dispatched.",
            "New settlements have appeared in places that were previously uninhabitable. They appear to have always been there.",
            "The world map no longer corresponds to the world. The map has been updated.",
            "Coastlines have shifted. The fish have adapted faster than the fishermen.",
            "The geological survey reports that the previous survey was, retroactively, inaccurate.",
            "Terrain that was previously mountainous is now plains. The mountains have not been located.",
        ],
    };
    let count = rng.gen_range(7..=9);
    let mut used = Vec::new();
    for _ in 0..count {
        let idx = rng.gen_range(0..pool.len());
        if !used.contains(&idx) {
            lines.push(pool[idx].to_string());
            used.push(idx);
        }
    }
    lines
}

fn doctrinal_prose(register: NarrativeRegister, rng: &mut StdRng) -> Vec<String> {
    let mut lines = Vec::new();
    let pool = match register {
        NarrativeRegister::Conspiratorial => vec![
            "An idea has propagated through every institution simultaneously. No one can identify its origin.",
            "Every organization has revised its founding doctrine. The revisions are suspiciously similar.",
            "Agent affiliations have become unstable. Loyalty is being recalculated.",
            "The new doctrine appears to have always existed. Records suggesting otherwise have been updated.",
            "Several institutions have splintered over their interpretation of the cascade. This was anticipated by no one.",
            "New organizations are forming around competing readings of the same foundational text.",
            "The doctrinal cascade has reached terminal velocity. Every belief is now provisional.",
            "Those who resisted the cascade have been absorbed by it. Those who embraced it have been transformed.",
        ],
        _ => vec![
            "A single idea has swept through every institution in the world, simultaneously.",
            "All founding doctrines have been revised. The revisions are comprehensive and irreversible.",
            "Agent loyalties are in flux. Many have discovered their institutions no longer believe what they believed.",
            "New institutions are forming from the doctrinal wreckage at an unprecedented rate.",
            "Every organization has either adopted the cascade, rejected it violently, or split trying to do both.",
            "The idea has no identifiable source. It appears to have emerged from the accumulated contradictions of all existing doctrines.",
            "Several agents have changed affiliation three times in rapid succession. They are not yet settled.",
            "The doctrinal landscape has been completely redrawn. Theologians are working overtime.",
        ],
    };
    let count = rng.gen_range(7..=9);
    let mut used = Vec::new();
    for _ in 0..count {
        let idx = rng.gen_range(0..pool.len());
        if !used.contains(&idx) {
            lines.push(pool[idx].to_string());
            used.push(idx);
        }
    }
    lines
}

fn arrival_prose(register: NarrativeRegister, rng: &mut StdRng) -> Vec<String> {
    let mut lines = Vec::new();
    let pool = match register {
        NarrativeRegister::Ominous => vec![
            "Something has arrived. It was expected, though no one can say by whom.",
            "The newcomers appeared without explanation. They seem to have been here before.",
            "They carry no credentials. Their purpose is not stated. Their influence is already measurable.",
            "The existing population has reacted with a mixture of deference and unease.",
            "Those of high paranoia recognized them immediately. Those of low paranoia are learning.",
            "The newcomers have established themselves in positions of quiet significance.",
            "Their origins are unclear. The registry has no record of their departure from anywhere.",
            "Something that was owed has been delivered. The delivery was not what was expected.",
        ],
        _ => vec![
            "A group of strangers has arrived. They appear to have no point of origin.",
            "The newcomers are integrating rapidly. Their qualifications are unverifiable but compelling.",
            "Existing agents are divided in their response. Some are welcoming. Some are locking their doors.",
            "The registry has been updated to include the newcomers. It is unclear who authorized this.",
            "They have begun accumulating influence with a speed that suggests prior preparation.",
            "The newcomers' stated purpose is vague. Their actual purpose appears to be everything.",
            "Several institutions have already recruited from among the arrivals. Competition for their loyalty is fierce.",
            "Something has arrived that the world appears to have been waiting for, whether it knew it or not.",
        ],
    };
    let count = rng.gen_range(7..=9);
    let mut used = Vec::new();
    for _ in 0..count {
        let idx = rng.gen_range(0..pool.len());
        if !used.contains(&idx) {
            lines.push(pool[idx].to_string());
            used.push(idx);
        }
    }
    lines
}

// ---------------------------------------------------------------------------
// Eschaton execution — applies world-altering effects
// ---------------------------------------------------------------------------

/// Execute the mechanical effects of a Reckoning of Debts.
/// Resolves all institutional disputes: merges some, dissolves others, inverts purposes.
pub fn execute_reckoning(
    institutions: &mut Vec<Institution>,
    agents: &mut Vec<Agent>,
    next_inst_id: &mut u64,
    phonemes: &[name_gen::PhonemeSet],
    weirdness: f32,
    rng: &mut StdRng,
) {
    let alive_count = institutions.iter().filter(|i| i.alive).count();
    if alive_count < 2 { return; }

    // Dissolve ~40% of institutions
    for inst in institutions.iter_mut() {
        if !inst.alive { continue; }
        if rng.gen_bool(0.4) {
            inst.alive = false;
            inst.chronicle.push("Dissolved in the Reckoning of Debts.".to_string());
            // Free members
            for &mid in &inst.member_ids {
                if let Some(agent) = agents.iter_mut().find(|a| a.id == mid) {
                    agent.institution_ids.retain(|&id| id != inst.id);
                    agent.current_goal = crate::sim::agent::Goal::Wander;
                }
            }
        }
    }

    // Invert the purpose of ~30% of survivors
    for inst in institutions.iter_mut() {
        if !inst.alive { continue; }
        if rng.gen_bool(0.3) {
            let old_charter = inst.charter.clone();
            inst.charter = format!("The explicit repudiation of: {}", old_charter);
            inst.actual_function = name_gen::generate_actual_function(&inst.kind, rng);
            inst.chronicle.push("Charter inverted during the Reckoning.".to_string());
        }
    }

    // Clear all relationships (disputes resolved)
    for inst in institutions.iter_mut() {
        if !inst.alive { continue; }
        inst.relationships.clear();
    }

    // Create 1-2 new institutions from the chaos
    let new_count = rng.gen_range(1..=2);
    for _ in 0..new_count {
        let kind = match rng.gen_range(0..6) {
            0 => InstitutionKind::Guild,
            1 => InstitutionKind::Government,
            2 => InstitutionKind::Cult,
            3 => InstitutionKind::MercenaryCompany,
            4 => InstitutionKind::RegulatoryBody,
            _ => InstitutionKind::SecretSociety,
        };
        let people_id = rng.gen_range(0..4usize);
        let name = name_gen::generate_institution_name_with_weirdness(&kind, phonemes, people_id, weirdness, rng);
        let id = *next_inst_id;
        *next_inst_id += 1;
        institutions.push(Institution {
            id,
            name,
            kind: kind.clone(),
            charter: name_gen::generate_charter(&kind, rng),
            actual_function: name_gen::generate_actual_function(&kind, rng),
            power: rng.gen_range(10..=30),
            doctrine: name_gen::generate_doctrines(&kind, rng),
            member_ids: Vec::new(),
            territory: Vec::new(),
            founded_tick: 0, // will be set by caller
            relationships: std::collections::HashMap::new(),
            chronicle: vec!["Founded in the aftermath of the Reckoning of Debts.".to_string()],
            people_id,
            alive: true,
        });
    }
}

/// Execute the Taxonomic Correction: revoke/reassign epithets, rename settlements, reclassify peoples.
pub fn execute_taxonomic_correction(
    agents: &mut Vec<Agent>,
    settlements: &mut Vec<Settlement>,
    phonemes: &[name_gen::PhonemeSet],
    rng: &mut StdRng,
) {
    // Revoke all epithets
    for agent in agents.iter_mut() {
        if !agent.alive { continue; }
        agent.epithets.clear();
        agent.last_epithet_tick = 0;
        // Assign one new epithet to each living agent
        let new_epithets = [
            "the Reclassified", "the Corrected", "the Reassigned", "the Revised",
            "the Formerly Known", "the Taxonomically Adjusted", "the Re-registered",
            "the Updated", "the Newly Designated", "the Provisionally Categorized",
        ];
        agent.epithets.push(new_epithets[rng.gen_range(0..new_epithets.len())].to_string());
    }

    // Rename ~40% of settlements
    for settlement in settlements.iter_mut() {
        if rng.gen_bool(0.4) {
            let set_idx = rng.gen_range(0..phonemes.len().max(1));
            settlement.name = name_gen::generate_settlement_name(phonemes, set_idx, rng);
        }
    }
}

/// Execute the Administrative Singularity: merge all institutions, then schism.
pub fn execute_singularity(
    institutions: &mut Vec<Institution>,
    agents: &mut Vec<Agent>,
    next_inst_id: &mut u64,
    phonemes: &[name_gen::PhonemeSet],
    weirdness: f32,
    rng: &mut StdRng,
) {
    // Collect all living members
    let mut all_members: Vec<u64> = Vec::new();
    for inst in institutions.iter_mut() {
        if !inst.alive { continue; }
        all_members.extend(&inst.member_ids);
        inst.alive = false;
        inst.chronicle.push("Absorbed into the Administrative Singularity.".to_string());
    }
    all_members.sort();
    all_members.dedup();

    // Clear all agent affiliations
    for agent in agents.iter_mut() {
        agent.institution_ids.clear();
    }

    // Create 3-5 successor bodies (more than the original count if possible)
    let successor_count = rng.gen_range(3..=5);
    let chunk_size = (all_members.len() / successor_count).max(1);

    for i in 0..successor_count {
        let kind = match rng.gen_range(0..6) {
            0 => InstitutionKind::Guild,
            1 => InstitutionKind::Government,
            2 => InstitutionKind::Cult,
            3 => InstitutionKind::MercenaryCompany,
            4 => InstitutionKind::RegulatoryBody,
            _ => InstitutionKind::SecretSociety,
        };
        let people_id = rng.gen_range(0..4usize);
        let name = name_gen::generate_institution_name_with_weirdness(&kind, phonemes, people_id, weirdness, rng);
        let id = *next_inst_id;
        *next_inst_id += 1;

        // Assign members from the pool
        let start = i * chunk_size;
        let end = if i == successor_count - 1 { all_members.len() } else { ((i + 1) * chunk_size).min(all_members.len()) };
        let members: Vec<u64> = if start < all_members.len() {
            all_members[start..end].to_vec()
        } else {
            Vec::new()
        };

        // Update agent affiliations
        for &mid in &members {
            if let Some(agent) = agents.iter_mut().find(|a| a.id == mid) {
                agent.institution_ids.push(id);
            }
        }

        // Make successor bodies rivals of each other
        institutions.push(Institution {
            id,
            name,
            kind: kind.clone(),
            charter: name_gen::generate_charter(&kind, rng),
            actual_function: name_gen::generate_actual_function(&kind, rng),
            power: rng.gen_range(5..=15),
            doctrine: name_gen::generate_doctrines(&kind, rng),
            member_ids: members,
            territory: Vec::new(),
            founded_tick: 0,
            relationships: std::collections::HashMap::new(),
            chronicle: vec![format!("Successor body #{} of the Administrative Singularity.", i + 1)],
            people_id,
            alive: true,
        });
    }

    // Make all successor bodies rivals of each other
    let successor_ids: Vec<u64> = institutions.iter()
        .filter(|i| i.alive)
        .map(|i| i.id)
        .collect();
    for inst in institutions.iter_mut() {
        if !inst.alive { continue; }
        for &other_id in &successor_ids {
            if other_id != inst.id {
                inst.relationships.insert(other_id, InstitutionRelationship::Rival);
            }
        }
    }
}

/// Execute the Geological Argument: reshape terrain, move/add/remove settlements.
pub fn execute_geological_argument(
    terrain: &mut Vec<Vec<Terrain>>,
    settlements: &mut Vec<Settlement>,
    phonemes: &[name_gen::PhonemeSet],
    rng: &mut StdRng,
) {
    // Shift ~20% of terrain tiles
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            if rng.gen_bool(0.2) {
                terrain[y][x] = match rng.gen_range(0..7) {
                    0 => Terrain::DeepWater,
                    1 => Terrain::ShallowWater,
                    2 => Terrain::Plains,
                    3 => Terrain::Hills,
                    4 => Terrain::Forest,
                    5 => Terrain::Mountains,
                    _ => Terrain::Desert,
                };
            }
        }
    }

    // Remove ~20% of settlements
    let remove_count = (settlements.len() as f32 * 0.2).ceil() as usize;
    for _ in 0..remove_count {
        if settlements.len() <= 2 { break; } // keep at least 2
        let idx = rng.gen_range(0..settlements.len());
        settlements.remove(idx);
    }

    // Move ~30% of remaining settlements to new valid locations
    for settlement in settlements.iter_mut() {
        if rng.gen_bool(0.3) {
            // Find a habitable tile
            for _ in 0..50 {
                let nx = rng.gen_range(1..MAP_WIDTH - 1);
                let ny = rng.gen_range(1..MAP_HEIGHT - 1);
                if terrain[ny][nx].is_habitable() {
                    settlement.x = nx;
                    settlement.y = ny;
                    break;
                }
            }
        }
    }

    // Add 1-3 new settlements
    let new_count = rng.gen_range(1..=3);
    for _ in 0..new_count {
        for _ in 0..100 {
            let nx = rng.gen_range(1..MAP_WIDTH - 1);
            let ny = rng.gen_range(1..MAP_HEIGHT - 1);
            if terrain[ny][nx].is_habitable() {
                let set_idx = rng.gen_range(0..phonemes.len().max(1));
                let name = name_gen::generate_settlement_name(phonemes, set_idx, rng);
                let size = match rng.gen_range(0..3) {
                    0 => SettlementSize::Hamlet,
                    1 => SettlementSize::Town,
                    _ => SettlementSize::City,
                };
                let floor = Some(crate::gen::dungeon_gen::generate_settlement_floor(&size, rng));
                settlements.push(Settlement { name, size, x: nx, y: ny, floor });
                break;
            }
        }
    }
}

/// Execute the Doctrinal Cascade: revise all doctrines, destabilize affiliations, create new institutions.
pub fn execute_doctrinal_cascade(
    institutions: &mut Vec<Institution>,
    agents: &mut Vec<Agent>,
    next_inst_id: &mut u64,
    phonemes: &[name_gen::PhonemeSet],
    weirdness: f32,
    rng: &mut StdRng,
) {
    // Every living institution revises all doctrines
    for inst in institutions.iter_mut() {
        if !inst.alive { continue; }
        inst.doctrine = name_gen::generate_doctrines(&inst.kind, rng);
        inst.chronicle.push("All doctrines revised during the Doctrinal Cascade.".to_string());
    }

    // ~30% of affiliated agents lose their affiliation
    for agent in agents.iter_mut() {
        if !agent.alive { continue; }
        if !agent.institution_ids.is_empty() && rng.gen_bool(0.3) {
            agent.institution_ids.clear();
            agent.current_goal = crate::sim::agent::Goal::Wander;
        }
    }

    // Remove departed members from institutions
    let agent_affiliations: std::collections::HashMap<u64, Vec<u64>> = agents.iter()
        .filter(|a| a.alive)
        .map(|a| (a.id, a.institution_ids.clone()))
        .collect();
    for inst in institutions.iter_mut() {
        if !inst.alive { continue; }
        inst.member_ids.retain(|mid| {
            agent_affiliations.get(mid).map_or(false, |ids| ids.contains(&inst.id))
        });
    }

    // Spawn 2-4 new institutions from the doctrinal chaos
    let new_count = rng.gen_range(2..=4);
    for _ in 0..new_count {
        let kind = match rng.gen_range(0..6) {
            0 => InstitutionKind::Guild,
            1 => InstitutionKind::Government,
            2 => InstitutionKind::Cult,
            3 => InstitutionKind::MercenaryCompany,
            4 => InstitutionKind::RegulatoryBody,
            _ => InstitutionKind::SecretSociety,
        };
        let people_id = rng.gen_range(0..4usize);
        let name = name_gen::generate_institution_name_with_weirdness(&kind, phonemes, people_id, weirdness, rng);
        let id = *next_inst_id;
        *next_inst_id += 1;
        institutions.push(Institution {
            id,
            name,
            kind: kind.clone(),
            charter: name_gen::generate_charter(&kind, rng),
            actual_function: name_gen::generate_actual_function(&kind, rng),
            power: rng.gen_range(5..=20),
            doctrine: name_gen::generate_doctrines(&kind, rng),
            member_ids: Vec::new(),
            territory: Vec::new(),
            founded_tick: 0,
            relationships: std::collections::HashMap::new(),
            chronicle: vec!["Founded during the Doctrinal Cascade.".to_string()],
            people_id,
            alive: true,
        });
    }
}

/// Execute the Arrival of Something Owed: spawn new agents with mysterious origins.
pub fn execute_arrival(
    agents: &mut Vec<Agent>,
    settlements: &[Settlement],
    peoples_count: usize,
    phonemes: &[name_gen::PhonemeSet],
    next_agent_id: u64,
    rng: &mut StdRng,
) -> u64 {
    let arrival_count = rng.gen_range(5..=12);
    let mut id = next_agent_id;

    for _ in 0..arrival_count {
        if settlements.is_empty() { break; }
        let settlement = &settlements[rng.gen_range(0..settlements.len())];
        let people_id = rng.gen_range(0..peoples_count.max(1));
        let set_idx = if people_id < phonemes.len() { people_id } else { 0 };
        let name = name_gen::generate_personal_name(phonemes, set_idx, rng);

        let disposition = Disposition {
            risk_tolerance: rng.gen_range(0.3..=0.9),
            ambition: rng.gen_range(0.5..=1.0),
            institutional_loyalty: rng.gen_range(0.2..=0.8),
            paranoia: rng.gen_range(0.3..=0.9),
        };

        let mysterious_epithets = [
            "the Arrived", "the Expected", "the Owed", "the Returned",
            "the Unannounced", "the Awaited", "the Delivered",
        ];

        let agent = Agent {
            id,
            name,
            people_id,
            x: settlement.x as u32,
            y: settlement.y as u32,
            health: 100,
            age: rng.gen_range(7300..=18250), // 20-50 years old
            disposition,
            current_goal: crate::sim::agent::Goal::Wander,
            chronicle: vec!["Appeared during the Arrival of Something Owed. Origin unknown.".to_string()],
            alive: true,
            epithets: vec![mysterious_epithets[rng.gen_range(0..mysterious_epithets.len())].to_string()],
            last_epithet_tick: 0,
            institution_ids: Vec::new(),
            is_adventurer: rng.gen_bool(0.3),
            held_artifacts: Vec::new(),
            relationships: Vec::new(),
            conversations: Vec::new(),
        };
        agents.push(agent);
        id += 1;
    }

    id // return the next available ID
}
