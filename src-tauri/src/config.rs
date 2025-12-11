use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Config {
    pub library_path: Option<String>,
    pub last_skin: Option<String>,
    pub volume: f32,
    pub visualizer_style: String,
    pub spotify_enabled: bool,
    pub youtube_enabled: bool,
    pub window_position: WindowPosition,
    pub window_size: WindowSize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WindowPosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WindowSize {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug)]
pub enum ConfigError {
    IoError(io::Error),
    SerializationError(serde_json::Error),
    InvalidPath,
}

impl From<io::Error> for ConfigError {
    fn from(err: io::Error) -> Self {
        ConfigError::IoError(err)
    }
}

impl From<serde_json::Error> for ConfigError {
    fn from(err: serde_json::Error) -> Self {
        ConfigError::SerializationError(err)
    }
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::IoError(e) => write!(f, "IO error: {}", e),
            ConfigError::SerializationError(e) => write!(f, "Serialization error: {}", e),
            ConfigError::InvalidPath => write!(f, "Invalid configuration path"),
        }
    }
}

impl std::error::Error for ConfigError {}

pub trait ConfigManager {
    fn load() -> Result<Config, ConfigError>;
    fn save(&self, config: &Config) -> Result<(), ConfigError>;
    fn get_default() -> Config;
}

pub struct FileConfigManager;

impl FileConfigManager {
    /// Get the configuration file path in the AppData directory
    pub fn get_config_path() -> Result<PathBuf, ConfigError> {
        let app_data = dirs::config_dir().ok_or(ConfigError::InvalidPath)?;
        let milk_dir = app_data.join("milk");

        // Create directory if it doesn't exist
        if !milk_dir.exists() {
            fs::create_dir_all(&milk_dir)?;
        }

        Ok(milk_dir.join("config.json"))
    }
}

impl ConfigManager for FileConfigManager {
    fn load() -> Result<Config, ConfigError> {
        let config_path = Self::get_config_path()?;

        if !config_path.exists() {
            // Return default config if file doesn't exist
            return Ok(Self::get_default());
        }

        let contents = fs::read_to_string(&config_path)?;

        // Try to parse the config, return default if corrupted
        match serde_json::from_str::<Config>(&contents) {
            Ok(config) => Ok(config),
            Err(_) => {
                // Config is corrupted, return default
                Ok(Self::get_default())
            }
        }
    }

    fn save(&self, config: &Config) -> Result<(), ConfigError> {
        let config_path = Self::get_config_path()?;
        let json = serde_json::to_string_pretty(config)?;
        fs::write(&config_path, json)?;
        Ok(())
    }

    fn get_default() -> Config {
        Config {
            library_path: None,
            last_skin: None,
            volume: 0.7,
            visualizer_style: "bars".to_string(),
            spotify_enabled: false,
            youtube_enabled: false,
            window_position: WindowPosition { x: 100, y: 100 },
            window_size: WindowSize {
                width: 800,
                height: 600,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = FileConfigManager::get_default();
        assert_eq!(config.volume, 0.7);
        assert_eq!(config.visualizer_style, "bars");
        assert_eq!(config.spotify_enabled, false);
        assert_eq!(config.youtube_enabled, false);
    }

    #[test]
    fn test_config_serialization() {
        let config = FileConfigManager::get_default();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: Config = serde_json::from_str(&json).unwrap();
        assert_eq!(config, deserialized);
    }
}

#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;
    use std::fs;
    use tempfile::TempDir;

    // Custom ConfigManager for testing that uses a temporary directory
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

        fn load(&self) -> Result<Config, ConfigError> {
            let config_path = self.get_config_path();

            if !config_path.exists() {
                return Ok(FileConfigManager::get_default());
            }

            let contents = fs::read_to_string(&config_path)?;

            match serde_json::from_str::<Config>(&contents) {
                Ok(config) => Ok(config),
                Err(_) => Ok(FileConfigManager::get_default()),
            }
        }

        fn save(&self, config: &Config) -> Result<(), ConfigError> {
            let config_path = self.get_config_path();
            let json = serde_json::to_string_pretty(config)?;
            fs::write(&config_path, json)?;
            Ok(())
        }
    }

    // Property test generators
    fn arb_config() -> impl Strategy<Value = Config> {
        (
            prop::option::of("[a-zA-Z0-9_/\\\\:. -]{1,100}"),
            prop::option::of("[a-zA-Z0-9_. -]{1,50}"),
            0.0f32..=1.0f32,
            prop::string::string_regex("(bars|waveform|spectrum)").unwrap(),
            any::<bool>(),
            any::<bool>(),
            -1000i32..=5000i32,
            -1000i32..=5000i32,
            100u32..=4000u32,
            100u32..=3000u32,
        )
            .prop_map(
                |(
                    library_path,
                    last_skin,
                    volume,
                    visualizer_style,
                    spotify_enabled,
                    youtube_enabled,
                    x,
                    y,
                    width,
                    height,
                )| {
                    Config {
                        library_path,
                        last_skin,
                        volume,
                        visualizer_style,
                        spotify_enabled,
                        youtube_enabled,
                        window_position: WindowPosition { x, y },
                        window_size: WindowSize { width, height },
                    }
                },
            )
    }

    // **Feature: milk-player, Property 22: Configuration persistence round-trip**
    // **Validates: Requirements 10.1, 10.2**
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_config_round_trip(config in arb_config()) {
            let manager = TestConfigManager::new();

            // Save the config
            manager.save(&config).unwrap();

            // Load it back
            let loaded_config = manager.load().unwrap();

            // Should be identical
            prop_assert_eq!(config, loaded_config);
        }
    }

    // **Feature: milk-player, Property 23: Configuration corruption recovery**
    // **Validates: Requirements 10.3**
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_config_corruption_recovery(corrupted_data in "[^{}\\[\\]]{1,100}") {
            let manager = TestConfigManager::new();
            let config_path = manager.get_config_path();

            // Write corrupted data to config file
            fs::write(&config_path, corrupted_data).unwrap();

            // Load should return default config without crashing
            let loaded_config = manager.load().unwrap();
            let default_config = FileConfigManager::get_default();

            // Should get default config when file is corrupted
            prop_assert_eq!(loaded_config, default_config);
        }
    }

    #[test]
    fn test_missing_config_returns_default() {
        let manager = TestConfigManager::new();
        let loaded_config = manager.load().unwrap();
        let default_config = FileConfigManager::get_default();
        assert_eq!(loaded_config, default_config);
    }

    #[test]
    fn test_empty_file_returns_default() {
        let manager = TestConfigManager::new();
        let config_path = manager.get_config_path();

        // Write empty file
        fs::write(&config_path, "").unwrap();

        let loaded_config = manager.load().unwrap();
        let default_config = FileConfigManager::get_default();
        assert_eq!(loaded_config, default_config);
    }

    #[test]
    fn test_invalid_json_returns_default() {
        let manager = TestConfigManager::new();
        let config_path = manager.get_config_path();

        // Write invalid JSON
        fs::write(&config_path, "{invalid json}").unwrap();

        let loaded_config = manager.load().unwrap();
        let default_config = FileConfigManager::get_default();
        assert_eq!(loaded_config, default_config);
    }
}
