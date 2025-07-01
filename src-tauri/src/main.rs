#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{command, Manager, Window};
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use regex::Regex;

#[command]
fn download_video(window: Window, url: String, format: String, trim_start: Option<String>, trim_end: Option<String>) -> Result<(), String> {
    let mut args = vec![url, "-f".into(), format, "--newline".into()];

    if let Some(start) = trim_start {
        args.push("--download-sections".into());
        args.push(format!("*{}-{}", start, trim_end.clone().unwrap_or_default()));
    }
    if let Some(end) = trim_end {
        if trim_start.is_none() {
            args.push("--download-sections".into());
            args.push(format!("*0-{}", end));
        }
    }

    let mut child = Command::new("yt-dlp")
        .args(&args)
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn yt-dlp: {}", e))?;

    if let Some(stdout) = child.stdout.take() {
        let win = window.clone();
        std::thread::spawn(move || {
            let reader = BufReader::new(stdout);
            let re = Regex::new(r"(\d+(?:\.\d+)?)%").unwrap();
            for line in reader.lines().flatten() {
                if let Some(caps) = re.captures(&line) {
                    if let Ok(p) = caps[1].parse::<f32>() {
                        let _ = win.emit("progress", p);
                    }
                }
            }
        });
    }

    match child.wait() {
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
