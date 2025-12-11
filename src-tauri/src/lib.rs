mod config;
mod error;
mod error_recovery;
mod library;
mod logging;
pub mod media_editor;
mod metadata;
pub mod performance;
mod playlist;
mod secure_storage;
mod skin;
mod spotify;
mod system_audio;
mod youtube;

#[cfg(test)]
mod error_tests;

#[cfg(test)]
mod config_tests;

use config::{Config, ConfigManager, FileConfigManager};
use error::{MilkError, MilkResult};
use library::{LibraryScanner, Track};
use logging::{log_error, log_error_with_context, log_info, log_warn, LoggerConfig};
use media_editor::image_ops::crop_image_command;
use media_editor::video_ops::{probe_video_metadata_command, trim_and_crop_video_command};
use metadata::{MetadataExtractor, TrackMetadata};
use performance::Timer;
use playlist::{Playlist, PlaylistManager, Track as PlaylistTrack};
use secure_storage::{PlatformSecureStorage, SecureStorage};
use skin::{ParsedSkin, SkinParser};
use spotify::{
    Credentials, SpotifyBridge, StreamingService, Token, TrackMetadata as SpotifyTrackMetadata,
};
use std::sync::{Arc, Mutex, OnceLock};
use system_audio::{
    is_system_audio_capture_active, start_system_audio_capture, stop_system_audio_capture,
    SystemAudioCapture,
};
use tauri::Emitter;
use youtube::YouTubeBridge;

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
            PlaylistManager::new()
                .await
                .expect("Failed to initialize playlist manager"),
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
            log_error(
                "Storage",
                &format!("Failed to store credential: {}", milk_err),
            );
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
            log_error(
                "Storage",
                &format!("Failed to retrieve credential: {}", milk_err),
            );
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
            log_error(
                "Storage",
                &format!("Failed to delete credential: {}", milk_err),
            );
            Err(milk_err.user_message())
        }
    }
}

/// Helper function using MilkResult to scan library with performance tracking
fn scan_library_with_timing(path: &std::path::Path) -> MilkResult<Vec<Track>> {
    let _timer = Timer::new(format!("Library scan: {}", path.display()));
    LibraryScanner::scan_directory(path).map_err(MilkError::from)
}

/// Validate audio file format (constructs DecodeError and UnsupportedFormat variants)
fn validate_audio_format(file_path: &std::path::Path) -> MilkResult<()> {
    let extension = file_path
        .extension()
        .and_then(|e| e.to_str())
        .ok_or_else(|| MilkError::UnsupportedFormat("unknown".to_string()))?;

    // Check if extension is supported
    if !LibraryScanner::is_supported_extension(extension) {
        return Err(MilkError::UnsupportedFormat(extension.to_string()));
    }

    // In a real implementation, we might verify the file header
    // For now, we just check if the file exists and is readable
    if !file_path.exists() {
        return Err(MilkError::DecodeError("File does not exist".to_string()));
    }

    Ok(())
}

/// Load and validate config (constructs InvalidConfig, MissingConfig variants)
fn load_and_validate_config() -> MilkResult<Config> {
    let config = match FileConfigManager::load() {
        Ok(c) => c,
        Err(_) => {
            // If config doesn't exist, that's MissingConfig
            return Err(MilkError::MissingConfig("configuration file".to_string()));
        }
    };

    // Validate required fields
    if config.library_path.is_none() {
        return Err(MilkError::MissingConfig("library_path".to_string()));
    }

    // Validate library path exists
    if let Some(ref path) = config.library_path {
        if !std::path::Path::new(path).exists() {
            return Err(MilkError::InvalidConfig(format!(
                "library_path: {} does not exist",
                path
            )));
        }
    }

    Ok(config)
}

/// Generic error handler that can construct Internal error variant
fn handle_unexpected_error<T>(result: Result<T, Box<dyn std::error::Error>>) -> MilkResult<T> {
    result.map_err(|e| MilkError::Internal(format!("Unexpected error: {}", e)))
}

#[tauri::command]
fn scan_library(path: String) -> Result<Vec<Track>, String> {
    use std::path::Path;
    log_info("Library", &format!("Scanning library: {}", path));
    let library_path = Path::new(&path);

    match scan_library_with_timing(library_path) {
        Ok(tracks) => {
            log_info("Library", &format!("Found {} tracks", tracks.len()));
            Ok(tracks)
        }
        Err(e) => {
            log_error_with_context("Library", &e, "Failed to scan library");
            Err(e.user_message())
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
            log_warn(
                "Metadata",
                &format!("Metadata extraction failed for {}: {}", file_path, milk_err),
            );
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
            log_error(
                "Playlist",
                &format!("Failed to create playlist: {}", milk_err),
            );
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
            log_error(
                "Playlist",
                &format!("Failed to list playlists: {}", milk_err),
            );
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
            log_error(
                "Playlist",
                &format!("Failed to load playlist: {}", milk_err),
            );
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
            log_error(
                "Playlist",
                &format!("Failed to delete playlist: {}", milk_err),
            );
            Err(milk_err.user_message())
        }
    }
}

#[tauri::command]
async fn add_track_to_playlist(
    playlist_id: String,
    track: PlaylistTrack,
) -> Result<Playlist, String> {
    log_info(
        "Playlist",
        &format!("Adding track to playlist: {}", playlist_id),
    );
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
async fn remove_track_from_playlist(
    playlist_id: String,
    track_id: String,
) -> Result<Playlist, String> {
    log_info(
        "Playlist",
        &format!("Removing track from playlist: {}", playlist_id),
    );
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
async fn reorder_playlist_tracks(
    playlist_id: String,
    track_ids: Vec<String>,
) -> Result<Playlist, String> {
    log_info(
        "Playlist",
        &format!("Reordering tracks in playlist: {}", playlist_id),
    );
    let manager = get_playlist_manager().await;
    let manager = manager.lock().await;
    match manager.reorder_tracks(&playlist_id, track_ids).await {
        Ok(playlist) => Ok(playlist),
        Err(e) => {
            let milk_err = MilkError::from(e);
            log_error(
                "Playlist",
                &format!("Failed to reorder tracks: {}", milk_err),
            );
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
            log_error(
                "Playlist",
                &format!("Failed to update playlist: {}", milk_err),
            );
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
                    log_warn(
                        "Skin",
                        &format!("Skin validation failed, using default: {}", milk_err),
                    );
                    Ok(SkinParser::get_default_skin())
                }
            }
        }
        Err(e) => {
            // Return default skin on error (graceful degradation)
            let milk_err = MilkError::from(e);
            log_warn(
                "Skin",
                &format!("Failed to load skin, using default: {}", milk_err),
            );
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
                    let mut config = FileConfigManager::load()
                        .unwrap_or_else(|_| FileConfigManager::get_default());
                    config.last_skin = Some(skin_path.clone());
                    if let Err(e) = FileConfigManager.save(&config) {
                        log_warn("Skin", &format!("Failed to save skin preference: {}", e));
                    }
                    log_info("Skin", "Skin applied successfully");
                    Ok(skin)
                }
                Err(e) => {
                    let milk_err = MilkError::from(e);
                    log_warn(
                        "Skin",
                        &format!("Skin validation failed, using default: {}", milk_err),
                    );
                    Ok(SkinParser::get_default_skin())
                }
            }
        }
        Err(e) => {
            let milk_err = MilkError::from(e);
            log_warn(
                "Skin",
                &format!("Failed to apply skin, using default: {}", milk_err),
            );
            Ok(SkinParser::get_default_skin())
        }
    }
}

#[tauri::command]
async fn spotify_authenticate(
    credentials: Credentials,
    auth_code: String,
) -> Result<Token, String> {
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
                log_warn(
                    "Spotify",
                    &format!("Failed to get now playing: {}", milk_err),
                );
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
fn spotify_check_token_expired() -> Result<bool, String> {
    let bridge = get_spotify_bridge();
    bridge.check_token_expired().map_err(|e| e.to_string())
}

#[tauri::command]
async fn spotify_ensure_valid_token(credentials: Option<Credentials>) -> Result<String, String> {
    let bridge = get_spotify_bridge();
    bridge
        .ensure_valid_token(credentials)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn youtube_authenticate(
    credentials: Credentials,
    auth_code: String,
) -> Result<Token, String> {
    let bridge = get_youtube_bridge();
    bridge
        .authenticate(credentials, auth_code)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn youtube_get_now_playing() -> Result<Option<SpotifyTrackMetadata>, String> {
    let bridge = get_youtube_bridge();
    bridge.get_now_playing().await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn youtube_refresh_token(credentials: Credentials) -> Result<Token, String> {
    let bridge = get_youtube_bridge();
    bridge
        .refresh_token(credentials)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn youtube_check_token_expired() -> Result<bool, String> {
    let bridge = get_youtube_bridge();
    bridge.check_token_expired().map_err(|e| e.to_string())
}

#[tauri::command]
async fn youtube_ensure_valid_token(credentials: Option<Credentials>) -> Result<String, String> {
    let bridge = get_youtube_bridge();
    bridge
        .ensure_valid_token(credentials)
        .await
        .map_err(|e| e.to_string())
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
    bridge
        .validate_api_key(&api_key)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn youtube_get_video_metadata(video_id: String) -> Result<SpotifyTrackMetadata, String> {
    let bridge = get_youtube_bridge();
    bridge
        .get_video_metadata(&video_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_performance_metrics() -> Option<performance::PerformanceMetrics> {
    performance::get_metrics()
}

#[tauri::command]
fn get_cache_hit_rate() -> f64 {
    if let Some(metrics) = performance::get_metrics() {
        metrics.cache_hit_rate()
    } else {
        0.0
    }
}

#[tauri::command]
fn get_memory_usage() -> Option<f64> {
    performance::get_metrics().and_then(|m| m.memory_usage_mb())
}

#[tauri::command]
fn get_peak_memory() -> Option<f64> {
    performance::get_metrics().and_then(|m| m.peak_memory_mb())
}

#[tauri::command]
fn check_metadata_completeness(file_path: String) -> Result<bool, String> {
    use std::path::Path;
    let path = Path::new(&file_path);
    let extractor = get_metadata_extractor();

    match extractor.extract(path) {
        Ok(metadata) => Ok(metadata.is_complete()),
        Err(e) => {
            let milk_err = MilkError::from(e);
            Err(milk_err.user_message())
        }
    }
}

#[tauri::command]
fn is_metadata_cached(file_path: String) -> bool {
    use std::path::Path;
    let path = Path::new(&file_path);
    let extractor = get_metadata_extractor();
    extractor.is_cached(path)
}

#[tauri::command]
fn clear_metadata_cache() {
    log_info("Metadata", "Clearing metadata cache");
    let extractor = get_metadata_extractor();
    extractor.clear_cache();
}

#[tauri::command]
fn check_file_extension_supported(extension: String) -> bool {
    LibraryScanner::is_supported_extension(&extension)
}

#[tauri::command]
fn validate_audio_file(file_path: String) -> Result<(), String> {
    use std::path::Path;
    let path = Path::new(&file_path);
    validate_audio_format(path).map_err(|e| e.user_message())
}

#[tauri::command]
fn load_validated_config() -> Result<Config, String> {
    load_and_validate_config().map_err(|e| e.user_message())
}

#[tauri::command]
fn test_internal_error_handling() -> Result<String, String> {
    // Example of using handle_unexpected_error
    let result: Result<String, Box<dyn std::error::Error>> = Ok("test".to_string());
    handle_unexpected_error(result).map_err(|e| e.user_message())
}

#[tauri::command]
fn get_skin_assets(
    skin_path: String,
) -> Result<std::collections::HashMap<String, Vec<u8>>, String> {
    use std::path::Path;
    let path = Path::new(&skin_path);

    let skin = if skin_path.to_lowercase().ends_with(".wsz") {
        SkinParser::parse_wsz(path)
    } else if skin_path.to_lowercase().ends_with(".wal") {
        SkinParser::parse_wal(path)
    } else {
        return Err("Invalid skin format".to_string());
    };

    match skin {
        Ok(skin) => match SkinParser::extract_assets(&skin) {
            Ok(assets) => Ok(assets),
            Err(e) => Err(e.to_string()),
        },
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
fn get_error_category(error_msg: String) -> String {
    // Create a generic error to demonstrate category usage
    let error = MilkError::Other(error_msg);
    error.category().to_string()
}

#[tauri::command]
fn is_error_critical(error_type: String) -> bool {
    // Map common error types to check criticality
    let error = match error_type.as_str() {
        "disk_full" => MilkError::DiskFull("test".to_string()),
        "permission_denied" => MilkError::PermissionDenied("test".to_string()),
        "audio_device" => MilkError::AudioDeviceUnavailable,
        "auth_failed" => MilkError::AuthenticationFailed("test".to_string()),
        _ => MilkError::Other(error_type),
    };
    error.is_critical()
}

#[tauri::command]
fn is_error_recoverable(error_type: String) -> bool {
    let error = match error_type.as_str() {
        "network_timeout" => MilkError::NetworkTimeout("test".to_string()),
        "rate_limit" => MilkError::RateLimitExceeded,
        "corrupted_file" => MilkError::CorruptedFile("test".to_string()),
        "skin_parse" => MilkError::SkinParseError("test".to_string()),
        "metadata" => MilkError::MetadataError("test".to_string()),
        _ => MilkError::Other(error_type),
    };
    error.is_recoverable()
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
        .setup(move |app| {
            // Record startup time once the app is ready
            let startup_duration = startup_start.elapsed();
            performance::record_startup_time(startup_duration);
            log_info(
                "Startup",
                &format!("Application ready in {:?}", startup_duration),
            );

            // Handle command-line arguments for file associations
            if let Some(args) = std::env::args().nth(1) {
                log_info(
                    "FileAssociation",
                    &format!("Received file argument: {}", args),
                );

                // Check if it's a skin file
                if args.to_lowercase().ends_with(".wsz") || args.to_lowercase().ends_with(".wal") {
                    log_info(
                        "FileAssociation",
                        "Detected skin file, will load on frontend",
                    );

                    // Emit event to frontend to load the skin
                    let app_handle = app.handle().clone();
                    tauri::async_runtime::spawn(async move {
                        if let Err(e) = app_handle.emit("load-skin-file", args) {
                            log_error(
                                "FileAssociation",
                                &format!("Failed to emit load-skin-file event: {}", e),
                            );
                        }
                    });
                }
            }

            Ok(())
        })
        .manage(system_audio::SystemAudioCaptureState(Arc::new(Mutex::new(
            SystemAudioCapture::new(),
        ))))
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
            check_metadata_completeness,
            is_metadata_cached,
            clear_metadata_cache,
            check_file_extension_supported,
            validate_audio_file,
            load_validated_config,
            test_internal_error_handling,
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
            get_skin_assets,
            spotify_authenticate,
            spotify_get_now_playing,
            spotify_refresh_token,
            spotify_check_token_expired,
            spotify_ensure_valid_token,
            youtube_authenticate,
            youtube_get_now_playing,
            youtube_refresh_token,
            youtube_check_token_expired,
            youtube_ensure_valid_token,
            youtube_store_api_key,
            youtube_get_api_key,
            youtube_validate_api_key,
            youtube_get_video_metadata,
            get_performance_metrics,
            get_cache_hit_rate,
            get_memory_usage,
            get_peak_memory,
            get_error_category,
            is_error_critical,
            is_error_recoverable,
            crop_image_command,
            probe_video_metadata_command,
            trim_and_crop_video_command,
            start_system_audio_capture,
            stop_system_audio_capture,
            is_system_audio_capture_active
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
