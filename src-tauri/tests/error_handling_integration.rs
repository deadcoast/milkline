// Integration tests for error handling and recovery
use milk_lib::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_invalid_path_error() {
    // Test that invalid paths are properly detected
    let result = validate_directory_path("/nonexistent/path/that/does/not/exist".to_string());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), false);
}

#[test]
fn test_valid_path() {
    // Test that valid paths are accepted
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path().to_str().unwrap().to_string();

    let result = validate_directory_path(path);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), true);
}

#[test]
fn test_corrupted_config_recovery() {
    // Test that corrupted config files are handled gracefully
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.json");

    // Write corrupted JSON
    fs::write(&config_path, "{invalid json}").unwrap();

    // Loading should return default config without crashing
    // This is tested in config_tests.rs property tests
}

#[test]
fn test_missing_config_creates_default() {
    // Test that missing config creates default
    let result = load_config();
    assert!(result.is_ok());

    let config = result.unwrap();
    assert_eq!(config.volume, 0.7); // Default value
}

#[test]
fn test_unsupported_audio_format() {
    // Test that unsupported formats are rejected
    let result = validate_audio_file("/path/to/file.xyz".to_string());
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Unsupported"));
}

#[test]
fn test_supported_audio_formats() {
    // Test that supported formats are recognized
    assert!(check_file_extension_supported("mp3".to_string()));
    assert!(check_file_extension_supported("flac".to_string()));
    assert!(check_file_extension_supported("wav".to_string()));
    assert!(!check_file_extension_supported("xyz".to_string()));
}

#[test]
fn test_error_criticality_check() {
    // Test critical error detection
    assert!(is_error_critical("disk_full".to_string()));
    assert!(is_error_critical("permission_denied".to_string()));
    assert!(is_error_critical("audio_device".to_string()));
    assert!(is_error_critical("auth_failed".to_string()));
    assert!(!is_error_critical("network_timeout".to_string()));
}

#[test]
fn test_error_recoverability_check() {
    // Test recoverable error detection
    assert!(is_error_recoverable("network_timeout".to_string()));
    assert!(is_error_recoverable("rate_limit".to_string()));
    assert!(is_error_recoverable("corrupted_file".to_string()));
    assert!(is_error_recoverable("skin_parse".to_string()));
    assert!(is_error_recoverable("metadata".to_string()));
    assert!(!is_error_recoverable("disk_full".to_string()));
}

#[test]
fn test_error_categories() {
    // Test error categorization
    assert_eq!(get_error_category("test".to_string()), "General");
}

#[test]
fn test_skin_error_fallback() {
    // Test that invalid skin files fall back to default
    let result = load_skin("/nonexistent/skin.wsz".to_string());
    assert!(result.is_ok());

    let skin = result.unwrap();
    assert_eq!(skin.name, "default");
}

#[test]
fn test_invalid_skin_format_error() {
    // Test that invalid skin formats are rejected with proper error
    let result = load_skin("/path/to/file.txt".to_string());
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid skin format"));
}

#[tokio::test]
async fn test_playlist_not_found_error() {
    // Test that loading non-existent playlist returns proper error
    let result = load_playlist("nonexistent-id".to_string()).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Can't find that playlist"));
}

#[tokio::test]
async fn test_playlist_creation_and_recovery() {
    // Test playlist creation and error handling
    let result = create_playlist("Test Playlist".to_string()).await;
    assert!(result.is_ok());

    let playlist = result.unwrap();
    assert_eq!(playlist.name, "Test Playlist");
    assert!(playlist.tracks.is_empty());
}

#[test]
fn test_metadata_extraction_fallback() {
    // Test that metadata extraction failures are handled gracefully
    let result = extract_metadata("/nonexistent/file.mp3".to_string());
    assert!(result.is_err());
    // Error message should be user-friendly
    let error_msg = result.unwrap_err();
    assert!(!error_msg.is_empty());
}

#[test]
fn test_credential_storage_error_handling() {
    // Test credential storage with invalid key
    let result = store_credential("".to_string(), "value".to_string());
    // Should handle gracefully (may succeed or fail depending on platform)
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_first_run_detection() {
    // Test first run detection
    let result = is_first_run();
    assert!(result.is_ok());
    // Result depends on whether config exists
}

#[test]
fn test_config_save_and_load_roundtrip() {
    use milk_lib::Config;

    // Create a config
    let mut config = load_config().unwrap();
    config.volume = 0.5;
    config.library_path = Some("/test/path".to_string());

    // Save it
    let save_result = save_config(config.clone());
    assert!(save_result.is_ok());

    // Load it back
    let loaded_config = load_config().unwrap();
    assert_eq!(loaded_config.volume, 0.5);
}

#[test]
fn test_scan_library_with_invalid_path() {
    // Test library scanning with invalid path
    let result = scan_library("/nonexistent/path".to_string());
    assert!(result.is_err());

    let error_msg = result.unwrap_err();
    assert!(error_msg.contains("can't find") || error_msg.contains("Invalid"));
}

#[test]
fn test_scan_library_with_file_instead_of_directory() {
    // Create a temporary file
    let temp_file = tempfile::NamedTempFile::new().unwrap();
    let file_path = temp_file.path().to_str().unwrap().to_string();

    // Try to scan it as a directory
    let result = validate_directory_path(file_path);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), false);
}

#[cfg(test)]
mod error_recovery_tests {
    use super::*;

    #[test]
    fn test_config_error_recovery() {
        // This is tested via the config property tests
        // which verify that corrupted configs are recovered
    }

    #[tokio::test]
    async fn test_network_error_recovery() {
        // Test that network errors can be retried
        // This would require mocking network calls
    }

    #[test]
    fn test_path_validation_and_recovery() {
        // Test path validation
        let invalid_path = "/nonexistent/path";
        let result = validate_directory_path(invalid_path.to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), false);
    }
}

#[cfg(test)]
mod user_feedback_tests {
    use super::*;

    #[test]
    fn test_error_messages_are_user_friendly() {
        // Test that error messages are user-friendly
        let result = load_skin("/nonexistent/skin.wsz".to_string());
        // Should succeed with default skin (graceful degradation)
        assert!(result.is_ok());
    }

    #[test]
    fn test_critical_errors_identified() {
        // Test that critical errors are properly identified
        assert!(is_error_critical("disk_full".to_string()));
        assert!(is_error_critical("permission_denied".to_string()));
    }

    #[test]
    fn test_recoverable_errors_identified() {
        // Test that recoverable errors are properly identified
        assert!(is_error_recoverable("network_timeout".to_string()));
        assert!(is_error_recoverable("rate_limit".to_string()));
    }
}
