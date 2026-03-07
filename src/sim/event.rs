use ratatui::style::Color;
use serde::{Deserialize, Serialize};

/// The type of event that occurred.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    AgentBorn,
    AgentDied,
    AgentArrived,
    AgentDeparted,
    SettlementGrew,
    SettlementShrank,
    WeatherEvent,
    AgeEvent,
    CensusReport,
    WorldGenesis,
    // Institutional events
    InstitutionFounded,
    InstitutionDissolved,
    SchismOccurred,
    DoctrineShifted,
    AllianceFormed,
    AllianceStrained,
    RivalryDeclared,
    MemberJoined,
    MemberDeparted,
    MemberExpelled,
    // Site events
    AgentEnteredSite,
    AgentLeftSite,
    // Artifact events
    ArtifactAcquired,
    ArtifactDelivered,
    AdventurerDiedInSite,
    // Demographic events
    AgentEmigrated,
    AgentImmigrated,
    NaturalDeath,
    // Inhabitant events
    InhabitantInteraction,
    // Faction events
    FactionDisbanded,
    // Relationship events
    RelationshipFormed,
    RelationshipChanged,
    // Seasonal events
    SeasonalTransition,
    // Eschaton events
    EschatonFired,
}

impl EventType {
    /// Log color by event category (truecolor).
    pub fn log_color(&self) -> Color {
        match self {
            // Personal events — warm white
            EventType::AgentBorn
            | EventType::AgentDied
            | EventType::AgentArrived
            | EventType::AgentDeparted
            | EventType::AgeEvent
            | EventType::AgentEmigrated
            | EventType::AgentImmigrated
            | EventType::NaturalDeath
            | EventType::RelationshipFormed
            | EventType::RelationshipChanged => Color::Rgb(200, 200, 195),

            // Institutional/faction events — teal
            EventType::InstitutionFounded
            | EventType::InstitutionDissolved
            | EventType::SchismOccurred
            | EventType::DoctrineShifted
            | EventType::MemberJoined
            | EventType::MemberDeparted
            | EventType::MemberExpelled
            | EventType::FactionDisbanded => Color::Rgb(100, 210, 220),

            // Political events — amber
            EventType::AllianceFormed
            | EventType::AllianceStrained
            | EventType::RivalryDeclared => Color::Rgb(220, 200, 100),

            // Environmental events — spring green
            EventType::WeatherEvent
            | EventType::SettlementGrew
            | EventType::SettlementShrank
            | EventType::SeasonalTransition => Color::Rgb(110, 200, 120),

            // Site events — rust
            EventType::AgentEnteredSite
            | EventType::AgentLeftSite
            | EventType::AdventurerDiedInSite
            | EventType::InhabitantInteraction => Color::Rgb(200, 110, 90),

            // Artifact events — gold
            EventType::ArtifactAcquired
            | EventType::ArtifactDelivered => Color::Rgb(240, 210, 100),

            // Cosmological / world-level — violet
            EventType::WorldGenesis
            | EventType::CensusReport => Color::Rgb(190, 130, 220),

            // Eschaton — bright crimson
            EventType::EschatonFired => Color::Rgb(255, 80, 80),
        }
    }

    /// Short category prefix for log display (color-coded).
    pub fn category_prefix(&self) -> String {
        match self {
            EventType::AgentBorn | EventType::AgentDied | EventType::AgentArrived
            | EventType::AgentDeparted | EventType::AgeEvent
            | EventType::AgentEmigrated | EventType::AgentImmigrated
            | EventType::NaturalDeath
            | EventType::RelationshipFormed | EventType::RelationshipChanged => "".to_string(),

            EventType::InstitutionFounded | EventType::InstitutionDissolved
            | EventType::SchismOccurred | EventType::DoctrineShifted
            | EventType::MemberJoined | EventType::MemberDeparted
            | EventType::MemberExpelled
            | EventType::FactionDisbanded => "\u{25C6} ".to_string(), // ◆

            EventType::AllianceFormed | EventType::AllianceStrained
            | EventType::RivalryDeclared => "\u{2694} ".to_string(), // ⚔ (crossed swords — political)

            EventType::WeatherEvent | EventType::SettlementGrew
            | EventType::SettlementShrank
            | EventType::SeasonalTransition => "\u{2618} ".to_string(), // ☘ (environmental)

            EventType::AgentEnteredSite | EventType::AgentLeftSite
            | EventType::AdventurerDiedInSite
            | EventType::InhabitantInteraction => "\u{2302} ".to_string(), // ⌂ (site)

            EventType::ArtifactAcquired | EventType::ArtifactDelivered
                => "\u{2726} ".to_string(), // ✦ (artifact)

            EventType::WorldGenesis | EventType::CensusReport
                => "\u{2735} ".to_string(), // ✵ (cosmological)

            EventType::EschatonFired => "\u{2620} ".to_string(), // ☠ (eschaton)
        }
    }
}

/// A single event in the simulation log.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub tick: u64,
    pub event_type: EventType,
    pub subject_id: Option<u64>,
    pub location: Option<(u32, u32)>,
    pub description: String,
}
