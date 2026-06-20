use std::process::Command;
use tauri::{command, AppHandle};

#[command]
pub async fn exec_external(_app: AppHandle, cmd: String) -> serde_json::Value {
    let cmd_trim = cmd.trim();
    if cmd_trim.is_empty() {
        return serde_json::json!({ "error": "empty command" });
    }

    let mut tokens = cmd_trim.split_whitespace();
    let Some(bin) = tokens.next() else {
        return serde_json::json!({ "error": "no executable name" });
    };
    let args: Vec<&str> = tokens.collect();

    match Command::new(bin).args(args).spawn() {
        Ok(_child) => serde_json::json!({}),
        Err(err) => serde_json::json!({ "error": err.to_string() }),
    }
}
