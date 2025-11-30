mod config;
mod secure_storage;
mod library;
mod metadata;
mod playlist;
mod skin;
mod spotify;
mod youtube;
mod performance;
mod error;
mod logging;

#[cfg(test)]
mod error_tests;

use config::{Config, ConfigManager, FileConfigManager};
use secure_storage::{PlatformSecureStorage, SecureStorage};
use library::{LibraryScanner, Track};
use metadata::{MetadataExtractor, TrackMetadata};
use playlist::{PlaylistManager, Playlist, Track as PlaylistTrack};
use skin::{SkinParser, ParsedSkin};
use spotify::{SpotifyBridge, StreamingService, Credentials, Token, TrackMetadata as SpotifyTrackMetadata};
use youtube::YouTubeBridge;
use error::MilkError;
use logging::{log_error, log_warn, log_info, LoggerConfig};
use std::sync::OnceLock;

// Global metadata extractor instance
static METADATA_EXTRACTOR: OnceLock<MetadataExtractor> = OnceLock::new();

fn get_metadata_extractor() -> &'static MetadataExtractor {
    METADATA_EXTRACTOR.get_or_init(|| MetadataExtractor::new())
}

// Global playlist manager instance (lazy initialized)
static PLAYLIST_MANAGER: OnceLock<tokio::sync::Mutex<PlaylistManager>> = OnceLock::new();

async fn get_playlist_manager() -> &'static tokio::sync::Mutex<PlaylistManager> {
    // Use get_or_try_init for async initialization
    if PLAYLIST_MANAGER.get().is_none() {
        let _ = PLAYLIST_MANAGER.set(tokio::sync::Mutex::new(
            PlaylistManager::new().await.expect("Failed to initialize playlist manager")
        ));
    }
    PLAYLIST_MANAGER.get().unwrap()
}

// Global Spotify bridge instance (lazy initialized)
static SPOTIFY_BRIDGE: OnceLock<SpotifyBridge> = OnceLock::new();

fn get_spotify_bridge() -> &'static SpotifyBridge {
    // Lazy initialization - only created when first accessed
    SPOTIFY_BRIDGE.get_or_init(|| {
        eprintln!("Initializing Spotify bridge (lazy)");
        SpotifyBridge::new()
    })
}

// Global YouTube bridge instance (lazy initialized)
static YOUTUBE_BRIDGE: OnceLock<YouTubeBridge> = OnceLock::new();

fn get_youtube_bridge() -> &'static YouTubeBridge {
    // Lazy initialization - only created when first accessed
    YOUTUBE_BRIDGE.get_or_init(|| {
        eprintln!("Initializing YouTube bridge (lazy)");
        YouTubeBridge::new()
    })
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn load_config() -> Result<Config, String> {
    log_info("Config", "Loading configuration");
    match FileConfigManager::load() {
        Ok(config) => {
            log_info("Config", "Configuration loaded successfully");
            Ok(config)
        }
        Err(e) => {
            let milk_err = MilkError::from(e);
            log_error("Config", &format!("Failed to load config: {}", milk_err));
            
            // Return user-friendly error message
            Err(milk_err.user_message())
        }
    }
}

#[tauri::command]
fn is_first_run() -> Result<bool, String> {
    let config_path = FileConfigManager::get_config_path().map_err(|e| e.to_string())?;
    Ok(!config_path.exists())
}

#[tauri::command]
fn validate_directory_path(path: String) -> Result<bool, String> {
    use std::path::Path;
    let dir_path = Path::new(&path);
    
    // Check if path exists
    if !dir_path.exists() {
        return Ok(false);
    }
    
    // Check if it's a directory
    if !dir_path.is_dir() {
        return Ok(false);
    }
    
    // Check if we can read the directory
    match std::fs::read_dir(dir_path) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

#[tauri::command]
fn save_config(config: Config) -> Result<(), String> {
    log_info("Config", "Saving configuration");
    let manager = FileConfigManager;
    match manager.save(&config) {
        Ok(()) => {
            log_info("Config", "Configuration saved successfully");
            Ok(())
        }
        Err(e) => {
            let milk_err = MilkError::from(e);
            log_error("Config", &format!("Failed to save config: {}", milk_err));
            Err(milk_err.user_message())
        }
    }
}

#[tauri::command]
fn store_credential(key: String, value: String) -> Result<(), String> {
    log_info("Storage", &format!("Storing credential: {}", key));
    let storage = PlatformSecureStorage::new();
    match storage.store(&key, &value) {
        Ok(()) => Ok(()),
        Err(e) => {
            let milk_err = MilkError::from(e);
            log_error("Storage", &format!("Failed to store credential: {}", milk_err));
            Err(milk_err.user_message())
        }
    }
}

#[tauri::command]
fn retrieve_credential(key: String) -> Result<Option<String>, String> {
    let storage = PlatformSecureStorage::new();
    match storage.retrieve(&key) {
        Ok(value) => Ok(value),
        Err(e) => {
            let milk_err = MilkError::from(e);
            log_error("Storage", &format!("Failed to retrieve credential: {}", milk_err));
            Err(milk_err.user_message())
        }
    }
}

#[tauri::command]
fn delete_credential(key: String) -> Result<(), String> {
    log_info("Storage", &format!("Deleting credential: {}", key));
    let storage = PlatformSecureStorage::new();
    match storage.delete(&key) {
        Ok(()) => Ok(()),
        Err(e) => {
            let milk_err = MilkError::from(e);
            log_error("Storage", &format!("Failed to delete credential: {}", milk_err));
            Err(milk_err.user_message())
        }
    }
}

#[tauri::command]
fn scan_library(path: String) -> Result<Vec<Track>, String> {
    use std::path::Path;
    log_info("Library", &format!("Scanning library: {}", path));
    let library_path = Path::new(&path);
    match LibraryScanner::scan_directory(library_path) {
        Ok(tracks) => {
            log_info("Library", &format!("Found {} tracks", tracks.len()));
            Ok(tracks)
        }
        Err(e) => {
            let milk_err = MilkError::from(e);
            log_error("Library", &format!("Library scan failed: {}", milk_err));
            Err(milk_err.user_message())
        }
    }
}

#[tauri::command]
fn extract_metadata(file_path: String) -> Result<TrackMetadata, String> {
    use std::path::Path;
    let path = Path::new(&file_path);
    let extractor = get_metadata_extractor();
    match extractor.extract(path) {
        Ok(metadata) => Ok(metadata),
        Err(e) => {
            let milk_err = MilkError::from(e);
            log_warn("Metadata", &format!("Metadata extraction failed for {}: {}", file_path, milk_err));
            // For metadata errors, we still want to return something (fallback will be applied)
            // So we log as warning but still return the error
            Err(milk_err.user_message())
        }
    }
}

#[tauri::command]
fn extract_artwork(file_path: String) -> Result<Option<Vec<u8>>, String> {
    use std::path::Path;
    let path = Path::new(&file_path);
    let extractor = get_metadata_extractor();
    extractor.extract_artwork(path).map_err(|e| e.to_string())
}

#[tauri::command]
async fn create_playlist(name: String) -> Result<Playlist, String> {
    log_info("Playlist", &format!("Creating playlist: {}", name));
    let manager = get_playlist_manager().await;
    let manager = manager.lock().await;
    match manager.create_playlist(name).await {
        Ok(playlist) => {
            log_info("Playlist", &format!("Playlist created: {}", playlist.id));
            Ok(playlist)
        }
        Err(e) => {
            let milk_err = MilkError::from(e);
            log_error("Playlist", &format!("Failed to create playlist: {}", milk_err));
            Err(milk_err.user_message())
        }
    }
}

#[tauri::command]
async fn list_playlists() -> Result<Vec<Playlist>, String> {
    let manager = get_playlist_manager().await;
    let manager = manager.lock().await;
    match manager.list_playlists().await {
        Ok(playlists) => Ok(playlists),
        Err(e) => {
            let milk_err = MilkError::from(e);
            log_error("Playlist", &format!("Failed to list playlists: {}", milk_err));
            Err(milk_err.user_message())
        }
    }
}

#[tauri::command]
async fn load_playlist(playlist_id: String) -> Result<Playlist, String> {
    log_info("Playlist", &format!("Loading playlist: {}", playlist_id));
    let manager = get_playlist_manager().await;
    let manager = manager.lock().await;
    match manager.load_playlist(&playlist_id).await {
        Ok(playlist) => Ok(playlist),
        Err(e) => {
            let milk_err = MilkError::from(e);
            log_error("Playlist", &format!("Failed to load playlist: {}", milk_err));
            Err(milk_err.user_message())
        }
    }
}

#[tauri::command]
async fn delete_playlist(playlist_id: String) -> Result<(), String> {
    log_info("Playlist", &format!("Deleting playlist: {}", playlist_id));
    let manager = get_playlist_manager().await;
    let manager = manager.lock().await;
    match manager.delete_playlist(&playlist_id).await {
        Ok(()) => {
            log_info("Playlist", "Playlist deleted successfully");
            Ok(())
        }
        Err(e) => {
            let milk_err = MilkError::from(e);
            log_error("Playlist", &format!("Failed to delete playlist: {}", milk_err));
            Err(milk_err.user_message())
        }
    }
}

#[tauri::command]
async fn add_track_to_playlist(playlist_id: String, track: PlaylistTrack) -> Result<Playlist, String> {
    log_info("Playlist", &format!("Adding track to playlist: {}", playlist_id));
    let manager = get_playlist_manager().await;
    let manager = manager.lock().await;
    match manager.add_track(&playlist_id, track).await {
        Ok(playlist) => Ok(playlist),
        Err(e) => {
            let milk_err = MilkError::from(e);
            log_error("Playlist", &format!("Failed to add track: {}", milk_err));
            Err(milk_err.user_message())
        }
    }
}

#[tauri::command]
async fn remove_track_from_playlist(playlist_id: String, track_id: String) -> Result<Playlist, String> {
    log_info("Playlist", &format!("Removing track from playlist: {}", playlist_id));
    let manager = get_playlist_manager().await;
    let manager = manager.lock().await;
    match manager.remove_track(&playlist_id, &track_id).await {
        Ok(playlist) => Ok(playlist),
        Err(e) => {
            let milk_err = MilkError::from(e);
            log_error("Playlist", &format!("Failed to remove track: {}", milk_err));
            Err(milk_err.user_message())
        }
    }
}

#[tauri::command]
async fn reorder_playlist_tracks(playlist_id: String, track_ids: Vec<String>) -> Result<Playlist, String> {
    log_info("Playlist", &format!("Reordering tracks in playlist: {}", playlist_id));
    let manager = get_playlist_manager().await;
    let manager = manager.lock().await;
    match manager.reorder_tracks(&playlist_id, track_ids).await {
        Ok(playlist) => Ok(playlist),
        Err(e) => {
            let milk_err = MilkError::from(e);
            log_error("Playlist", &format!("Failed to reorder tracks: {}", milk_err));
            Err(milk_err.user_message())
        }
    }
}

#[tauri::command]
async fn update_playlist(playlist_id: String, name: Option<String>) -> Result<Playlist, String> {
    log_info("Playlist", &format!("Updating playlist: {}", playlist_id));
    let manager = get_playlist_manager().await;
    let manager = manager.lock().await;
    match manager.update_playlist(&playlist_id, name).await {
        Ok(playlist) => Ok(playlist),
        Err(e) => {
            let milk_err = MilkError::from(e);
            log_error("Playlist", &format!("Failed to update playlist: {}", milk_err));
            Err(milk_err.user_message())
        }
    }
}

#[tauri::command]
fn load_skin(skin_path: String) -> Result<ParsedSkin, String> {
    use std::path::Path;
    log_info("Skin", &format!("Loading skin: {}", skin_path));
    let path = Path::new(&skin_path);
    
    // Try to parse as .wsz or .wal
    let result = if skin_path.to_lowercase().ends_with(".wsz") {
        SkinParser::parse_wsz(path)
    } else if skin_path.to_lowercase().ends_with(".wal") {
        SkinParser::parse_wal(path)
    } else {
        let err = MilkError::InvalidSkinFormat(skin_path.clone());
        log_error("Skin", &format!("{}", err));
        return Err(err.user_message());
    };

    match result {
        Ok(skin) => {
            // Validate the skin
            match SkinParser::validate_skin(&skin) {
                Ok(_) => {
                    log_info("Skin", "Skin loaded and validated successfully");
                    Ok(skin)
                }
                Err(e) => {
                    let milk_err = MilkError::from(e);
                    log_warn("Skin", &format!("Skin validation failed, using default: {}", milk_err));
                    Ok(SkinParser::get_default_skin())
                }
            }
        }
        Err(e) => {
            // Return default skin on error (graceful degradation)
            let milk_err = MilkError::from(e);
            log_warn("Skin", &format!("Failed to load skin, using default: {}", milk_err));
            Ok(SkinParser::get_default_skin())
        }
    }
}

#[tauri::command]
fn apply_skin(skin_path: String) -> Result<ParsedSkin, String> {
    use std::path::Path;
    log_info("Skin", &format!("Applying skin: {}", skin_path));
    let path = Path::new(&skin_path);
    
    // Load and validate the skin
    let skin = if skin_path.to_lowercase().ends_with(".wsz") {
        SkinParser::parse_wsz(path)
    } else if skin_path.to_lowercase().ends_with(".wal") {
        SkinParser::parse_wal(path)
    } else {
        let err = MilkError::InvalidSkinFormat(skin_path.clone());
        log_error("Skin", &format!("{}", err));
        return Err(err.user_message());
    };

    match skin {
        Ok(skin) => {
            // Validate the skin
            match SkinParser::validate_skin(&skin) {
                Ok(_) => {
                    // Save the skin path to config
                    let mut config = FileConfigManager::load().unwrap_or_else(|_| FileConfigManager::get_default());
                    config.last_skin = Some(skin_path.clone());
                    if let Err(e) = FileConfigManager.save(&config) {
                        log_warn("Skin", &format!("Failed to save skin preference: {}", e));
                    }
                    log_info("Skin", "Skin applied successfully");
                    Ok(skin)
                }
                Err(e) => {
                    let milk_err = MilkError::from(e);
                    log_warn("Skin", &format!("Skin validation failed, using default: {}", milk_err));
                    Ok(SkinParser::get_default_skin())
                }
            }
        }
        Err(e) => {
            let milk_err = MilkError::from(e);
            log_warn("Skin", &format!("Failed to apply skin, using default: {}", milk_err));
            Ok(SkinParser::get_default_skin())
        }
    }
}

#[tauri::command]
async fn spotify_authenticate(credentials: Credentials, auth_code: String) -> Result<Token, String> {
    log_info("Spotify", "Authenticating with Spotify");
    let bridge = get_spotify_bridge();
    match bridge.authenticate(credentials, auth_code).await {
        Ok(token) => {
            log_info("Spotify", "Authentication successful");
            Ok(token)
        }
        Err(e) => {
            let milk_err = MilkError::from(e);
            log_error("Spotify", &format!("Authentication failed: {}", milk_err));
            Err(milk_err.user_message())
        }
    }
}

#[tauri::command]
async fn spotify_get_now_playing() -> Result<Option<SpotifyTrackMetadata>, String> {
    let bridge = get_spotify_bridge();
    match bridge.get_now_playing().await {
        Ok(metadata) => Ok(metadata),
        Err(e) => {
            // Check error type before converting
            let is_no_playback = matches!(e, spotify::ApiError::NoActivePlayback);
            let milk_err = MilkError::from(e);
            
            // Only log as warning for "no active playback" which is not really an error
            if is_no_playback {
                log_info("Spotify", "No active playback");
            } else {
                log_warn("Spotify", &format!("Failed to get now playing: {}", milk_err));
            }
            Err(milk_err.user_message())
        }
    }
}

#[tauri::command]
async fn spotify_refresh_token(credentials: Credentials) -> Result<Token, String> {
    log_info("Spotify", "Refreshing Spotify token");
    let bridge = get_spotify_bridge();
    match bridge.refresh_token(credentials).await {
        Ok(token) => {
            log_info("Spotify", "Token refreshed successfully");
            Ok(token)
        }
        Err(e) => {
            let milk_err = MilkError::from(e);
            log_error("Spotify", &format!("Token refresh failed: {}", milk_err));
            Err(milk_err.user_message())
        }
    }
}

#[tauri::command]
async fn youtube_authenticate(credentials: Credentials, auth_code: String) -> Result<Token, String> {
    let bridge = get_youtube_bridge();
    bridge.authenticate(credentials, auth_code).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn youtube_get_now_playing() -> Result<Option<SpotifyTrackMetadata>, String> {
    let bridge = get_youtube_bridge();
    bridge.get_now_playing().await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn youtube_refresh_token(credentials: Credentials) -> Result<Token, String> {
    let bridge = get_youtube_bridge();
    bridge.refresh_token(credentials).await.map_err(|e| e.to_string())
}

#[tauri::command]
fn youtube_store_api_key(api_key: String) -> Result<(), String> {
    let bridge = get_youtube_bridge();
    bridge.store_api_key(&api_key).map_err(|e| e.to_string())
}

#[tauri::command]
fn youtube_get_api_key() -> Result<Option<String>, String> {
    let bridge = get_youtube_bridge();
    bridge.get_api_key().map_err(|e| e.to_string())
}

#[tauri::command]
async fn youtube_validate_api_key(api_key: String) -> Result<bool, String> {
    let bridge = get_youtube_bridge();
    bridge.validate_api_key(&api_key).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn youtube_get_video_metadata(video_id: String) -> Result<SpotifyTrackMetadata, String> {
    let bridge = get_youtube_bridge();
    bridge.get_video_metadata(&video_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
fn get_performance_metrics() -> Option<performance::PerformanceMetrics> {
    performance::get_metrics()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    use std::time::Instant;
    
    // Initialize logging system
    let log_config = LoggerConfig::default();
    if let Err(e) = logging::init_logger(log_config) {
        eprintln!("Failed to initialize logger: {}", e);
    }
    
    log_info("Startup", "milk application starting");
    
    // Start tracking startup time
    let startup_start = Instant::now();
    performance::init_performance_tracking();
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(move |_app| {
            // Record startup time once the app is ready
            let startup_duration = startup_start.elapsed();
            performance::record_startup_time(startup_duration);
            log_info("Startup", &format!("Application ready in {:?}", startup_duration));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            load_config,
            save_config,
            is_first_run,
            validate_directory_path,
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
            update_playlist,
            load_skin,
            apply_skin,
            spotify_authenticate,
            spotify_get_now_playing,
            spotify_refresh_token,
            youtube_authenticate,
            youtube_get_now_playing,
            youtube_refresh_token,
            youtube_store_api_key,
            youtube_get_api_key,
            youtube_validate_api_key,
            youtube_get_video_metadata,
            get_performance_metrics
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
