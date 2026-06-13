use tauri::{command, AppHandle, Manager};

#[command]
pub async fn minimize_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.minimize().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[command]
pub async fn default_music_dir() -> String {
    dirs::audio_dir()
        .or_else(|| dirs::home_dir().map(|h| h.join("Music")))
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default()
}
