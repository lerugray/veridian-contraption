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
