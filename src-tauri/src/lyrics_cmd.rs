use serde::{Deserialize, Serialize};
use tauri::{command, AppHandle, Emitter, Manager};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LyricsUpdateData {
    current: String,
    next: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LyricsThemeData {
    font: Option<String>,
    #[serde(rename = "fontSize")]
    font_size: Option<f64>,
    fg: Option<String>,
    #[serde(rename = "fgDim")]
    fg_dim: Option<String>,
    accent: Option<String>,
    bg: Option<String>,
    #[serde(rename = "lyricsAccent")]
    lyrics_accent: Option<String>,
    #[serde(rename = "lyricsFg")]
    lyrics_fg: Option<String>,
    #[serde(rename = "lyricsNextCount")]
    lyrics_next_count: Option<i32>,
    #[serde(rename = "lyricsGap")]
    lyrics_gap: Option<f64>,
    #[serde(rename = "lyricsShadow")]
    lyrics_shadow: Option<String>,
    #[serde(rename = "lyricsAlign")]
    lyrics_align: Option<String>,
    #[serde(rename = "lyricsCurrentSize")]
    lyrics_current_size: Option<f64>,
    #[serde(rename = "lyricsNextSize")]
    lyrics_next_size: Option<f64>,
    #[serde(rename = "lyricsVertical")]
    lyrics_vertical: Option<String>,
}

use std::sync::Mutex;

static LAST_LYRICS_THEME: Mutex<Option<LyricsThemeData>> = Mutex::new(None);

#[tauri::command]
#[cfg(desktop)]
pub async fn show_lyrics_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(w) = app.get_webview_window("lyrics") {
        w.show().map_err(|e| e.to_string())?;
        return Ok(());
    }

    let _window = tauri::WindowBuilder::new(&app, "lyrics")
        // 窗口基础尺寸限制
        .inner_size(600.0, 400.0)
        .min_inner_size(600.0, 80.0)
        .max_inner_size(600.0, 10000.0)
        // 原 decorations(false) 无边框
        .decorations(false)
        // 原 shadow(false) 关闭窗口阴影
        .shadow(false)
        // 置顶
        .always_on_top(true)
        // 不在任务栏显示
        .skip_taskbar(true)
        .resizable(true)
        // 透明窗口
        .transparent(true)
        // 转换为 WebviewWindowBuilder，加载页面
        .into_webview(tauri::WebviewUrl::App("/#/lyrics".into()))
        .title("Lyrics")
        .build()
        .map_err(|e| e.to_string())?;

    // Replay last theme after window loads
    if let Ok(guard) = LAST_LYRICS_THEME.lock() {
        if let Some(ref theme) = *guard {
            if let Some(w) = app.get_webview_window("lyrics") {
                let _ = w.emit("lyrics:update-theme", theme.clone());
            }
        }
    }

    Ok(())
}

// 移动端空存根（你已禁用Android，可保留或删除）
#[tauri::command]
#[cfg(not(desktop))]
pub async fn show_lyrics_window(_app: tauri::AppHandle) -> Result<(), String> {
    Ok(())
}

#[command]
pub async fn hide_lyrics_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("lyrics") {
        window.destroy().map_err(|e| e.to_string())?;
    }
    if let Some(main) = app.get_webview_window("main") {
        let _ = main.emit(
            "lyrics:visibility-changed",
            serde_json::json!({"visible": false}),
        );
    }
    Ok(())
}

#[command]
pub async fn send_lyrics_update(app: AppHandle, data: LyricsUpdateData) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("lyrics") {
        window
            .emit("lyrics:update", data)
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[command]
pub async fn send_lyrics_theme(app: AppHandle, data: LyricsThemeData) -> Result<(), String> {
    if let Ok(mut guard) = LAST_LYRICS_THEME.lock() {
        *guard = Some(data.clone());
    }
    if let Some(w) = app.get_webview_window("lyrics") {
        w.emit("lyrics:update-theme", data)
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[command]
pub async fn lyrics_auto_size(app: AppHandle, _w: f64, h: f64) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("lyrics") {
        // Use LogicalSize so CSS pixels map 1:1 regardless of DPI scale.
        let new_h = ((h + 48.0).max(80.0)).round();
        window
            .set_size(tauri::Size::Logical(tauri::LogicalSize {
                width: 600.0,
                height: new_h,
            }))
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
#[cfg(desktop)]
pub async fn lyrics_set_mouse_events(app: AppHandle, enabled: bool) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("lyrics") {
        // enabled = 允许鼠标交互 → accept_mouse_events = enabled
        window
            .set_accept_mouse_events(enabled)
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

// 移动端空实现
#[tauri::command]
#[cfg(not(desktop))]
pub async fn lyrics_set_mouse_events(_app: AppHandle, _enabled: bool) -> Result<(), String> {
    Ok(())
}
