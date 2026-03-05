use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::time::SystemTime;

use crate::sim::event::Event;
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
