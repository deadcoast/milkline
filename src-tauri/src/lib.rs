mod config;
mod secure_storage;
mod library;
mod metadata;
mod playlist;

use config::{Config, ConfigManager, FileConfigManager};
use secure_storage::{PlatformSecureStorage, SecureStorage};
use library::{LibraryScanner, Track};
use metadata::{MetadataExtractor, TrackMetadata};
use playlist::{PlaylistManager, Playlist, Track as PlaylistTrack};
use std::sync::{OnceLock, Mutex};

// Global metadata extractor instance
static METADATA_EXTRACTOR: OnceLock<MetadataExtractor> = OnceLock::new();

fn get_metadata_extractor() -> &'static MetadataExtractor {
    METADATA_EXTRACTOR.get_or_init(|| MetadataExtractor::new())
}

// Global playlist manager instance
static PLAYLIST_MANAGER: OnceLock<Mutex<PlaylistManager>> = OnceLock::new();

fn get_playlist_manager() -> &'static Mutex<PlaylistManager> {
    PLAYLIST_MANAGER.get_or_init(|| {
        Mutex::new(PlaylistManager::new().expect("Failed to initialize playlist manager"))
    })
}

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

#[tauri::command]
fn extract_metadata(file_path: String) -> Result<TrackMetadata, String> {
    use std::path::Path;
    let path = Path::new(&file_path);
    let extractor = get_metadata_extractor();
    extractor.extract(path).map_err(|e| e.to_string())
}

#[tauri::command]
fn extract_artwork(file_path: String) -> Result<Option<Vec<u8>>, String> {
    use std::path::Path;
    let path = Path::new(&file_path);
    let extractor = get_metadata_extractor();
    extractor.extract_artwork(path).map_err(|e| e.to_string())
}

#[tauri::command]
fn create_playlist(name: String) -> Result<Playlist, String> {
    let manager = get_playlist_manager().lock().unwrap();
    manager.create_playlist(name).map_err(|e| e.to_string())
}

#[tauri::command]
fn list_playlists() -> Result<Vec<Playlist>, String> {
    let manager = get_playlist_manager().lock().unwrap();
    manager.list_playlists().map_err(|e| e.to_string())
}

#[tauri::command]
fn load_playlist(playlist_id: String) -> Result<Playlist, String> {
    let manager = get_playlist_manager().lock().unwrap();
    manager.load_playlist(&playlist_id).map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_playlist(playlist_id: String) -> Result<(), String> {
    let manager = get_playlist_manager().lock().unwrap();
    manager.delete_playlist(&playlist_id).map_err(|e| e.to_string())
}

#[tauri::command]
fn add_track_to_playlist(playlist_id: String, track: PlaylistTrack) -> Result<Playlist, String> {
    let manager = get_playlist_manager().lock().unwrap();
    manager.add_track(&playlist_id, track).map_err(|e| e.to_string())
}

#[tauri::command]
fn remove_track_from_playlist(playlist_id: String, track_id: String) -> Result<Playlist, String> {
    let manager = get_playlist_manager().lock().unwrap();
    manager.remove_track(&playlist_id, &track_id).map_err(|e| e.to_string())
}

#[tauri::command]
fn reorder_playlist_tracks(playlist_id: String, track_ids: Vec<String>) -> Result<Playlist, String> {
    let manager = get_playlist_manager().lock().unwrap();
    manager.reorder_tracks(&playlist_id, track_ids).map_err(|e| e.to_string())
}

#[tauri::command]
fn update_playlist(playlist_id: String, name: Option<String>) -> Result<Playlist, String> {
    let manager = get_playlist_manager().lock().unwrap();
    manager.update_playlist(&playlist_id, name).map_err(|e| e.to_string())
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
            scan_library,
            extract_metadata,
            extract_artwork,
            create_playlist,
            list_playlists,
            load_playlist,
            delete_playlist,
            add_track_to_playlist,
            remove_track_from_playlist,
            reorder_playlist_tracks,
            update_playlist
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
