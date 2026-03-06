/// Generate sample prose output at various weirdness levels and registers.
/// Run with: cargo run --example prose_samples

use rand::rngs::StdRng;
use rand::SeedableRng;

// We need to reference the crate's modules
use veridian_contraption::gen::prose_gen;
use veridian_contraption::gen::name_gen;
use veridian_contraption::sim::event::EventType;
use veridian_contraption::sim::world::NarrativeRegister;
use veridian_contraption::sim::institution::InstitutionKind;

fn main() {
    let mut rng = StdRng::seed_from_u64(42);

    let registers = [
        NarrativeRegister::Bureaucratic,
        NarrativeRegister::Clinical,
        NarrativeRegister::Lyrical,
        NarrativeRegister::Ominous,
        NarrativeRegister::Conspiratorial,
    ];

    let event_types = [
        EventType::AgentDied,
        EventType::AgentArrived,
        EventType::AgentBorn,
        EventType::SettlementGrew,
        EventType::WeatherEvent,
    ];

    let weirdness_levels: [(f32, &str); 3] = [
        (0.2, "Low Weirdness"),
        (0.5, "Medium Weirdness"),
        (0.9, "High Weirdness"),
    ];

    println!("=== VERIDIAN CONTRAPTION — PROSE SAMPLES ===\n");

    // Generate 20 sample entries across registers, event types, and weirdness
    let mut count = 0;
    for (weirdness, weirdness_label) in &weirdness_levels {
        for register in &registers {
            if count >= 20 { break; }

            // Pick an event type cycling through
            let event_type = &event_types[count % event_types.len()];

            let description = prose_gen::generate_description(
                event_type,
                Some("Whelm Durr-Anquist"),
                Some("Pelmwick"),
                100,
                &mut rng,
                *register,
                *weirdness,
            );

            println!("[{} / {} / {:?}]", weirdness_label, register.label(), event_type);
            println!("  {}\n", description);
            count += 1;
        }
    }

    // Show some institutional events
    println!("\n=== INSTITUTIONAL EVENTS ===\n");

    let inst_events = [
        EventType::InstitutionFounded,
        EventType::SchismOccurred,
        EventType::MemberExpelled,
        EventType::AllianceStrained,
        EventType::InstitutionDissolved,
    ];

    for (i, event_type) in inst_events.iter().enumerate() {
        let reg = registers[i % registers.len()];
        let w = 0.7;
        let desc = prose_gen::generate_institutional_description(
            event_type,
            Some("Orrith the Twice-Appointed"),
            Some("The Bureau of Provisional Territories"),
            Some("The Confraternity of the Ossified Scale"),
            &mut rng,
            reg,
            w,
        );
        println!("[{} / {:?}]", reg.label(), event_type);
        println!("  {}\n", desc);
    }

    // Show some adventurer deaths
    println!("\n=== ADVENTURER DEATHS ===\n");
    for reg in &registers {
        let desc = prose_gen::generate_adventurer_death(
            "Krath the Ill-Advised",
            "the Vaults of Indeterminate Purpose",
            &mut rng,
            *reg,
            0.8,
        );
        println!("[{}]", reg.label());
        println!("  {}\n", desc);
    }

    // Show weirdness-sensitive institution names
    println!("\n=== INSTITUTION NAMES (High Weirdness) ===\n");
    let phonemes = name_gen::load_phoneme_data();
    let kinds = [
        InstitutionKind::Guild,
        InstitutionKind::Government,
        InstitutionKind::RegulatoryBody,
        InstitutionKind::Cult,
        InstitutionKind::SecretSociety,
    ];
    for kind in &kinds {
        let name = name_gen::generate_institution_name_with_weirdness(
            kind, &phonemes, 0, 0.95, &mut rng,
        );
        println!("  {}", name);
    }

    // Show oxymoronic epithets
    println!("\n=== OXYMORONIC EPITHETS (High Weirdness) ===\n");
    for _ in 0..8 {
        let epithet = name_gen::generate_epithet_with_weirdness(
            &EventType::AgentArrived,
            Some("Pelmwick"),
            0.95,
            &mut rng,
        );
        println!("  {}", epithet);
    }

    println!("\n=== END OF SAMPLES ===");
}
