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

        if !agent.institution_ids.is_empty() {
            let affils: Vec<String> = agent.institution_ids.iter()
                .filter_map(|&id| sim.institutions.iter().find(|i| i.id == id).map(|i| format!("{} ({})", i.name, i.kind.label())))
                .collect();
            writeln!(file, "  Affiliations: {}", affils.join(", "))?;
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

/// Info about a save file for the Load World screen.
#[derive(Debug, Clone)]
pub struct SaveFileInfo {
    pub name: String,
    pub path: String,
}

/// List all save files in /saves/.
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
                saves.push(SaveFileInfo {
                    name,
                    path: path.to_string_lossy().to_string(),
                });
            }
        }
    }

    saves.sort_by(|a, b| a.name.cmp(&b.name));
    saves
}

/// Check whether an autosave file exists.
pub fn has_autosave() -> bool {
    PathBuf::from("saves").join("autosave.json").exists()
}
