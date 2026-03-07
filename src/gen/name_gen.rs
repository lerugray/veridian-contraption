// Phoneme-based name generation and epithet system.

use rand::rngs::StdRng;
use rand::Rng;
use serde::Deserialize;

use crate::sim::event::EventType;
use crate::sim::institution::InstitutionKind;

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
    generate_institution_name_with_weirdness(kind, phonemes, phoneme_set, 0.5, rng)
}

/// Generate an institution name with weirdness coefficient influencing absurdity.
/// High weirdness produces names that suggest impossible functions.
pub fn generate_institution_name_with_weirdness(
    kind: &InstitutionKind,
    phonemes: &[PhonemeSet],
    phoneme_set: usize,
    weirdness: f32,
    rng: &mut StdRng,
) -> String {
    // At high weirdness, chance of an impossible-function name
    if weirdness > 0.7 && rng.gen_bool(0.35) {
        return generate_impossible_institution_name(kind, phonemes, phoneme_set, rng);
    }

    let prefix = match kind {
        InstitutionKind::Guild => pick(
            &["The Guild of", "The Confraternity of the", "The Society of", "The Fellowship of"],
            rng,
        ),
        InstitutionKind::Government => pick(
            &["The Bureau of", "The Office of", "The Ministry of", "The Department of"],
            rng,
        ),
        InstitutionKind::Cult => pick(
            &["The Order of the", "The Congregation of", "The Synod of", "The Temple of the"],
            rng,
        ),
        InstitutionKind::MercenaryCompany => pick(
            &["The Company of the", "The Legion of", "The Guard of", "The Defenders of"],
            rng,
        ),
        InstitutionKind::RegulatoryBody => pick(
            &["The Commission of", "The Board of", "The Registry of", "The Bureau of"],
            rng,
        ),
        InstitutionKind::SecretSociety => pick(
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

/// Generate institution names that suggest impossible functions.
/// These are Kafkaesque: the impossibility is presented as mundane.
fn generate_impossible_institution_name(
    kind: &InstitutionKind,
    phonemes: &[PhonemeSet],
    phoneme_set: usize,
    rng: &mut StdRng,
) -> String {
    let set = &phonemes[phoneme_set % phonemes.len()];
    let cultural_word = generate_name_part(set, 1, 2, rng);

    let impossible_names: &[&str] = match kind {
        InstitutionKind::Guild => &[
            "The Guild of Retroactive Carpentry",
            "The Confraternity of the Already Concluded",
            "The Society for the Regulation of Things That Have Not Yet Occurred",
            "The Fellowship of Anticipated Regret",
            "The Guild of Procedures That Reference Themselves",
        ],
        InstitutionKind::Government => &[
            "The Bureau of Retroactive Nomenclature",
            "The Office of Locations That Have Since Moved",
            "The Ministry of the Previous Administration's Oversight",
            "The Department of Answering Questions with Questions",
            "The Bureau of Determining What Is and Is Not a Bureau",
        ],
        InstitutionKind::Cult => &[
            "The Order of the Unforthcoming Revelation",
            "The Congregation of What Was Meant to Happen Next",
            "The Synod of the Doctrine That Precedes Itself",
            "The Temple of the Conclusion That Was Always There",
            "The Order of Events That Occurred in the Wrong Sequence",
        ],
        InstitutionKind::MercenaryCompany => &[
            "The Company of the Preemptive Surrender",
            "The Legion of Those Who Arrived After",
            "The Guard of the Door That Doesn't Exist",
            "The Defenders of the Indefensible (Administrative Division)",
            "The Company of Contractual Obligations to No One in Particular",
        ],
        InstitutionKind::RegulatoryBody => &[
            "The Commission on Determining What Commissions Are",
            "The Board of Reviewing the Board's Previous Reviews",
            "The Registry of Things the Registry Has Declined to Register",
            "The Bureau of Provisional Finality",
            "The Commission for the Oversight of Oversight",
        ],
        InstitutionKind::SecretSociety => &[
            "The Lodge of Those Who Know What the Lodge Is For",
            "The Fellowship of the Secret That Turned Out to Be Procedural",
            "The Circle of Knowing Glances and Uncomfortable Silences",
            "The Hidden Order of the Openly Known",
            "The Society for the Preservation of What Cannot Be Named",
        ],
    };

    let base = pick(impossible_names, rng);
    // Sometimes append cultural name
    if rng.gen_bool(0.3) {
        format!("{} of {}", base, cultural_word)
    } else {
        base.to_string()
    }
}

/// Generate a charter (stated purpose) for an institution.
pub fn generate_charter(kind: &InstitutionKind, rng: &mut StdRng) -> String {
    match kind {
        InstitutionKind::Guild => pick(&[
            "The regulation and advancement of a particular trade",
            "The mutual protection of its members' commercial interests",
            "The standardization of weights, measures, and contractual obligations",
            "The preservation of craft secrets and the training of apprentices",
        ], rng).to_string(),
        InstitutionKind::Government => pick(&[
            "The orderly administration of territorial affairs",
            "The collection of revenues and the enforcement of civic obligations",
            "The maintenance of public order and the resolution of disputes",
            "The provisional governance of contested or recently acquired territories",
        ], rng).to_string(),
        InstitutionKind::Cult => pick(&[
            "The veneration of a principle that defies conventional categorization",
            "The interpretation and dissemination of received cosmological truths",
            "The preparation for an event that has been anticipated for some time",
            "The maintenance of ritual obligations whose origin is no longer documented",
        ], rng).to_string(),
        InstitutionKind::MercenaryCompany => pick(&[
            "The provision of armed services to those who can afford them",
            "The defense of interests that the regular authorities decline to protect",
            "The enforcement of debts and the recovery of disputed property",
            "The conduct of military operations on a contractual basis",
        ], rng).to_string(),
        InstitutionKind::RegulatoryBody => pick(&[
            "The oversight and certification of activities deemed consequential",
            "The enforcement of standards that were established under unclear authority",
            "The licensing of practices that require formal permission to perform",
            "The investigation of irregularities in administrative proceedings",
        ], rng).to_string(),
        InstitutionKind::SecretSociety => pick(&[
            "Purposes that are disclosed only to members of sufficient standing",
            "The advancement of an agenda that public institutions have declined to pursue",
            "The preservation of knowledge that has been officially suppressed",
            "Activities that are best conducted without general awareness",
        ], rng).to_string(),
    }
}

/// Generate an "actual function" that may diverge from the charter.
pub fn generate_actual_function(kind: &InstitutionKind, rng: &mut StdRng) -> String {
    let diverged = rng.gen_bool(0.4); // 40% chance the actual function differs
    if !diverged {
        return generate_charter(kind, rng);
    }
    pick(&[
        "Primarily concerned with suppressing a rival organization",
        "Mostly occupied with internal procedural disputes",
        "Functioning as a social club for its senior members",
        "Engaged in the accumulation of administrative influence",
        "Dedicated to perpetuating its own existence",
        "Operating as an informal intelligence network",
        "Focused on controlling access to a particular resource",
        "Serving as a vehicle for the ambitions of its leadership",
    ], rng).to_string()
}

/// Generate 2-4 doctrinal positions for an institution.
pub fn generate_doctrines(kind: &InstitutionKind, rng: &mut StdRng) -> Vec<String> {
    let count = rng.gen_range(2..=4);
    let pool: &[&str] = match kind {
        InstitutionKind::Guild => &[
            "Quality must be maintained at the expense of efficiency",
            "Members shall not undercut one another's prices",
            "Trade secrets are held in common trust",
            "Apprenticeship is the only legitimate path to mastery",
            "External competition is to be resisted by all available means",
            "Innovation is acceptable only when properly documented",
        ],
        InstitutionKind::Government => &[
            "Authority derives from documented precedent",
            "Taxation is a natural consequence of territorial habitation",
            "Disputes are resolved through established procedure, not force",
            "Census records are sacred and inviolable",
            "All territorial claims require proper documentation",
            "Administrative positions shall be filled by appointment, not election",
        ],
        InstitutionKind::Cult => &[
            "The fundamental nature of reality is other than it appears",
            "Certain obligations transcend the authority of temporal institutions",
            "Revelation is ongoing and may contradict earlier revelation",
            "The uninitiated are not qualified to assess doctrinal matters",
            "Ritual observance is non-negotiable",
            "The anticipated event approaches on a schedule known only to the worthy",
        ],
        InstitutionKind::MercenaryCompany => &[
            "A contract, once accepted, is fulfilled regardless of circumstance",
            "Payment is required in advance of services rendered",
            "The company does not take sides in disputes; it takes fees",
            "Loyalty to the company supersedes all other obligations",
            "Retreat is a tactical option, not a moral failing",
            "Former enemies are potential future clients",
        ],
        InstitutionKind::RegulatoryBody => &[
            "All activities within our jurisdiction require formal approval",
            "Compliance is not optional",
            "Ambiguity in the regulations shall be resolved in favor of oversight",
            "Appeals are permitted but rarely successful",
            "Our authority is derived from necessity, not popularity",
            "Inspection schedules are confidential for operational reasons",
        ],
        InstitutionKind::SecretSociety => &[
            "What is known to us is not to be shared with outsiders",
            "Membership is by invitation only",
            "The true purpose of the organization is revealed in stages",
            "Public institutions serve a function but lack essential knowledge",
            "Discretion is the highest virtue",
            "The symbols are not merely decorative",
        ],
    };

    let mut doctrines = Vec::new();
    let mut used = vec![false; pool.len()];
    while doctrines.len() < count && doctrines.len() < pool.len() {
        let idx = rng.gen_range(0..pool.len());
        if !used[idx] {
            used[idx] = true;
            doctrines.push(pool[idx].to_string());
        }
    }
    doctrines
}

// ---------------------------------------------------------------------------
// Epithet system
// ---------------------------------------------------------------------------

/// Generate an epithet with weirdness-influenced oxymoronic variants.
pub fn generate_epithet_with_weirdness(
    event_type: &EventType,
    location_name: Option<&str>,
    weirdness: f32,
    rng: &mut StdRng,
) -> String {
    // High weirdness: chance of an oxymoronic epithet
    if weirdness > 0.65 && rng.gen_bool(0.25) {
        return pick(&[
            "the Provisionally Permanent",
            "the Accurately Mistaken",
            "the Officially Unofficial",
            "the Reliably Absent",
            "the Conspicuously Unnoticed",
            "the Voluntarily Compelled",
            "the Precisely Approximate",
            "the Famously Anonymous",
            "the Thoroughly Superficial",
            "the Deliberately Accidental",
            "the Publicly Confidential",
            "the Recently Eternal",
            "the Formally Informal",
            "the Predictably Unprecedented",
            "the Conclusively Inconclusive",
            "the Locally Everywhere",
            "the Temporarily Permanent",
            "the Enthusiastically Indifferent",
        ], rng).to_string();
    }

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
        EventType::InstitutionFounded => {
            match rng.gen_range(0..4) {
                0 => "the Founder".to_string(),
                1 => "the Charter-Bearer".to_string(),
                2 => "Who Established the Precedent".to_string(),
                _ => "the Incorporator".to_string(),
            }
        }
        EventType::MemberJoined => {
            match rng.gen_range(0..4) {
                0 => "the Newly Affiliated".to_string(),
                1 => "the Initiated".to_string(),
                2 => "the Enrolled".to_string(),
                _ => "of Recent Membership".to_string(),
            }
        }
        EventType::MemberExpelled => {
            match rng.gen_range(0..4) {
                0 => "the Expelled".to_string(),
                1 => "the Defenestrated".to_string(),
                2 => "Whose Membership Was Revoked".to_string(),
                _ => "the Formerly Affiliated".to_string(),
            }
        }
        EventType::MemberDeparted => {
            match rng.gen_range(0..4) {
                0 => "the Disaffiliated".to_string(),
                1 => "Who Resigned".to_string(),
                2 => "the Voluntarily Departed".to_string(),
                _ => "of No Current Affiliation".to_string(),
            }
        }
        EventType::ArtifactAcquired => {
            match rng.gen_range(0..5) {
                0 => "the Acquisitor".to_string(),
                1 => "the Bearer of Relics".to_string(),
                2 => format!("Who Retrieved Something from {}", loc),
                3 => "the Collector".to_string(),
                _ => "of Dubious Salvage".to_string(),
            }
        }
        EventType::ArtifactDelivered => {
            match rng.gen_range(0..4) {
                0 => "the Courier".to_string(),
                1 => format!("Who Delivered to {}", loc),
                2 => "the Returning".to_string(),
                _ => "the Bearer of Things Owed".to_string(),
            }
        }
        EventType::AdventurerDiedInSite => {
            match rng.gen_range(0..4) {
                0 => "the Ill-Advised".to_string(),
                1 => format!("Who Fell in {}", loc),
                2 => "Whose File Was Closed Prematurely".to_string(),
                _ => "the Architecturally Defeated".to_string(),
            }
        }
        EventType::CombatOccurred => {
            match rng.gen_range(0..8) {
                0 => "the Undisputed".to_string(),
                1 => format!("Who Prevailed at {}", loc),
                2 => "the Convincing".to_string(),
                3 => "Whose Arguments Were Physical".to_string(),
                4 => "the Decisive".to_string(),
                5 => "the Uncontested".to_string(),
                6 => "Who Resolved the Matter".to_string(),
                _ => "the Persuasive".to_string(),
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

/// Generate an era name for the World Annals.
pub fn generate_era_name(era_number: u32, rng: &mut StdRng) -> String {
    let ordinals = [
        "First", "Second", "Third", "Fourth", "Fifth",
        "Sixth", "Seventh", "Eighth", "Ninth", "Tenth",
        "Eleventh", "Twelfth", "Thirteenth", "Fourteenth", "Fifteenth",
    ];
    let ordinal = if (era_number as usize) < ordinals.len() {
        ordinals[era_number as usize].to_string()
    } else {
        format!("{}th", era_number + 1)
    };

    let era_nouns = [
        "Dispensation", "Reckoning", "Arrangement", "Accounting",
        "Adjustment", "Tenure", "Administration", "Catalogue",
        "Enumeration", "Correspondence", "Proceeding", "Interval",
        "Incumbency", "Ledger", "Codification", "Registry",
    ];
    let noun = era_nouns[rng.gen_range(0..era_nouns.len())];
    format!("The {} {}", ordinal, noun)
}
