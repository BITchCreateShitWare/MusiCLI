#![allow(unused)]
use tauri::command;
use tauri_plugin_dialog::DialogExt;

#[command]
#[cfg(desktop)]
pub async fn open_files_dialog(app: tauri::AppHandle) -> Result<Vec<String>, String> {
    let result = app
        .dialog()
        .file()
        .add_filter("Audio Files", &["mp3", "flac", "wav", "ogg", "m4a"])
        .set_title("Select Audio Files")
        .pick_files()
        .await
        .map_err(|e| e.to_string())?;

    match result {
        Some(paths) => Ok(paths.iter().map(|p| p.to_string()).collect()),
        None => Ok(vec![]),
    }
}

#[command]
#[cfg(not(desktop))]
pub async fn open_files_dialog(_app: tauri::AppHandle) -> Result<Vec<String>, String> {
    Ok(vec![])
}

#[command]
#[cfg(desktop)]
pub async fn open_image_dialog(app: tauri::AppHandle) -> Result<Option<String>, String> {
    let result = app
        .dialog()
        .file()
        .add_filter("Images", &["jpg", "jpeg", "png", "gif", "webp", "bmp"])
        .set_title("Select Background Image")
        .pick_file()
        .await
        .map_err(|e| e.to_string())?;
    Ok(result.map(|p| p.to_string()))
}

#[command]
#[cfg(not(desktop))]
pub async fn open_image_dialog(_app: tauri::AppHandle) -> Result<Option<String>, String> {
    Ok(None)
}

#[command]
#[cfg(desktop)]
pub async fn open_folder_dialog(app: tauri::AppHandle) -> Result<Option<String>, String> {
    // 关键：替换 .file() → .folder()
    let result = app
        .dialog()
        .folder()
        .set_title("Select Music Folder")
        .pick_folder()
        .await
        .map_err(|e| e.to_string())?;
    Ok(result.map(|p| p.to_string()))
}

#[command]
#[cfg(not(desktop))]
pub async fn open_folder_dialog(_app: tauri::AppHandle) -> Result<Option<String>, String> {
    Ok(None)
}

#[command]
#[cfg(desktop)]
pub async fn open_font_dialog(app: tauri::AppHandle) -> Result<Option<String>, String> {
    let result = app
        .dialog()
        .file()
        .add_filter("Font Files", &["ttf", "otf", "woff", "woff2"])
        .set_title("Select Font File")
        .pick_file()
        .await
        .map_err(|e| e.to_string())?;
    Ok(result.map(|p| p.to_string()))
}

#[command]
#[cfg(not(desktop))]
pub async fn open_font_dialog(_app: tauri::AppHandle) -> Result<Option<String>, String> {
    Ok(None)
}

#[command]
#[cfg(desktop)]
pub async fn save_file_dialog(
    app: tauri::AppHandle,
    default_name: String,
    filters: Option<Vec<serde_json::Value>>,
) -> Result<Option<String>, String> {
    let mut dialog = app
        .dialog()
        .file()
        .set_title("Save File")
        .set_file_name(&default_name);

    if let Some(ref f) = filters {
        for filter in f {
            if let (Some(name), Some(exts)) = (filter.get("name"), filter.get("extensions")) {
                if let (Some(name_str), Some(ext_arr)) = (name.as_str(), exts.as_array()) {
                    let exts_vec: Vec<&str> = ext_arr.iter().filter_map(|e| e.as_str()).collect();
                    dialog = dialog.add_filter(name_str, &exts_vec);
                }
            }
        }
    } else {
        dialog = dialog.add_filter("Theme Files", &["json"]);
    }

    let result = dialog.save_file().await.map_err(|e| e.to_string())?;
    Ok(result.map(|p| p.to_string()))
}

#[command]
#[cfg(not(desktop))]
pub async fn save_file_dialog(
    _app: tauri::AppHandle,
    _default_name: String,
    _filters: Option<Vec<serde_json::Value>>,
) -> Result<Option<String>, String> {
    Ok(None)
}

#[command]
#[cfg(desktop)]
pub async fn open_theme_dialog(app: tauri::AppHandle) -> Result<Option<String>, String> {
    let result = app
        .dialog()
        .file()
        .add_filter("Theme Files", &["json"])
        .set_title("Import Theme")
        .pick_file()
        .await
        .map_err(|e| e.to_string())?;
    Ok(result.map(|p| p.to_string()))
}

#[command]
#[cfg(not(desktop))]
pub async fn open_theme_dialog(_app: tauri::AppHandle) -> Result<Option<String>, String> {
    Ok(None)
}

#[tauri::command]
#[cfg(desktop)]
pub async fn save_dir_dialog(app: tauri::AppHandle) -> Result<Option<String>, String> {
    let result = app
        .dialog()
        .folder()
        .set_title("Select Export Directory")
        .pick_folder()
        .await
        .map_err(|e| e.to_string())?;
    Ok(result.map(|p| p.to_string()))
}

#[tauri::command]
#[cfg(not(desktop))]
pub async fn save_dir_dialog(_app: tauri::AppHandle) -> Result<Option<String>, String> {
    Ok(None)
}

#[command]
#[cfg(desktop)]
pub async fn open_sync_dialog(app: tauri::AppHandle) -> Result<Option<String>, String> {
    let result = app
        .dialog()
        .file()
        .add_filter("MusicLI Sync Package", &["zip"])
        .add_filter("MusicLI Manifest", &["json"])
        .set_title("Select MusicLI Sync File")
        .pick_file()
        .await
        .map_err(|e| e.to_string())?;
    Ok(result.map(|p| p.to_string()))
}

#[command]
#[cfg(not(desktop))]
pub async fn open_sync_dialog(_app: tauri::AppHandle) -> Result<Option<String>, String> {
    Ok(None)
}
