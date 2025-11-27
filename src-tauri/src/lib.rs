mod config;
mod secure_storage;
mod library;

use config::{Config, ConfigManager, FileConfigManager};
use secure_storage::{PlatformSecureStorage, SecureStorage};
use library::{LibraryScanner, Track};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn load_config() -> Result<Config, String> {
    FileConfigManager::load().map_err(|e| e.to_string())
}

#[tauri::command]
fn save_config(config: Config) -> Result<(), String> {
    let manager = FileConfigManager;
    manager.save(&config).map_err(|e| e.to_string())
}

#[tauri::command]
fn store_credential(key: String, value: String) -> Result<(), String> {
    let storage = PlatformSecureStorage::new();
    storage.store(&key, &value).map_err(|e| e.to_string())
}

#[tauri::command]
fn retrieve_credential(key: String) -> Result<Option<String>, String> {
    let storage = PlatformSecureStorage::new();
    storage.retrieve(&key).map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_credential(key: String) -> Result<(), String> {
    let storage = PlatformSecureStorage::new();
    storage.delete(&key).map_err(|e| e.to_string())
}

#[tauri::command]
fn scan_library(path: String) -> Result<Vec<Track>, String> {
    use std::path::Path;
    let library_path = Path::new(&path);
    LibraryScanner::scan_directory(library_path).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            load_config,
            save_config,
            store_credential,
            retrieve_credential,
            delete_credential,
            scan_library
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
