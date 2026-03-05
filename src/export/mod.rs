use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::time::SystemTime;

use crate::sim::event::Event;

/// Export all events to a TXT file in /exports/.
/// Returns the path of the exported file on success.
pub fn export_log(events: &[Event], prefix: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Ensure exports directory exists
    let export_dir = PathBuf::from("exports");
    fs::create_dir_all(&export_dir)?;

    // Generate a timestamp-ish suffix from system time
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    // Sanitize prefix: keep only alphanumeric, hyphens, underscores
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
