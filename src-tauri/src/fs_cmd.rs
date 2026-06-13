use std::fs;
use std::path::Path;
use tauri::command;

#[command]
pub async fn read_file(path: String) -> Result<String, String> {
    fs::read_to_string(&path).map_err(|e| e.to_string())
}

#[command]
pub async fn write_file(path: String, content: String) -> Result<(), String> {
    if let Some(parent) = Path::new(&path).parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    fs::write(&path, content).map_err(|e| e.to_string())
}

#[command]
pub async fn read_file_base64(path: String) -> Result<String, String> {
    let bytes = fs::read(&path).map_err(|e| e.to_string())?;
    use base64::Engine;
    Ok(base64::engine::general_purpose::STANDARD.encode(&bytes))
}

#[command]
pub async fn list_audio_files(dir_path: String) -> Result<Vec<String>, String> {
    let exts = [".mp3", ".flac", ".wav", ".ogg", ".m4a", ".wma"];
    let mut files: Vec<String> = fs::read_dir(&dir_path)
        .map_err(|e| e.to_string())?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .map(|ext| {
                    let ext_str = format!(".{}", ext.to_string_lossy().to_lowercase());
                    exts.contains(&ext_str.as_str())
                })
                .unwrap_or(false)
        })
        .map(|e| e.path().to_string_lossy().to_string())
        .collect();
    files.sort();
    Ok(files)
}

#[command]
pub async fn dir_exists(dir_path: String) -> Result<bool, String> {
    Ok(Path::new(&dir_path).is_dir())
}

#[command]
pub async fn copy_file(src: String, dest: String) -> Result<(), String> {
    if let Some(parent) = Path::new(&dest).parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    fs::copy(&src, &dest).map_err(|e| e.to_string())?;
    Ok(())
}

#[command]
pub async fn make_dir(dir: String) -> Result<(), String> {
    fs::create_dir_all(&dir).map_err(|e| e.to_string())
}
