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
}

impl EventType {
    /// Log color by event category.
    pub fn log_color(&self) -> Color {
        match self {
            // Personal events — white
            EventType::AgentBorn
            | EventType::AgentDied
            | EventType::AgentArrived
            | EventType::AgentDeparted
            | EventType::AgeEvent => Color::White,

            // Institutional/faction events — cyan
            EventType::InstitutionFounded
            | EventType::InstitutionDissolved
            | EventType::SchismOccurred
            | EventType::DoctrineShifted
            | EventType::MemberJoined
            | EventType::MemberDeparted
            | EventType::MemberExpelled => Color::Cyan,

            // Political events — yellow
            EventType::AllianceFormed
            | EventType::AllianceStrained
            | EventType::RivalryDeclared => Color::Yellow,

            // Environmental events — green
            EventType::WeatherEvent
            | EventType::SettlementGrew
            | EventType::SettlementShrank => Color::Green,

            // Cosmological / world-level — magenta
            EventType::WorldGenesis
            | EventType::CensusReport => Color::Magenta,
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
