use rand::rngs::StdRng;
use rand::Rng;
use serde::{Deserialize, Serialize};

/// Injury severity from combat.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InjuryStatus {
    Uninjured,
    Bruised,
    Wounded,
    GravelyWounded,
}

impl Default for InjuryStatus {
    fn default() -> Self {
        InjuryStatus::Uninjured
    }
}

impl InjuryStatus {
    pub fn recovery_ticks(self) -> u32 {
        match self {
            InjuryStatus::Uninjured => 0,
            InjuryStatus::Bruised => 30,
            InjuryStatus::Wounded => 120,
            InjuryStatus::GravelyWounded => 300,
        }
    }
}

/// Combat experience tier (hidden stat, displayed as prose).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CombatExperienceTier {
    /// No combat experience.
    Untested,
    /// 1-2 combats survived.
    Blooded,
    /// 3-5 combats survived.
    Seasoned,
    /// 6+ combats survived.
    Dangerous,
}

impl CombatExperienceTier {
    /// Advance experience based on total combats survived.
    pub fn from_count(combats_survived: u16) -> Self {
        match combats_survived {
            0 => CombatExperienceTier::Untested,
            1..=2 => CombatExperienceTier::Blooded,
            3..=5 => CombatExperienceTier::Seasoned,
            _ => CombatExperienceTier::Dangerous,
        }
    }

    /// Prose description for the inspect screen. Returns None for Untested.
    pub fn prose_description(self, rng: &mut StdRng) -> Option<&'static str> {
        match self {
            CombatExperienceTier::Untested => None,
            CombatExperienceTier::Blooded => {
                let opts = &[
                    "Carries themselves with the wariness of someone who has fought before.",
                    "Has been in at least one altercation that left an impression.",
                    "Moves with a caution that suggests prior encounters of a physical nature.",
                ];
                Some(opts[rng.gen_range(0..opts.len())])
            }
            CombatExperienceTier::Seasoned => {
                let opts = &[
                    "Has survived encounters that others have not.",
                    "Is regarded with a certain circumspection by those aware of their record.",
                    "Bears the composure of one who has been tested and found adequate.",
                ];
                Some(opts[rng.gen_range(0..opts.len())])
            }
            CombatExperienceTier::Dangerous => {
                let opts = &[
                    "Is regarded as dangerous by those who have observed them.",
                    "Has accumulated a reputation that precedes them into rooms.",
                    "Carries themselves with the stillness of someone who no longer needs to prove anything.",
                    "Is known to have resolved several disputes in a manner that left no room for appeal.",
                ];
                Some(opts[rng.gen_range(0..opts.len())])
            }
        }
    }
}

/// Result of a single combat resolution.
pub struct CombatResult {
    pub winner_id: u64,
    pub loser_id: u64,
    pub is_draw: bool,
    pub winner_injury: InjuryStatus,
    pub loser_injury: InjuryStatus,
    pub margin: f32, // how decisive the outcome was (0.0 = near-draw, 1.0 = domination)
}

/// Compute the hidden combat weight for an agent.
/// Higher = more effective combatant.
pub fn combat_weight(
    risk_tolerance: f32,
    age_ticks: u32,
    experience_count: u16,
    injury: InjuryStatus,
) -> f32 {
    // Risk tolerance is the primary driver (weighted heavily)
    let base = risk_tolerance * 0.6;

    // Age curve: peaks around 30-40 years (10950-14600 ticks), declines past ~60 (21900)
    let age_years = age_ticks as f32 / 365.0;
    let age_factor = if age_years < 20.0 {
        0.5 + (age_years / 20.0) * 0.5 // ramps from 0.5 to 1.0
    } else if age_years < 45.0 {
        1.0 // prime fighting years
    } else if age_years < 70.0 {
        1.0 - ((age_years - 45.0) / 25.0) * 0.5 // declines to 0.5
    } else {
        0.5 // elderly
    };

    // Experience bump: small but meaningful
    let exp_bonus = match CombatExperienceTier::from_count(experience_count) {
        CombatExperienceTier::Untested => 0.0,
        CombatExperienceTier::Blooded => 0.05,
        CombatExperienceTier::Seasoned => 0.12,
        CombatExperienceTier::Dangerous => 0.2,
    };

    // Injury penalty
    let injury_penalty = match injury {
        InjuryStatus::Uninjured => 0.0,
        InjuryStatus::Bruised => 0.0,
        InjuryStatus::Wounded => 0.15,
        InjuryStatus::GravelyWounded => 0.35,
    };

    (base * age_factor + exp_bonus - injury_penalty).max(0.05)
}

/// Resolve combat between two participants.
/// Returns CombatResult with winner/loser and injuries.
pub fn resolve_combat(
    id_a: u64, weight_a: f32,
    id_b: u64, weight_b: f32,
    rng: &mut StdRng,
) -> CombatResult {
    // Each participant rolls: weight + random variance
    let roll_a = weight_a + rng.gen_range(-0.15..0.15_f32);
    let roll_b = weight_b + rng.gen_range(-0.15..0.15_f32);

    let margin = (roll_a - roll_b).abs();
    let is_draw = margin < 0.05;

    let (winner_id, loser_id) = if is_draw {
        (id_a, id_b) // arbitrary for draws
    } else if roll_a > roll_b {
        (id_a, id_b)
    } else {
        (id_b, id_a)
    };

    let winner_injury;
    let loser_injury;

    if is_draw {
        // Both get bruised or wounded
        let roll: f32 = rng.gen();
        winner_injury = if roll < 0.6 { InjuryStatus::Bruised } else { InjuryStatus::Wounded };
        let roll2: f32 = rng.gen();
        loser_injury = if roll2 < 0.6 { InjuryStatus::Bruised } else { InjuryStatus::Wounded };
    } else {
        // Winner: usually uninjured, small chance of bruise
        winner_injury = if rng.gen_bool(0.2) { InjuryStatus::Bruised } else { InjuryStatus::Uninjured };

        // Loser: severity weighted by margin of defeat
        let roll: f32 = rng.gen();
        loser_injury = if margin > 0.3 {
            // Decisive defeat — higher chance of grave wounds
            if roll < 0.15 { InjuryStatus::Bruised }
            else if roll < 0.55 { InjuryStatus::Wounded }
            else { InjuryStatus::GravelyWounded }
        } else if margin > 0.15 {
            // Moderate defeat
            if roll < 0.3 { InjuryStatus::Bruised }
            else if roll < 0.75 { InjuryStatus::Wounded }
            else { InjuryStatus::GravelyWounded }
        } else {
            // Close fight
            if roll < 0.5 { InjuryStatus::Bruised }
            else if roll < 0.85 { InjuryStatus::Wounded }
            else { InjuryStatus::GravelyWounded }
        };
    }

    CombatResult {
        winner_id,
        loser_id,
        is_draw,
        winner_injury,
        loser_injury,
        margin,
    }
}

/// Prose description of an agent's current injury for the inspect screen.
/// Returns None if uninjured.
pub fn injury_prose(injury: InjuryStatus, recovery_remaining: u32, location_name: Option<&str>, rng: &mut StdRng) -> Option<String> {
    match injury {
        InjuryStatus::Uninjured => None,
        InjuryStatus::Bruised => {
            let opts = [
                "Bears minor bruises of no administrative consequence.",
                "Shows signs of a recent physical disagreement.",
                "Has sustained superficial injuries that the records office has declined to classify.",
            ];
            Some(opts[rng.gen_range(0..opts.len())].to_string())
        }
        InjuryStatus::Wounded => {
            if recovery_remaining > 60 {
                let base = match location_name {
                    Some(loc) => format!("Bears a wound sustained near {}, not yet healed.", loc),
                    None => "Is recovering from injuries of some severity.".to_string(),
                };
                Some(base)
            } else {
                Some("Is recovering from injuries, though the worst appears to have passed.".to_string())
            }
        }
        InjuryStatus::GravelyWounded => {
            if recovery_remaining > 150 {
                let opts = [
                    "Was gravely wounded and has not fully recovered.",
                    "Carries injuries that the attending clerk classified as 'of concern.'",
                    "Is in a condition that the medical registrar has described, with characteristic understatement, as 'notable.'",
                ];
                Some(opts[rng.gen_range(0..opts.len())].to_string())
            } else {
                Some("Is recovering from grave injuries, though survival now appears probable.".to_string())
            }
        }
    }
}
