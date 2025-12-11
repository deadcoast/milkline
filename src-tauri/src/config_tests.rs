// Integration tests for first-run detection and setup flow
#[cfg(test)]
mod first_run_tests {
    use crate::config::{Config, ConfigManager, FileConfigManager};
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    /// Helper to create a test config manager with a temporary directory
    struct TestConfigManager {
        temp_dir: TempDir,
    }

    impl TestConfigManager {
        fn new() -> Self {
            TestConfigManager {
                temp_dir: TempDir::new().unwrap(),
            }
        }

        fn get_config_path(&self) -> PathBuf {
            self.temp_dir.path().join("config.json")
        }

        fn is_first_run(&self) -> bool {
            !self.get_config_path().exists()
        }

        fn save_config(&self, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
            let config_path = self.get_config_path();
            let json = serde_json::to_string_pretty(config)?;
            fs::write(&config_path, json)?;
            Ok(())
        }

        fn load_config(&self) -> Result<Config, Box<dyn std::error::Error>> {
            let config_path = self.get_config_path();

            if !config_path.exists() {
                return Ok(FileConfigManager::get_default());
            }

            let contents = fs::read_to_string(&config_path)?;
            let config: Config = serde_json::from_str(&contents)?;
            Ok(config)
        }
    }

    #[test]
    fn test_is_first_run_when_no_config_exists() {
        let manager = TestConfigManager::new();

        // Should be first run when config doesn't exist
        assert!(manager.is_first_run());
    }

    #[test]
    fn test_is_not_first_run_after_config_saved() {
        let manager = TestConfigManager::new();

        // Initially should be first run
        assert!(manager.is_first_run());

        // Save a config
        let config = FileConfigManager::get_default();
        manager.save_config(&config).unwrap();

        // Should no longer be first run
        assert!(!manager.is_first_run());
    }

    #[test]
    fn test_setup_flow_saves_library_path() {
        let manager = TestConfigManager::new();

        // Simulate setup flow
        let mut config = FileConfigManager::get_default();
        config.library_path = Some("/path/to/music".to_string());
        config.spotify_enabled = true;
        config.youtube_enabled = false;

        // Save config (simulating setup completion)
        manager.save_config(&config).unwrap();

        // Load config back
        let loaded_config = manager.load_config().unwrap();

        // Verify setup values were persisted
        assert_eq!(
            loaded_config.library_path,
            Some("/path/to/music".to_string())
        );
        assert_eq!(loaded_config.spotify_enabled, true);
        assert_eq!(loaded_config.youtube_enabled, false);
    }

    #[test]
    fn test_setup_flow_with_empty_library_path() {
        let manager = TestConfigManager::new();

        // Simulate setup flow where user skips library path
        let mut config = FileConfigManager::get_default();
        config.library_path = None;
        config.spotify_enabled = false;
        config.youtube_enabled = false;

        // Save config
        manager.save_config(&config).unwrap();

        // Load config back
        let loaded_config = manager.load_config().unwrap();

        // Verify None was persisted
        assert_eq!(loaded_config.library_path, None);
    }

    #[test]
    fn test_setup_flow_preserves_other_settings() {
        let manager = TestConfigManager::new();

        // Create config with custom settings
        let mut config = FileConfigManager::get_default();
        config.library_path = Some("/music".to_string());
        config.volume = 0.5;
        config.visualizer_style = "waveform".to_string();
        config.window_position.x = 200;
        config.window_position.y = 300;
        config.window_size.width = 1024;
        config.window_size.height = 768;

        // Save config
        manager.save_config(&config).unwrap();

        // Load config back
        let loaded_config = manager.load_config().unwrap();

        // Verify all settings were preserved
        assert_eq!(loaded_config.library_path, Some("/music".to_string()));
        assert_eq!(loaded_config.volume, 0.5);
        assert_eq!(loaded_config.visualizer_style, "waveform");
        assert_eq!(loaded_config.window_position.x, 200);
        assert_eq!(loaded_config.window_position.y, 300);
        assert_eq!(loaded_config.window_size.width, 1024);
        assert_eq!(loaded_config.window_size.height, 768);
    }

    #[test]
    fn test_multiple_setup_completions() {
        let manager = TestConfigManager::new();

        // First setup
        let mut config1 = FileConfigManager::get_default();
        config1.library_path = Some("/music1".to_string());
        manager.save_config(&config1).unwrap();

        // Second setup (user changes settings)
        let mut config2 = manager.load_config().unwrap();
        config2.library_path = Some("/music2".to_string());
        config2.spotify_enabled = true;
        manager.save_config(&config2).unwrap();

        // Load final config
        let final_config = manager.load_config().unwrap();

        // Verify latest settings are persisted
        assert_eq!(final_config.library_path, Some("/music2".to_string()));
        assert_eq!(final_config.spotify_enabled, true);
    }

    #[test]
    fn test_config_file_created_after_setup() {
        let manager = TestConfigManager::new();
        let config_path = manager.get_config_path();

        // Config file should not exist initially
        assert!(!config_path.exists());

        // Complete setup
        let config = FileConfigManager::get_default();
        manager.save_config(&config).unwrap();

        // Config file should now exist
        assert!(config_path.exists());

        // File should be valid JSON
        let contents = fs::read_to_string(&config_path).unwrap();
        let parsed: Config = serde_json::from_str(&contents).unwrap();
        assert_eq!(parsed, config);
    }
}
