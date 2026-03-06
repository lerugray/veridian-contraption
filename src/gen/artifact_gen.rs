// Artifact generation — names, materials, properties, and initial placement.

use rand::rngs::StdRng;
use rand::Rng;

use crate::gen::name_gen;
use crate::sim::artifact::{Artifact, ArtifactKind, ArtifactLocation};

const MATERIALS: &[&str] = &[
    "iron", "bone", "obsidian", "parchment", "lead-lined tin",
    "petrified wood", "unidentified stone", "lacquered ceramic",
    "tarnished copper", "something that resembles glass but is not",
    "vellum of uncertain provenance", "a material the assayer declined to name",
    "cold-forged steel", "an alloy not listed in the standard registry",
    "wax-sealed clay", "a substance officially classified as 'pending'",
];

const PROPERTIES: &[&str] = &[
    "Warm to the touch regardless of ambient temperature",
    "Heavier than its dimensions would suggest",
    "Bears an inscription in a script no living scholar can read",
    "Emits a faint hum at intervals that correspond to no known schedule",
    "Causes mild administrative anxiety in those who handle it",
    "Cannot be accurately weighed on standard scales",
    "Its surface is covered in notations that appear to be a census of something",
    "Smells faintly of institutional correspondence",
    "Occasionally vibrates in the presence of unresolved contractual obligations",
    "Was catalogued three times by three different offices, each under a different name",
    "Produces a sound when struck that the Bureau of Acoustics refuses to classify",
    "Its shadow falls in a direction that does not correspond to the light source",
    "Is slightly damp, perpetually, regardless of storage conditions",
    "The serial number engraved on its base does not match any known registry",
    "Resists being filed under any standard taxonomic category",
    "Bears the seal of an institution that officially never existed",
    "Contains a cavity that is larger on the inside than the outside permits",
    "Is listed as 'missing' in records that predate its creation",
];

/// Generate 8-20 artifacts, distributed between sites and settlements.
pub fn generate_artifacts(
    site_count: usize,
    settlement_count: usize,
    settlement_names: &[String],
    site_names: &[String],
    institution_info: &[(u64, String)],
    phonemes: &[name_gen::PhonemeSet],
    rng: &mut StdRng,
) -> Vec<Artifact> {
    let count = rng.gen_range(8..=20);
    let mut artifacts = Vec::with_capacity(count);

    for i in 0..count {
        let kind = match rng.gen_range(0..8) {
            0 => ArtifactKind::Weapon,
            1 => ArtifactKind::Document,
            2 => ArtifactKind::Vessel,
            3 => ArtifactKind::Instrument,
            4 => ArtifactKind::Relic,
            5 => ArtifactKind::FormalWrit,
            6 => ArtifactKind::TaxonomicSpecimen,
            _ => ArtifactKind::KeyToSomething,
        };

        let material = MATERIALS[rng.gen_range(0..MATERIALS.len())].to_string();

        let name = generate_artifact_name(
            &kind,
            phonemes,
            settlement_names,
            institution_info,
            rng,
        );

        // 1-3 properties
        let prop_count = rng.gen_range(1..=3);
        let mut properties = Vec::new();
        let mut used = vec![false; PROPERTIES.len()];
        while properties.len() < prop_count {
            let idx = rng.gen_range(0..PROPERTIES.len());
            if !used[idx] {
                used[idx] = true;
                properties.push(PROPERTIES[idx].to_string());
            }
        }

        // Place: ~60% in sites, ~30% in settlements, ~10% lost
        let location = {
            let roll: f32 = rng.gen();
            if roll < 0.6 && site_count > 0 {
                ArtifactLocation::InSite(rng.gen_range(0..site_count))
            } else if roll < 0.9 && settlement_count > 0 {
                ArtifactLocation::InSettlement(rng.gen_range(0..settlement_count))
            } else {
                ArtifactLocation::Lost
            }
        };

        // Some artifacts are claimed by an institution
        let institutional_claim = if !institution_info.is_empty() && rng.gen_bool(0.3) {
            Some(institution_info[rng.gen_range(0..institution_info.len())].0)
        } else {
            None
        };

        // Initial history entry
        let history_entry = match &location {
            ArtifactLocation::InSite(idx) => {
                let site_label = site_names.get(*idx)
                    .map(|s| s.as_str())
                    .unwrap_or("an unnamed site");
                format!("Discovered at {}. Provenance uncertain.", site_label)
            }
            ArtifactLocation::InSettlement(idx) => {
                let sett_label = settlement_names.get(*idx)
                    .map(|s| s.as_str())
                    .unwrap_or("an unnamed settlement");
                format!("Held at {}. Ownership documented but not verified.", sett_label)
            }
            ArtifactLocation::Lost => {
                "Last known location is a matter of administrative dispute.".to_string()
            }
            ArtifactLocation::HeldByAgent(_) => {
                "Currently in private possession.".to_string()
            }
        };

        artifacts.push(Artifact {
            id: i as u64,
            name,
            kind,
            material,
            history: vec![history_entry],
            current_location: location,
            properties,
            institutional_claim,
        });
    }

    artifacts
}

/// Generate an evocative artifact name based on its kind.
fn generate_artifact_name(
    kind: &ArtifactKind,
    phonemes: &[name_gen::PhonemeSet],
    settlement_names: &[String],
    institution_info: &[(u64, String)],
    rng: &mut StdRng,
) -> String {
    let set = &phonemes[rng.gen_range(0..phonemes.len())];
    let cultural_word = name_gen::generate_name_part_public(set, 1, 2, rng);
    let personal_name = name_gen::generate_personal_name(phonemes, rng.gen_range(0..phonemes.len()), rng);

    // Pick a settlement name or institution name for context
    let place_name = if !settlement_names.is_empty() && rng.gen_bool(0.5) {
        settlement_names[rng.gen_range(0..settlement_names.len())].clone()
    } else {
        cultural_word.clone()
    };

    let inst_name = if !institution_info.is_empty() {
        let (_, ref name) = institution_info[rng.gen_range(0..institution_info.len())];
        name.clone()
    } else {
        format!("the {} Compact", cultural_word)
    };

    match kind {
        ArtifactKind::Weapon => {
            let weapon = pick_weapon_type(rng);
            match rng.gen_range(0..4) {
                0 => format!("The Blade of {} (Notarized)", personal_name),
                1 => format!("The {} of {}", weapon, cultural_word),
                2 => format!("{}'s {} — Jurisdiction Unclear", personal_name, weapon),
                _ => format!("The Confiscated {} of {}", weapon, place_name),
            }
        }
        ArtifactKind::Document => {
            match rng.gen_range(0..5) {
                0 => format!("The {} Register of the {} Compact", pick_ordinal(rng), place_name),
                1 => format!("A Deed of Transfer (Signatories Disputed) — {} Edition", cultural_word),
                2 => format!("The Unsigned Memorandum of {}", personal_name),
                3 => format!("Proceedings of the {} Inquiry, Volume {}", place_name, rng.gen_range(2..17)),
                _ => format!("The {} Codex (Contents Under Review)", cultural_word),
            }
        }
        ArtifactKind::Vessel => {
            match rng.gen_range(0..4) {
                0 => format!("A Vessel of Ambiguous Provenance (Contents Disputed)"),
                1 => format!("The Sealed Urn of {} — Do Not Open {}", cultural_word, pick_temporal_hedge(rng)),
                2 => format!("The {} Amphora (Classification Pending)", place_name),
                _ => format!("Container {}-{}: {} (Unlabelled)", rng.gen_range(3..99), rng.gen_range(1..9), cultural_word),
            }
        }
        ArtifactKind::Instrument => {
            match rng.gen_range(0..4) {
                0 => format!("The {} of {} (Calibration Uncertain)", pick_instrument(rng), personal_name),
                1 => format!("A {} Issued by {}", pick_instrument(rng), inst_name),
                2 => format!("The Standard {} of {} — Revised", pick_instrument(rng), place_name),
                _ => format!("{}'s Portable {} (Not to Scale)", personal_name, pick_instrument(rng)),
            }
        }
        ArtifactKind::Relic => {
            match rng.gen_range(0..4) {
                0 => format!("The {} Relic (Authenticity Contested)", cultural_word),
                1 => format!("A Fragment of the {} Accord", place_name),
                2 => format!("The Tooth of {} — Taxonomic Classification Withheld", personal_name),
                _ => format!("The {} Object (Purpose Officially Undetermined)", cultural_word),
            }
        }
        ArtifactKind::FormalWrit => {
            match rng.gen_range(0..5) {
                0 => format!("The Warrant of {}", personal_name),
                1 => format!("A Writ of Compulsion Issued to the Residents of {}", place_name),
                2 => format!("The Standing Order of {} (Never Rescinded)", inst_name),
                3 => format!("Letter of Marque — Bearer: {} (Expired)", personal_name),
                _ => format!("The {} Dispensation (Terms Subject to Revision)", cultural_word),
            }
        }
        ArtifactKind::TaxonomicSpecimen => {
            match rng.gen_range(0..4) {
                0 => format!("Specimen {}: {} (Classification Deferred)", rng.gen_range(1..999), cultural_word),
                1 => format!("The Preserved {} of the {} Region", pick_specimen_type(rng), place_name),
                2 => format!("A {} Collected Near {} Under Protest", pick_specimen_type(rng), place_name),
                _ => format!("The {} Sample (Three Taxonomists Disagree)", cultural_word),
            }
        }
        ArtifactKind::KeyToSomething => {
            match rng.gen_range(0..4) {
                0 => format!("The Key to the {} Vault (Lock Not Found)", cultural_word),
                1 => format!("A Key Bearing the Seal of {}", inst_name),
                2 => format!("The {} Key — What It Opens Is a Matter of Record", place_name),
                _ => format!("Key #{} (Issued to {}, Never Returned)", rng.gen_range(1..300), personal_name),
            }
        }
    }
}

fn pick_weapon_type(rng: &mut StdRng) -> &'static str {
    let types = ["Blade", "Mace", "Spear", "Halberd", "Cudgel", "Ceremonial Pike"];
    types[rng.gen_range(0..types.len())]
}

fn pick_instrument(rng: &mut StdRng) -> &'static str {
    let types = ["Sextant", "Astrolabe", "Measuring Chain", "Theodolite", "Compass", "Surveyor's Level"];
    types[rng.gen_range(0..types.len())]
}

fn pick_specimen_type(rng: &mut StdRng) -> &'static str {
    let types = ["Organism", "Appendage", "Carapace Fragment", "Egg", "Root System", "Secretion"];
    types[rng.gen_range(0..types.len())]
}

fn pick_ordinal(rng: &mut StdRng) -> &'static str {
    let ordinals = ["Second", "Third", "Fifth", "Seventh", "Ninth", "Eleventh", "Thirteenth"];
    ordinals[rng.gen_range(0..ordinals.len())]
}

fn pick_temporal_hedge(rng: &mut StdRng) -> &'static str {
    let hedges = ["Pending Review", "Until Further Notice", "Subject to Revision",
                  "Without Prejudice", "Until the Relevant Inquiry Concludes"];
    hedges[rng.gen_range(0..hedges.len())]
}
