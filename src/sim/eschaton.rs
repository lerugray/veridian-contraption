use serde::{Deserialize, Serialize};

/// The six types of world-altering Eschaton events.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EschatonType {
    TheReckoningOfDebts,
    TheTaxonomicCorrection,
    TheAdministrativeSingularity,
    TheGeologicalArgument,
    TheDoctrinalCascade,
    TheArrivalOfSomethingOwed,
}

impl EschatonType {
    pub fn label(&self) -> &'static str {
        match self {
            EschatonType::TheReckoningOfDebts => "The Reckoning of Debts",
            EschatonType::TheTaxonomicCorrection => "The Taxonomic Correction",
            EschatonType::TheAdministrativeSingularity => "The Administrative Singularity",
            EschatonType::TheGeologicalArgument => "The Geological Argument",
            EschatonType::TheDoctrinalCascade => "The Doctrinal Cascade",
            EschatonType::TheArrivalOfSomethingOwed => "The Arrival of Something Owed",
        }
    }

    /// Description shown in the World Report when listing past eschatons.
    #[allow(dead_code)]
    pub fn description(&self) -> &'static str {
        match self {
            EschatonType::TheReckoningOfDebts =>
                "All outstanding institutional disputes resolve simultaneously, catastrophically. \
                 Factions merge, dissolve, or invert their stated purposes. The political map \
                 rewrites itself in a single convulsive adjustment.",
            EschatonType::TheTaxonomicCorrection =>
                "All agent epithets are revoked and reassigned. The world's peoples are \
                 reclassified. Some settlements are renamed. The census is updated to reflect \
                 a reality that differs slightly from the previous one.",
            EschatonType::TheAdministrativeSingularity =>
                "All institutions merge into one vast entity, which immediately begins to \
                 schism. The resulting bureaucratic prose is of unprecedented density.",
            EschatonType::TheGeologicalArgument =>
                "The land itself disagrees with recent history. Geography reshapes: settlements \
                 move, disappear, or appear. Sites are reclassified. The world map updates to \
                 reflect terrain that has decided to be elsewhere.",
            EschatonType::TheDoctrinalCascade =>
                "Every institution simultaneously revises its foundational doctrine. Agent \
                 affiliations become unstable. New institutions form from the chaos of \
                 competing interpretations.",
            EschatonType::TheArrivalOfSomethingOwed =>
                "A new group of agents appears with mysterious origins and unclear purpose. \
                 Existing agents react according to their disposition profiles. The newcomers \
                 begin accumulating influence immediately.",
        }
    }

    /// Select a random eschaton type.
    pub fn random(rng: &mut rand::rngs::StdRng) -> Self {
        use rand::Rng;
        match rng.gen_range(0..6) {
            0 => EschatonType::TheReckoningOfDebts,
            1 => EschatonType::TheTaxonomicCorrection,
            2 => EschatonType::TheAdministrativeSingularity,
            3 => EschatonType::TheGeologicalArgument,
            4 => EschatonType::TheDoctrinalCascade,
            _ => EschatonType::TheArrivalOfSomethingOwed,
        }
    }
}

/// A record of a past Eschaton event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EschatonRecord {
    pub eschaton_type: EschatonType,
    pub tick: u64,
    pub era_name_before: String,
    pub era_name_after: String,
}

/// Minimum ticks between Eschaton events.
pub const ESCHATON_COOLDOWN: u64 = 500;

/// Tension threshold for autonomous trigger.
pub const TENSION_THRESHOLD: f32 = 0.7;

/// Cosmological density threshold for autonomous trigger.
pub const COSMO_THRESHOLD: f32 = 0.65;
