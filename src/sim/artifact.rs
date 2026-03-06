use serde::{Deserialize, Serialize};

/// The kind of artifact.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArtifactKind {
    Weapon,
    Document,
    Vessel,
    Instrument,
    Relic,
    FormalWrit,
    TaxonomicSpecimen,
    KeyToSomething,
}

impl ArtifactKind {
    pub fn label(&self) -> &'static str {
        match self {
            ArtifactKind::Weapon => "Weapon",
            ArtifactKind::Document => "Document",
            ArtifactKind::Vessel => "Vessel",
            ArtifactKind::Instrument => "Instrument",
            ArtifactKind::Relic => "Relic",
            ArtifactKind::FormalWrit => "Formal Writ",
            ArtifactKind::TaxonomicSpecimen => "Taxonomic Specimen",
            ArtifactKind::KeyToSomething => "Key to Something",
        }
    }
}

/// Where an artifact currently is.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArtifactLocation {
    /// In a site (site index).
    InSite(usize),
    /// Held by an agent (agent id).
    HeldByAgent(u64),
    /// In a settlement (settlement index).
    InSettlement(usize),
    /// Lost — location unknown.
    Lost,
}

/// A notable object with a history.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub id: u64,
    pub name: String,
    pub kind: ArtifactKind,
    pub material: String,
    /// Chronicle of ownership and significant events.
    pub history: Vec<String>,
    pub current_location: ArtifactLocation,
    /// 1-3 descriptive properties.
    pub properties: Vec<String>,
    /// Institution that considers this artifact theirs, if any.
    pub institutional_claim: Option<u64>,
}
