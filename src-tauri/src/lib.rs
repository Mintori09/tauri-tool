use futures_util::TryFutureExt;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod metruyenchu;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use metruyenchu::download::download_truyencv;
use tauri::{AppHandle, Manager};

#[tauri::command]
async fn start_download(app: AppHandle, base_url: String) -> Result<(), String> {
    download_truyencv(app, &base_url)
        .await
        .map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![start_download])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
