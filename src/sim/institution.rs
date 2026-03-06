use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// The kind of institution — determines naming style and behavioral tendencies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InstitutionKind {
    Guild,
    Government,
    Cult,
    MercenaryCompany,
    RegulatoryBody,
    SecretSociety,
}

impl InstitutionKind {
    pub fn label(&self) -> &'static str {
        match self {
            InstitutionKind::Guild => "Guild",
            InstitutionKind::Government => "Government",
            InstitutionKind::Cult => "Cult",
            InstitutionKind::MercenaryCompany => "Mercenary Company",
            InstitutionKind::RegulatoryBody => "Regulatory Body",
            InstitutionKind::SecretSociety => "Secret Society",
        }
    }
}

/// How one institution relates to another.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InstitutionRelationship {
    Allied,
    Neutral,
    Rival,
    Disputed(String),
}

impl InstitutionRelationship {
    pub fn label(&self) -> String {
        match self {
            InstitutionRelationship::Allied => "Allied".to_string(),
            InstitutionRelationship::Neutral => "Neutral".to_string(),
            InstitutionRelationship::Rival => "Rival".to_string(),
            InstitutionRelationship::Disputed(reason) => format!("Disputed ({})", reason),
        }
    }
}

/// An institution: faction, guild, government, cult, mercenary company, or regulatory body.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Institution {
    pub id: u64,
    pub name: String,
    pub kind: InstitutionKind,
    /// Stated purpose at founding.
    pub charter: String,
    /// What they actually do — may diverge over time.
    pub actual_function: String,
    /// Abstract resource/influence.
    pub power: u32,
    /// Official doctrinal positions (2-4 at founding).
    pub doctrine: Vec<String>,
    /// Agent IDs of current members.
    pub member_ids: Vec<u64>,
    /// Grid tiles this institution influences.
    pub territory: Vec<(u32, u32)>,
    /// Tick when this institution was founded.
    pub founded_tick: u64,
    /// How this institution relates to others (keyed by institution id).
    pub relationships: HashMap<u64, InstitutionRelationship>,
    /// Narrative history of this institution.
    pub chronicle: Vec<String>,
    /// People/culture this institution is associated with.
    pub people_id: usize,
    /// Whether this institution still exists.
    pub alive: bool,
}

impl Institution {
    /// One-line summary for the faction list.
    pub fn summary(&self) -> String {
        if self.charter == self.actual_function {
            self.charter.clone()
        } else {
            self.actual_function.clone()
        }
    }
}
