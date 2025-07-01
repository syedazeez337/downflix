#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


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
