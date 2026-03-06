// Phoneme-based name generation and epithet system.

use rand::rngs::StdRng;
use rand::Rng;
use serde::Deserialize;

use crate::sim::event::EventType;

/// A phoneme set defining the sound palette for a cultural group.
#[derive(Debug, Clone, Deserialize)]
pub struct PhonemeSet {
    pub name_flavor: String,
    pub onset: Vec<String>,
    pub nucleus: Vec<String>,
    pub coda: Vec<String>,
    pub syllable_patterns: Vec<String>,
    pub settlement_suffixes: Vec<String>,
    #[serde(default)]
    pub compound: bool,
}

/// Types of institutions (placeholder for Phase 2 — will move to sim/institution.rs).
#[derive(Debug, Clone)]
pub enum InstitutionKind {
    Guild,
    Government,
    Religious,
    Military,
    Regulatory,
    Secret,
}

/// Load phoneme data from the embedded JSON file.
pub fn load_phoneme_data() -> Vec<PhonemeSet> {
    let json = include_str!("../../data/phonemes.json");
    serde_json::from_str(json).expect("Failed to parse phonemes.json")
}

fn pick<'a>(options: &[&'a str], rng: &mut StdRng) -> &'a str {
    options[rng.gen_range(0..options.len())]
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

/// Generate a single syllable from a phoneme set and pattern string.
/// Pattern chars: C = consonant (onset before V, coda after V), V = vowel.
fn generate_syllable(set: &PhonemeSet, pattern: &str, rng: &mut StdRng) -> String {
    let mut result = String::new();
    let chars: Vec<char> = pattern.chars().collect();
    let v_pos = chars.iter().position(|&c| c == 'V').unwrap_or(chars.len());

    for (i, &ch) in chars.iter().enumerate() {
        match ch {
            'C' => {
                if i < v_pos {
                    result.push_str(&set.onset[rng.gen_range(0..set.onset.len())]);
                } else {
                    result.push_str(&set.coda[rng.gen_range(0..set.coda.len())]);
                }
            }
            'V' => {
                result.push_str(&set.nucleus[rng.gen_range(0..set.nucleus.len())]);
            }
            _ => {}
        }
    }
    result
}

/// Generate a single name-part (one word) with the given syllable count range.
fn generate_name_part(set: &PhonemeSet, min_syl: usize, max_syl: usize, rng: &mut StdRng) -> String {
    let count = if min_syl == max_syl {
        min_syl
    } else {
        rng.gen_range(min_syl..=max_syl)
    };
    let mut name = String::new();
    for _ in 0..count {
        let pattern = &set.syllable_patterns[rng.gen_range(0..set.syllable_patterns.len())];
        name.push_str(&generate_syllable(set, pattern, rng));
    }
    capitalize(&name)
}

/// Public access to generate a single name part (used for people names).
pub fn generate_name_part_public(set: &PhonemeSet, min_syl: usize, max_syl: usize, rng: &mut StdRng) -> String {
    generate_name_part(set, min_syl, max_syl, rng)
}

/// Generate a personal name for an agent of the given phoneme set.
pub fn generate_personal_name(phonemes: &[PhonemeSet], phoneme_set: usize, rng: &mut StdRng) -> String {
    let idx = phoneme_set % phonemes.len();
    let set = &phonemes[idx];

    if set.compound {
        // Compound names: "First-Second"
        let p1 = generate_name_part(set, 1, 2, rng);
        let p2 = generate_name_part(set, 1, 2, rng);
        format!("{}-{}", p1, p2)
    } else {
        // Syllable counts vary by set character
        let (first_min, first_max, last_min, last_max) = match idx {
            0 => (1, 1, 1, 2), // Guttural: short, punchy
            1 => (2, 3, 2, 3), // Sibilant: flowing, longer
            2 => (2, 2, 2, 2), // Nasal: rhythmic pairs
            _ => (1, 2, 1, 2), // Default
        };
        let first = generate_name_part(set, first_min, first_max, rng);
        let last = generate_name_part(set, last_min, last_max, rng);
        format!("{} {}", first, last)
    }
}

/// Generate a settlement name for the given phoneme set.
pub fn generate_settlement_name(phonemes: &[PhonemeSet], phoneme_set: usize, rng: &mut StdRng) -> String {
    let idx = phoneme_set % phonemes.len();
    let set = &phonemes[idx];

    if set.compound || set.settlement_suffixes.is_empty() {
        // Compound settlement: "Name-Name"
        let p1 = generate_name_part(set, 1, 2, rng);
        let p2 = generate_name_part(set, 1, 1, rng);
        format!("{}-{}", p1, p2)
    } else {
        let syl_count = match idx {
            0 => rng.gen_range(1..=2),
            _ => 2,
        };
        let base = generate_name_part(set, syl_count, syl_count, rng).to_lowercase();
        let suffix = &set.settlement_suffixes[rng.gen_range(0..set.settlement_suffixes.len())];
        capitalize(&format!("{}{}", base, suffix))
    }
}

/// Generate a world name (grander, 2-3 syllables).
pub fn generate_world_name(phonemes: &[PhonemeSet], rng: &mut StdRng) -> String {
    let idx = rng.gen_range(0..phonemes.len());
    let set = &phonemes[idx];

    if set.compound {
        let p1 = generate_name_part(set, 1, 2, rng);
        let p2 = generate_name_part(set, 1, 2, rng);
        format!("{}-{}", p1, p2)
    } else {
        generate_name_part(set, 2, 3, rng)
    }
}

/// Generate an institution name (bureaucratic compound name).
pub fn generate_institution_name(
    kind: &InstitutionKind,
    phonemes: &[PhonemeSet],
    phoneme_set: usize,
    rng: &mut StdRng,
) -> String {
    let prefix = match kind {
        InstitutionKind::Guild => pick(
            &["The Guild of", "The Confraternity of the", "The Society of", "The Fellowship of"],
            rng,
        ),
        InstitutionKind::Government => pick(
            &["The Bureau of", "The Office of", "The Ministry of", "The Department of"],
            rng,
        ),
        InstitutionKind::Religious => pick(
            &["The Order of the", "The Congregation of", "The Synod of", "The Temple of the"],
            rng,
        ),
        InstitutionKind::Military => pick(
            &["The Company of the", "The Legion of", "The Guard of", "The Defenders of"],
            rng,
        ),
        InstitutionKind::Regulatory => pick(
            &["The Commission of", "The Board of", "The Registry of", "The Bureau of"],
            rng,
        ),
        InstitutionKind::Secret => pick(
            &["The Lodge of the", "The Fellowship of the", "The Circle of", "The Hidden"],
            rng,
        ),
    };

    let adjectives = [
        "Ossified", "Provisional", "Ambiguous", "Disputed", "Enumerated",
        "Persistent", "Lateral", "Undisclosed", "Archived", "Perpetual",
        "Contested", "Expedient", "Formal", "Insolvent", "Obscured",
        "Accumulated", "Procedural", "Consequential",
    ];

    let nouns = [
        "Scale", "Territories", "Debts", "Commerce", "Contracts",
        "Wool", "Archives", "Thresholds", "Obligations", "Appointments",
        "Registers", "Seals", "Keys", "Borders", "Permits",
        "Precedents", "Margins", "Reckoning",
    ];

    let adj = adjectives[rng.gen_range(0..adjectives.len())];
    let noun = nouns[rng.gen_range(0..nouns.len())];

    // Some names incorporate a cultural word from the people's phoneme set
    let set = &phonemes[phoneme_set % phonemes.len()];
    let cultural_word = generate_name_part(set, 1, 2, rng);

    match rng.gen_range(0..3) {
        0 => format!("{} {} {}", prefix, adj, noun),
        1 => format!("{} {} {} of {}", prefix, adj, noun, cultural_word),
        _ => format!("{} {}", prefix, cultural_word),
    }
}

// ---------------------------------------------------------------------------
// Epithet system
// ---------------------------------------------------------------------------

/// Generate an epithet based on a triggering event type and location.
pub fn generate_epithet(
    event_type: &EventType,
    location_name: Option<&str>,
    rng: &mut StdRng,
) -> String {
    let loc = location_name.unwrap_or("the Unregistered Reaches");

    match event_type {
        EventType::AgentArrived => {
            match rng.gen_range(0..6) {
                0 => format!("the Newly Registered at {}", loc),
                1 => "the Arrived".to_string(),
                2 => "the Uninvited".to_string(),
                3 => format!("Who Came to {}", loc),
                4 => "the Present".to_string(),
                _ => "the Visitor".to_string(),
            }
        }
        EventType::AgentDeparted => {
            match rng.gen_range(0..6) {
                0 => "the Itinerant".to_string(),
                1 => "the Departed".to_string(),
                2 => format!("Who Left {}", loc),
                3 => "the Unfixed".to_string(),
                4 => "the Relocator".to_string(),
                _ => "the Formerly Present".to_string(),
            }
        }
        EventType::AgeEvent => {
            match rng.gen_range(0..6) {
                0 => "the Persistent".to_string(),
                1 => "of Considerable Tenure".to_string(),
                2 => "the Long-Enduring".to_string(),
                3 => "Who Was Counted".to_string(),
                4 => "the Duly Recorded".to_string(),
                _ => "the Venerable".to_string(),
            }
        }
        EventType::AgentDied => {
            match rng.gen_range(0..4) {
                0 => "the Late".to_string(),
                1 => "the Formerly Living".to_string(),
                2 => "Whose File Was Closed".to_string(),
                _ => "the Concluded".to_string(),
            }
        }
        _ => {
            // General epithets for other event types
            match rng.gen_range(0..10) {
                0 => "the Appointed".to_string(),
                1 => "the Provisional".to_string(),
                2 => "the Noted".to_string(),
                3 => "the Enumerated".to_string(),
                4 => "the Twice-Mentioned".to_string(),
                5 => "the Disputed".to_string(),
                6 => "the Formerly Relevant".to_string(),
                7 => format!("of {}", loc),
                8 => "the Overlooked".to_string(),
                _ => "the Documented".to_string(),
            }
        }
    }
}
