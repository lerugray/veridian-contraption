use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::time::SystemTime;

use crate::sim::event::Event;
use crate::sim::institution::Institution;
use crate::sim::{SaveData, SimState};

// ---------------------------------------------------------------------------
// TXT Export
// ---------------------------------------------------------------------------

/// Export all events to a TXT file in /exports/.
/// Returns the path of the exported file on success.
pub fn export_log(events: &[Event], prefix: &str) -> Result<String, Box<dyn std::error::Error>> {
    let export_dir = PathBuf::from("exports");
    fs::create_dir_all(&export_dir)?;

    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let safe_prefix: String = prefix
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
        .collect();
    let safe_prefix = if safe_prefix.is_empty() {
        "log".to_string()
    } else {
        safe_prefix
    };

    let filename = format!("{}_{}.txt", safe_prefix, timestamp);
    let path = export_dir.join(&filename);

    let mut file = fs::File::create(&path)?;

    writeln!(file, "VERIDIAN CONTRAPTION — LIVE LOG EXPORT")?;
    writeln!(file, "======================================")?;
    writeln!(file)?;

    for event in events {
        writeln!(file, "[{}] {}", event.tick, event.description)?;
    }

    writeln!(file)?;
    writeln!(file, "--- End of export ({} entries) ---", events.len())?;

    Ok(path.to_string_lossy().to_string())
}

/// Export a Faction Record for all living institutions.
pub fn export_faction_record(sim: &SimState, prefix: &str) -> Result<String, Box<dyn std::error::Error>> {
    let export_dir = PathBuf::from("exports");
    fs::create_dir_all(&export_dir)?;

    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let safe_prefix = sanitize_prefix(prefix, "factions");
    let filename = format!("{}_{}.txt", safe_prefix, timestamp);
    let path = export_dir.join(&filename);

    let mut file = fs::File::create(&path)?;

    writeln!(file, "VERIDIAN CONTRAPTION — FACTION RECORD")?;
    writeln!(file, "World: {}  |  Tick: {}", sim.world.name, sim.world.tick)?;
    writeln!(file, "======================================")?;
    writeln!(file)?;

    let living: Vec<&Institution> = sim.institutions.iter().filter(|i| i.alive).collect();
    let defunct: Vec<&Institution> = sim.institutions.iter().filter(|i| !i.alive).collect();

    writeln!(file, "ACTIVE INSTITUTIONS ({})", living.len())?;
    writeln!(file, "--------------------------------------")?;
    writeln!(file)?;

    for inst in &living {
        write_institution_record(&mut file, inst, sim)?;
    }

    if !defunct.is_empty() {
        writeln!(file, "DEFUNCT INSTITUTIONS ({})", defunct.len())?;
        writeln!(file, "--------------------------------------")?;
        writeln!(file)?;
        for inst in &defunct {
            write_institution_record(&mut file, inst, sim)?;
        }
    }

    writeln!(file, "--- End of Faction Record ({} total) ---", sim.institutions.len())?;

    Ok(path.to_string_lossy().to_string())
}

fn write_institution_record(file: &mut fs::File, inst: &Institution, sim: &SimState) -> Result<(), Box<dyn std::error::Error>> {
    writeln!(file, "  {}", inst.name)?;
    writeln!(file, "  Kind: {}  |  Power: {}  |  Members: {}  |  Founded: tick {}", inst.kind.label(), inst.power, inst.member_ids.len(), inst.founded_tick)?;
    writeln!(file, "  Charter: {}", inst.charter)?;
    writeln!(file, "  Actual function: {}", inst.actual_function)?;
    if !inst.doctrine.is_empty() {
        writeln!(file, "  Doctrine: {}", inst.doctrine.join("; "))?;
    }
    if !inst.relationships.is_empty() {
        for (&other_id, rel) in &inst.relationships {
            let other_name = sim.institutions.iter()
                .find(|i| i.id == other_id)
                .map(|i| i.name.as_str())
                .unwrap_or("unknown");
            writeln!(file, "  Relationship: {} — {}", other_name, rel.label())?;
        }
    }
    // Members
    let member_names: Vec<String> = inst.member_ids.iter()
        .filter_map(|&id| sim.agents.iter().find(|a| a.id == id).map(|a| a.display_name()))
        .collect();
    if !member_names.is_empty() {
        writeln!(file, "  Members: {}", member_names.join(", "))?;
    }
    // Chronicle
    if !inst.chronicle.is_empty() {
        writeln!(file, "  Chronicle:")?;
        for entry in &inst.chronicle {
            writeln!(file, "    - {}", entry)?;
        }
    }
    writeln!(file)?;
    Ok(())
}

/// Export a Character Chronicle for all living agents.
pub fn export_character_chronicle(sim: &SimState, prefix: &str) -> Result<String, Box<dyn std::error::Error>> {
    let export_dir = PathBuf::from("exports");
    fs::create_dir_all(&export_dir)?;

    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let safe_prefix = sanitize_prefix(prefix, "chronicles");
    let filename = format!("{}_{}.txt", safe_prefix, timestamp);
    let path = export_dir.join(&filename);

    let mut file = fs::File::create(&path)?;

    writeln!(file, "VERIDIAN CONTRAPTION — CHARACTER CHRONICLES")?;
    writeln!(file, "World: {}  |  Tick: {}", sim.world.name, sim.world.tick)?;
    writeln!(file, "============================================")?;
    writeln!(file)?;

    let mut living: Vec<usize> = sim.agents.iter().enumerate()
        .filter(|(_, a)| a.alive)
        .map(|(i, _)| i)
        .collect();
    living.sort_by(|&a, &b| sim.agents[a].name.cmp(&sim.agents[b].name));

    writeln!(file, "LIVING AGENTS ({})", living.len())?;
    writeln!(file, "--------------------------------------------")?;
    writeln!(file)?;

    for &idx in &living {
        let agent = &sim.agents[idx];
        let people_name = if agent.people_id < sim.world.peoples.len() {
            &sim.world.peoples[agent.people_id].name
        } else {
            "Unknown"
        };

        writeln!(file, "  {}", agent.display_name())?;
        writeln!(file, "  People: {}  |  Age: {} years  |  Health: {}/100", people_name, agent.age / 365, agent.health)?;
        writeln!(file, "  Location: ({}, {})", agent.x, agent.y)?;

        if !agent.epithets.is_empty() {
            writeln!(file, "  Epithets: {}", agent.epithets.join(", "))?;
        }

        if agent.is_adventurer {
            writeln!(file, "  Role: Adventurer")?;
        }

        if !agent.institution_ids.is_empty() {
            let affils: Vec<String> = agent.institution_ids.iter()
                .filter_map(|&id| sim.institutions.iter().find(|i| i.id == id).map(|i| format!("{} ({})", i.name, i.kind.label())))
                .collect();
            writeln!(file, "  Affiliations: {}", affils.join(", "))?;
        }

        // Currently held artifacts
        if !agent.held_artifacts.is_empty() {
            writeln!(file, "  Held artifacts:")?;
            for &art_id in &agent.held_artifacts {
                if let Some(art) = sim.artifacts.iter().find(|a| a.id == art_id) {
                    writeln!(file, "    - {} ({})", art.name, art.kind.label())?;
                }
            }
        }

        // Agent's events from the log
        let agent_events: Vec<&Event> = sim.events.iter()
            .filter(|e| e.subject_id == Some(agent.id))
            .collect();
        if !agent_events.is_empty() {
            writeln!(file, "  Chronicle:")?;
            for event in &agent_events {
                writeln!(file, "    [{}] {}", event.tick, event.description)?;
            }
        }
        writeln!(file)?;
    }

    writeln!(file, "--- End of Character Chronicles ({} agents) ---", living.len())?;

    Ok(path.to_string_lossy().to_string())
}

/// Export the World Annals as a formatted historical document.
pub fn export_world_annals(sim: &SimState, prefix: &str) -> Result<String, Box<dyn std::error::Error>> {
    let export_dir = PathBuf::from("exports");
    fs::create_dir_all(&export_dir)?;

    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let safe_prefix = sanitize_prefix(prefix, "annals");
    let filename = format!("{}_{}.txt", safe_prefix, timestamp);
    let path = export_dir.join(&filename);

    let mut file = fs::File::create(&path)?;

    writeln!(file, "VERIDIAN CONTRAPTION — WORLD ANNALS")?;
    writeln!(file, "World: {}  |  Tick: {}", sim.world.name, sim.world.tick)?;
    writeln!(file, "=====================================")?;
    writeln!(file)?;

    if sim.annals.is_empty() {
        writeln!(file, "No completed eras have been recorded.")?;
        writeln!(file)?;
    }

    for entry in &sim.annals {
        writeln!(file, "═══════════════════════════════════════")?;
        writeln!(file, "{}", entry.era_name)?;
        writeln!(file, "Ticks {} – {}", entry.start_tick, entry.end_tick)?;
        writeln!(file, "═══════════════════════════════════════")?;
        writeln!(file)?;
        writeln!(file, "{}", entry.summary)?;
        writeln!(file)?;
        if !entry.notable_agents.is_empty() {
            writeln!(file, "Notable figures: {}", entry.notable_agents.join(", "))?;
        }
        if !entry.notable_institutions.is_empty() {
            writeln!(file, "Notable institutions: {}", entry.notable_institutions.join(", "))?;
        }
        writeln!(file, "Defining event: {}", entry.defining_event)?;
        writeln!(file)?;
    }

    writeln!(file, "═══════════════════════════════════════")?;
    writeln!(file, "{} (tick {}–present)  CURRENT ERA (ongoing)", sim.current_era_name, sim.current_era_start)?;
    writeln!(file, "═══════════════════════════════════════")?;
    writeln!(file)?;

    let alive = sim.agents.iter().filter(|a| a.alive).count();
    let living_inst = sim.institutions.iter().filter(|i| i.alive).count();
    writeln!(file, "Population: {}  |  Institutions: {}  |  Major events this era: {}",
        alive, living_inst, sim.era_major_events)?;
    writeln!(file)?;
    writeln!(file, "--- End of World Annals ({} completed eras) ---", sim.annals.len())?;

    Ok(path.to_string_lossy().to_string())
}

/// Sanitize a user-provided prefix for use as a filename component.
fn sanitize_prefix(prefix: &str, default: &str) -> String {
    let safe: String = prefix
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
        .collect();
    if safe.is_empty() { default.to_string() } else { safe }
}

// ---------------------------------------------------------------------------
// Save / Load System
// ---------------------------------------------------------------------------

/// Maximum number of named save slots (excluding autosave).
pub const MAX_SAVE_SLOTS: usize = 10;

/// Ensure the saves directory exists.
pub fn ensure_saves_dir() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("saves")?;
    Ok(())
}

/// Save the simulation state to a JSON file in /saves/.
/// Returns the path of the saved file on success.
pub fn save_world(sim: &SimState, name: &str) -> Result<String, Box<dyn std::error::Error>> {
    ensure_saves_dir()?;

    let save_data = sim.to_save_data();
    let json = serde_json::to_string_pretty(&save_data)?;

    // Sanitize name for filename safety
    let safe_name: String = name
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_' || *c == ' ')
        .collect();
    let safe_name = if safe_name.is_empty() {
        "world".to_string()
    } else {
        safe_name
    };

    let filename = format!("{}.json", safe_name);
    let path = PathBuf::from("saves").join(&filename);
    fs::write(&path, json)?;

    Ok(path.to_string_lossy().to_string())
}

/// Load a simulation state from a JSON save file.
pub fn load_world(path: &str) -> Result<SimState, Box<dyn std::error::Error>> {
    let json = fs::read_to_string(path)?;
    let save_data: SaveData = serde_json::from_str(&json)?;
    Ok(SimState::from_save_data(save_data))
}

/// Enriched info about a save file for the Load World screen.
#[derive(Debug, Clone)]
pub struct SaveFileInfo {
    pub name: String,
    pub path: String,
    /// World name from inside the save.
    pub world_name: String,
    /// Current tick of the world.
    pub tick: u64,
    /// Number of living agents.
    pub population: usize,
    /// Current era name.
    pub era_name: String,
    /// Number of completed eras.
    pub era_count: usize,
    /// Whether this is an autosave.
    pub is_autosave: bool,
    /// File modification timestamp (seconds since epoch).
    pub modified_secs: u64,
}

/// Lightweight struct for reading save metadata without full deserialization.
#[derive(serde::Deserialize)]
struct SaveMetadata {
    world: WorldMeta,
    #[serde(default)]
    agents: Vec<AgentMeta>,
    #[serde(default)]
    annals: Vec<serde_json::Value>,
    #[serde(default)]
    current_era_name: Option<String>,
}

#[derive(serde::Deserialize)]
struct WorldMeta {
    #[serde(default)]
    name: String,
    #[serde(default)]
    tick: u64,
}

#[derive(serde::Deserialize)]
struct AgentMeta {
    #[serde(default)]
    alive: bool,
}

/// Read metadata from a save file without loading the full SimState.
fn read_save_metadata(path: &std::path::Path) -> Option<SaveMetadata> {
    let json = fs::read_to_string(path).ok()?;
    serde_json::from_str(&json).ok()
}

/// List all save files in /saves/, enriched with world metadata.
/// Sorted by modification time, most recent first.
pub fn list_saves() -> Vec<SaveFileInfo> {
    let save_dir = PathBuf::from("saves");
    if !save_dir.exists() {
        return Vec::new();
    }

    let mut saves = Vec::new();
    if let Ok(entries) = fs::read_dir(&save_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |e| e == "json") {
                let name = path
                    .file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();

                let is_autosave = name == "autosave";

                let modified_secs = fs::metadata(&path)
                    .and_then(|m| m.modified())
                    .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)))
                    .map(|d| d.as_secs())
                    .unwrap_or(0);

                // Read metadata from inside the save file
                let (world_name, tick, population, era_name, era_count) =
                    if let Some(meta) = read_save_metadata(&path) {
                        let pop = meta.agents.iter().filter(|a| a.alive).count();
                        let era = meta.current_era_name.unwrap_or_else(|| "Unknown Era".to_string());
                        let eras = meta.annals.len();
                        (meta.world.name, meta.world.tick, pop, era, eras)
                    } else {
                        (name.clone(), 0, 0, "Unknown".to_string(), 0)
                    };

                saves.push(SaveFileInfo {
                    name,
                    path: path.to_string_lossy().to_string(),
                    world_name,
                    tick,
                    population,
                    era_name,
                    era_count,
                    is_autosave,
                    modified_secs,
                });
            }
        }
    }

    // Sort by modification time, most recent first
    saves.sort_by(|a, b| b.modified_secs.cmp(&a.modified_secs));
    saves
}

/// Count the number of named (non-autosave) save files.
pub fn named_save_count() -> usize {
    let save_dir = PathBuf::from("saves");
    if !save_dir.exists() {
        return 0;
    }
    fs::read_dir(&save_dir)
        .map(|entries| {
            entries.flatten().filter(|e| {
                let path = e.path();
                path.extension().map_or(false, |ext| ext == "json")
                    && path.file_stem().map_or(false, |s| s != "autosave")
            }).count()
        })
        .unwrap_or(0)
}

/// Delete a save file. Returns Ok(()) on success.
pub fn delete_save(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    fs::remove_file(path)?;
    Ok(())
}

/// Find the path to the most recently modified save file (any type).
/// Returns None if no saves exist.
pub fn most_recent_save() -> Option<SaveFileInfo> {
    let saves = list_saves();
    saves.into_iter().next()
}
