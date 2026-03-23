use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HistoryEntry {
    pub expression: String,
    pub result: f64,
}

#[derive(Serialize, Deserialize)]
struct HistoryFile {
    entries: Vec<HistoryEntry>,
}

pub fn export_history(path: &str, entries: &[(String, f64)]) -> Result<(), String> {
    let file = HistoryFile {
        entries: entries
            .iter()
            .map(|(expr, result)| HistoryEntry {
                expression: expr.clone(),
                result: *result,
            })
            .collect(),
    };
    let json = serde_json::to_string_pretty(&file).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())
}

pub fn import_history(path: &str) -> Result<Vec<HistoryEntry>, String> {
    if !Path::new(path).exists() {
        return Err(format!("File not found: {}", path));
    }
    let json = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let file: HistoryFile = serde_json::from_str(&json).map_err(|e| e.to_string())?;
    Ok(file.entries)
}
