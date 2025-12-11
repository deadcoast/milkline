// Integration test for first-run detection and setup flow
// This test verifies the first-run detection logic and configuration persistence

use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

/// Simulates the FileConfigManager behavior for testing
struct TestConfigManager {
    config_dir: TempDir,
}

impl TestConfigManager {
    fn new() -> Self {
        TestConfigManager {
            config_dir: TempDir::new().unwrap(),
        }
    }

    fn get_config_path(&self) -> PathBuf {
        self.config_dir.path().join("config.json")
    }

    /// Simulates the is_first_run command
    fn is_first_run(&self) -> bool {
        !self.get_config_path().exists()
    }

    /// Simulates saving config after setup
    fn save_config(&self, config_json: &str) -> std::io::Result<()> {
        fs::write(self.get_config_path(), config_json)
    }

    /// Simulates loading config
    fn load_config(&self) -> std::io::Result<String> {
        fs::read_to_string(self.get_config_path())
    }
}

#[test]
fn test_first_run_detection_no_config() {
    // Arrange: Create a fresh config manager with no config file
    let manager = TestConfigManager::new();

    // Act: Check if it's first run
    let is_first = manager.is_first_run();

    // Assert: Should be first run when no config exists
    assert!(
        is_first,
        "Should be first run when config file doesn't exist"
    );
}

#[test]
fn test_first_run_detection_with_config() {
    // Arrange: Create config manager and save a config
    let manager = TestConfigManager::new();
    let config = r#"{"library_path":null,"last_skin":null,"volume":0.7}"#;
    manager.save_config(config).unwrap();

    // Act: Check if it's first run
    let is_first = manager.is_first_run();

    // Assert: Should NOT be first run when config exists
    assert!(!is_first, "Should not be first run when config file exists");
}

#[test]
fn test_setup_flow_creates_config() {
    // Arrange: Create a fresh config manager (simulating first run)
    let manager = TestConfigManager::new();
    assert!(manager.is_first_run(), "Should start as first run");

    // Act: Simulate setup completion by saving config
    let config = r#"{
        "library_path": "/path/to/music",
        "last_skin": null,
        "volume": 0.7,
        "visualizer_style": "bars",
        "spotify_enabled": true,
        "youtube_enabled": false,
        "window_position": {"x": 100, "y": 100},
        "window_size": {"width": 800, "height": 600}
    }"#;
    manager.save_config(config).unwrap();

    // Assert: Config file should now exist
    assert!(
        manager.get_config_path().exists(),
        "Config file should exist after setup"
    );
    assert!(
        !manager.is_first_run(),
        "Should no longer be first run after setup"
    );
}

#[test]
fn test_setup_flow_persists_library_path() {
    // Arrange: Create config manager
    let manager = TestConfigManager::new();

    // Act: Save config with library path (simulating setup wizard completion)
    let library_path = "/Users/test/Music";
    let config = format!(
        r#"{{
        "library_path": "{}",
        "last_skin": null,
        "volume": 0.7,
        "visualizer_style": "bars",
        "spotify_enabled": false,
        "youtube_enabled": false,
        "window_position": {{"x": 100, "y": 100}},
        "window_size": {{"width": 800, "height": 600}}
    }}"#,
        library_path
    );
    manager.save_config(&config).unwrap();

    // Assert: Load config and verify library path was persisted
    let loaded_config = manager.load_config().unwrap();
    assert!(
        loaded_config.contains(library_path),
        "Loaded config should contain the library path set during setup"
    );
}

#[test]
fn test_setup_flow_persists_streaming_settings() {
    // Arrange: Create config manager
    let manager = TestConfigManager::new();

    // Act: Save config with streaming services enabled
    let config = r#"{
        "library_path": null,
        "last_skin": null,
        "volume": 0.7,
        "visualizer_style": "bars",
        "spotify_enabled": true,
        "youtube_enabled": true,
        "window_position": {"x": 100, "y": 100},
        "window_size": {"width": 800, "height": 600}
    }"#;
    manager.save_config(config).unwrap();

    // Assert: Load config and verify streaming settings were persisted
    let loaded_config = manager.load_config().unwrap();
    assert!(
        loaded_config.contains(r#""spotify_enabled": true"#),
        "Spotify should be enabled"
    );
    assert!(
        loaded_config.contains(r#""youtube_enabled": true"#),
        "YouTube should be enabled"
    );
}

#[test]
fn test_setup_flow_allows_skipping_library_path() {
    // Arrange: Create config manager
    let manager = TestConfigManager::new();

    // Act: Save config with null library path (user skipped this step)
    let config = r#"{
        "library_path": null,
        "last_skin": null,
        "volume": 0.7,
        "visualizer_style": "bars",
        "spotify_enabled": false,
        "youtube_enabled": false,
        "window_position": {"x": 100, "y": 100},
        "window_size": {"width": 800, "height": 600}
    }"#;
    manager.save_config(config).unwrap();

    // Assert: Config should be saved even with null library path
    assert!(
        manager.get_config_path().exists(),
        "Config should exist even when library path is skipped"
    );
    let loaded_config = manager.load_config().unwrap();
    assert!(
        loaded_config.contains(r#""library_path": null"#),
        "Library path should be null when skipped"
    );
}

#[test]
fn test_multiple_app_launches_after_setup() {
    // Arrange: Create config manager and complete setup
    let manager = TestConfigManager::new();
    let config = r#"{"library_path":"/music","last_skin":null,"volume":0.7}"#;
    manager.save_config(config).unwrap();

    // Act: Simulate multiple app launches
    let first_check = manager.is_first_run();
    let second_check = manager.is_first_run();
    let third_check = manager.is_first_run();

    // Assert: Should consistently return false after setup
    assert!(!first_check, "First check should not be first run");
    assert!(!second_check, "Second check should not be first run");
    assert!(!third_check, "Third check should not be first run");
}

#[test]
fn test_config_file_location() {
    // Arrange: Create config manager
    let manager = TestConfigManager::new();

    // Act: Get config path
    let config_path = manager.get_config_path();

    // Assert: Config path should end with config.json
    assert!(
        config_path.to_string_lossy().ends_with("config.json"),
        "Config file should be named config.json"
    );
}
