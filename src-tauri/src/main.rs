#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{command, Manager};
use std::process::Command;

#[command]
fn download_video(url: String, format: String, trim_start: Option<String>, trim_end: Option<String>) -> Result<(), String> {
    let mut args = vec![url, "-f".into(), format];

    if let Some(start) = trim_start { args.push("--download-sections".into()); args.push(format!("*{}-{}", start, trim_end.clone().unwrap_or_default())); }
    if let Some(end) = trim_end { if trim_start.is_none() { args.push("--download-sections".into()); args.push(format!("*0-{}", end)); } }

    match Command::new("yt-dlp").args(&args).status() {
        Ok(status) if status.success() => Ok(()),
        Ok(status) => Err(format!("yt-dlp exited with status: {}", status)),
        Err(e) => Err(format!("Failed to execute yt-dlp: {}", e)),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![download_video])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
