use futures_util::TryFutureExt;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod metruyenchu;

use metruyenchu::download::download_truyencv;
use tauri::{AppHandle, Manager};

#[tauri::command]
async fn start_download(app: AppHandle, base_url: String) -> Result<(), String> {
    download_truyencv(app, &base_url)
        .await
        .map_err(|e| e.to_string())
}
use lazy_static::lazy_static;
use std::sync::atomic::{AtomicBool, Ordering};
lazy_static! {
    static ref STOP_FLAG: AtomicBool = AtomicBool::new(false);
}

#[tauri::command]
async fn stop_download() {
    STOP_FLAG.store(true, Ordering::SeqCst);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![start_download, stop_download])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
