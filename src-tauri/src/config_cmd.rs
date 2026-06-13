use std::fs;
use std::path::PathBuf;
use tauri::command;

fn config_path(music_folder: &str, key: &str) -> PathBuf {
    PathBuf::from(music_folder)
        .join("config")
        .join(format!("{}.json", key))
}

#[command]
pub async fn read_config(
    music_folder: String,
    key: String,
) -> Result<Option<serde_json::Value>, String> {
    let path = config_path(&music_folder, &key);
    if !path.exists() {
        return Ok(None);
    }
    let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let val: serde_json::Value = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    Ok(Some(val))
}

#[command]
pub async fn write_config(
    music_folder: String,
    key: String,
    data: serde_json::Value,
) -> Result<(), String> {
    let dir = PathBuf::from(&music_folder).join("config");
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let path = dir.join(format!("{}.json", key));
    let json = serde_json::to_string_pretty(&data).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| e.to_string())
}
