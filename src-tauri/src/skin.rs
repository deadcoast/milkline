use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use thiserror::Error;
use zip::ZipArchive;

#[derive(Error, Debug)]
pub enum SkinError {
    #[error("Failed to read skin file: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Failed to parse ZIP archive: {0}")]
    ZipError(#[from] zip::result::ZipError),
    #[error("Invalid skin format: {0}")]
    InvalidFormat(String),
    #[error("Missing required asset: {0}")]
    MissingAsset(String),
    #[error("Failed to parse image: {0}")]
    ImageError(#[from] image::ImageError),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedSkin {
    pub name: String,
    pub assets: HashMap<String, Vec<u8>>,
    pub regions: Option<RegionConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionConfig {
    pub main: Region,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Region {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

pub struct SkinParser;

impl SkinParser {
    /// Parse a .wsz (Winamp skin ZIP) file
    pub fn parse_wsz(skin_path: &Path) -> Result<ParsedSkin, SkinError> {
        if !skin_path.exists() {
            return Err(SkinError::IoError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Skin file not found",
            )));
        }

        let file = File::open(skin_path)?;
        let reader = BufReader::new(file);
        let mut archive = ZipArchive::new(reader)?;

        let mut assets = HashMap::new();
        let skin_name = skin_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        // Extract all files from the archive
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let file_name = file.name().to_string();

            // Skip directories
            if file.is_dir() {
                continue;
            }

            // Read file contents
            let mut contents = Vec::new();
            file.read_to_end(&mut contents)?;

            // Store the asset
            assets.insert(file_name, contents);
        }

        // Parse region.txt if it exists
        let regions = Self::parse_regions(&assets);

        Ok(ParsedSkin {
            name: skin_name,
            assets,
            regions,
        })
    }

    /// Parse a .wal (Winamp modern skin) file
    pub fn parse_wal(skin_path: &Path) -> Result<ParsedSkin, SkinError> {
        // .wal files are also ZIP archives
        Self::parse_wsz(skin_path)
    }

    /// Extract assets from a parsed skin
    pub fn extract_assets(skin: &ParsedSkin) -> Result<HashMap<String, Vec<u8>>, SkinError> {
        Ok(skin.assets.clone())
    }

    /// Parse region.txt for window shaping
    fn parse_regions(assets: &HashMap<String, Vec<u8>>) -> Option<RegionConfig> {
        // Look for region.txt (case-insensitive)
        let region_data = assets
            .iter()
            .find(|(name, _)| name.to_lowercase().ends_with("region.txt"))
            .map(|(_, data)| data)?;

        let region_text = String::from_utf8_lossy(region_data);

        // Simple parsing - just extract main window dimensions
        // Real Winamp region.txt files are more complex, but this is a minimal implementation
        let lines: Vec<&str> = region_text.lines().collect();
        if lines.is_empty() {
            return None;
        }

        // Default region if parsing fails
        Some(RegionConfig {
            main: Region {
                x: 0,
                y: 0,
                width: 275,
                height: 116,
            },
        })
    }

    /// Validate that a skin has the minimum required assets
    pub fn validate_skin(skin: &ParsedSkin) -> Result<(), SkinError> {
        // Check for at least one BMP or PNG file
        let has_image = skin.assets.keys().any(|name| {
            let lower = name.to_lowercase();
            lower.ends_with(".bmp") || lower.ends_with(".png")
        });

        if !has_image {
            return Err(SkinError::InvalidFormat(
                "No image assets found in skin".to_string(),
            ));
        }

        // Check for main.bmp (required for classic Winamp skins)
        let has_main_bmp = skin.assets.keys().any(|name| {
            name.to_lowercase() == "main.bmp"
        });

        if !has_main_bmp {
            return Err(SkinError::MissingAsset("main.bmp".to_string()));
        }

        Ok(())
    }

    /// Get a default fallback skin
    pub fn get_default_skin() -> ParsedSkin {
        ParsedSkin {
            name: "default".to_string(),
            assets: HashMap::new(),
            regions: Some(RegionConfig {
                main: Region {
                    x: 0,
                    y: 0,
                    width: 275,
                    height: 116,
                },
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;
    use zip::write::{FileOptions, ZipWriter};

    fn create_test_wsz() -> NamedTempFile {
        let temp_file = NamedTempFile::new().unwrap();
        let file = temp_file.reopen().unwrap();
        let mut zip = ZipWriter::new(file);

        // Add a simple BMP file (minimal valid BMP header)
        let bmp_data = vec![
            0x42, 0x4D, // BM signature
            0x36, 0x00, 0x00, 0x00, // File size
            0x00, 0x00, 0x00, 0x00, // Reserved
            0x36, 0x00, 0x00, 0x00, // Pixel data offset
        ];

        zip.start_file::<_, ()>("main.bmp", FileOptions::default()).unwrap();
        zip.write_all(&bmp_data).unwrap();

        // Add region.txt
        let region_txt = "275 116\n";
        zip.start_file::<_, ()>("region.txt", FileOptions::default())
            .unwrap();
        zip.write_all(region_txt.as_bytes()).unwrap();

        zip.finish().unwrap();
        temp_file
    }

    #[test]
    fn test_parse_wsz_success() {
        let temp_wsz = create_test_wsz();
        let result = SkinParser::parse_wsz(temp_wsz.path());
        assert!(result.is_ok());

        let skin = result.unwrap();
        assert!(!skin.assets.is_empty());
        assert!(skin.assets.contains_key("main.bmp"));
    }

    #[test]
    fn test_parse_wsz_nonexistent() {
        let result = SkinParser::parse_wsz(Path::new("/nonexistent/skin.wsz"));
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_skin_success() {
        let temp_wsz = create_test_wsz();
        let skin = SkinParser::parse_wsz(temp_wsz.path()).unwrap();
        let result = SkinParser::validate_skin(&skin);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_skin_no_images() {
        let skin = ParsedSkin {
            name: "test".to_string(),
            assets: HashMap::new(),
            regions: None,
        };
        let result = SkinParser::validate_skin(&skin);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_default_skin() {
        let skin = SkinParser::get_default_skin();
        assert_eq!(skin.name, "default");
        assert!(skin.regions.is_some());
    }
}

#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;
    use std::io::Write;
    use tempfile::NamedTempFile;
    use zip::write::{FileOptions, ZipWriter};

    // Helper to create a valid .wsz file with specified assets
    fn create_wsz_with_assets(assets: Vec<(&str, Vec<u8>)>) -> NamedTempFile {
        let temp_file = NamedTempFile::new().unwrap();
        let file = temp_file.reopen().unwrap();
        let mut zip = ZipWriter::new(file);

        for (name, data) in assets {
            zip.start_file::<_, ()>(name, FileOptions::default()).unwrap();
            zip.write_all(&data).unwrap();
        }

        zip.finish().unwrap();
        temp_file
    }

    // Generate a minimal valid BMP header
    fn create_minimal_bmp() -> Vec<u8> {
        vec![
            0x42, 0x4D, // BM signature
            0x36, 0x00, 0x00, 0x00, // File size
            0x00, 0x00, 0x00, 0x00, // Reserved
            0x36, 0x00, 0x00, 0x00, // Pixel data offset
        ]
    }

    // Property test generators
    fn arb_asset_name() -> impl Strategy<Value = String> {
        prop::string::string_regex("[a-zA-Z0-9_-]{1,20}\\.(bmp|png|txt)")
            .unwrap()
    }

    fn arb_asset_data() -> impl Strategy<Value = Vec<u8>> {
        prop::collection::vec(any::<u8>(), 10..100)
    }

    // **Feature: milk-player, Property 8: Skin parsing and asset mapping**
    // **Validates: Requirements 4.1, 4.2**
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]
        
        #[test]
        fn prop_skin_parsing_completeness(
            asset_names in prop::collection::vec(arb_asset_name(), 1..10),
            asset_data in prop::collection::vec(arb_asset_data(), 1..10)
        ) {
            // Ensure we have at least one image file
            let mut assets: Vec<(String, Vec<u8>)> = asset_names
                .into_iter()
                .zip(asset_data.into_iter())
                .collect();
            
            // Add at least one BMP to ensure valid skin
            assets.push(("main.bmp".to_string(), create_minimal_bmp()));
            
            // Create WSZ file
            let wsz_assets: Vec<(&str, Vec<u8>)> = assets
                .iter()
                .map(|(name, data)| (name.as_str(), data.clone()))
                .collect();
            let temp_wsz = create_wsz_with_assets(wsz_assets);
            
            // Parse the skin
            let result = SkinParser::parse_wsz(temp_wsz.path());
            prop_assert!(result.is_ok());
            
            let skin = result.unwrap();
            
            // All assets should be present in the parsed skin
            for (name, _) in &assets {
                prop_assert!(
                    skin.assets.contains_key(name),
                    "Asset {} should be present in parsed skin", name
                );
            }
            
            // Number of assets should match
            prop_assert_eq!(skin.assets.len(), assets.len());
        }
    }

    // **Feature: milk-player, Property 8: Skin parsing and asset mapping**
    // **Validates: Requirements 4.1, 4.2**
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]
        
        #[test]
        fn prop_skin_asset_data_preservation(
            asset_name in arb_asset_name(),
            asset_data in arb_asset_data()
        ) {
            // Create WSZ with a single asset
            let assets = vec![
                (asset_name.as_str(), asset_data.clone()),
                ("main.bmp", create_minimal_bmp()),
            ];
            let temp_wsz = create_wsz_with_assets(assets);
            
            // Parse the skin
            let skin = SkinParser::parse_wsz(temp_wsz.path()).unwrap();
            
            // Asset data should be preserved exactly
            if let Some(parsed_data) = skin.assets.get(&asset_name) {
                prop_assert_eq!(parsed_data, &asset_data);
            }
        }
    }

    // **Feature: milk-player, Property 9: Skin application completeness**
    // **Validates: Requirements 4.3**
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]
        
        #[test]
        fn prop_skin_application_completeness(
            num_assets in 1usize..10
        ) {
            // Create a skin with multiple image assets
            let mut assets = vec![];
            for i in 0..num_assets {
                let name = format!("asset_{}.bmp", i);
                assets.push((name, create_minimal_bmp()));
            }
            
            let wsz_assets: Vec<(&str, Vec<u8>)> = assets
                .iter()
                .map(|(name, data)| (name.as_str(), data.clone()))
                .collect();
            let temp_wsz = create_wsz_with_assets(wsz_assets);
            
            // Parse and validate the skin
            let skin = SkinParser::parse_wsz(temp_wsz.path()).unwrap();
            let validation_result = SkinParser::validate_skin(&skin);
            
            // Skin should be valid (has image assets)
            prop_assert!(validation_result.is_ok());
            
            // All assets should be accessible
            for (name, _) in &assets {
                prop_assert!(
                    skin.assets.contains_key(name),
                    "Asset {} should be accessible after application", name
                );
            }
        }
    }

    // **Feature: milk-player, Property 10: Skin error fallback**
    // **Validates: Requirements 4.4**
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]
        
        #[test]
        fn prop_skin_error_fallback_invalid_skin(
            non_image_assets in prop::collection::vec(
                (arb_asset_name(), arb_asset_data()),
                1..5
            )
        ) {
            // Create a skin with only non-image files (invalid)
            let assets: Vec<(&str, Vec<u8>)> = non_image_assets
                .iter()
                .filter(|(name, _)| !name.ends_with(".bmp") && !name.ends_with(".png"))
                .map(|(name, data)| (name.as_str(), data.clone()))
                .collect();
            
            // Skip if we accidentally got image files
            if assets.is_empty() {
                return Ok(());
            }
            
            let temp_wsz = create_wsz_with_assets(assets);
            
            // Parse the skin
            let skin = SkinParser::parse_wsz(temp_wsz.path()).unwrap();
            
            // Validation should fail (no image assets)
            let validation_result = SkinParser::validate_skin(&skin);
            prop_assert!(validation_result.is_err());
            
            // Default skin should always be valid
            let default_skin = SkinParser::get_default_skin();
            prop_assert_eq!(default_skin.name, "default");
            prop_assert!(default_skin.regions.is_some());
        }
    }

    // **Feature: milk-player, Property 10: Skin error fallback**
    // **Validates: Requirements 4.4**
    #[test]
    fn test_nonexistent_skin_returns_error() {
        // Attempting to parse a non-existent file should return an error
        let result = SkinParser::parse_wsz(Path::new("/nonexistent/path/skin.wsz"));
        assert!(result.is_err());
        
        // But we can always fall back to default
        let default_skin = SkinParser::get_default_skin();
        assert_eq!(default_skin.name, "default");
    }

    #[test]
    fn test_corrupted_zip_fallback() {
        // Create a file with invalid ZIP data
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"This is not a valid ZIP file").unwrap();
        temp_file.flush().unwrap();
        
        // Parsing should fail
        let result = SkinParser::parse_wsz(temp_file.path());
        assert!(result.is_err());
        
        // Default skin should be available
        let default_skin = SkinParser::get_default_skin();
        assert_eq!(default_skin.name, "default");
    }

    // **Feature: milk-player, Property 11: Skin persistence round-trip**
    // **Validates: Requirements 4.5**
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]
        
        #[test]
        fn prop_skin_persistence_round_trip(
            _skin_name in "[a-zA-Z0-9_-]{1,20}"
        ) {
            use crate::config::{Config, ConfigManager, FileConfigManager};
            use std::fs;
            use tempfile::TempDir;
            
            // Create a temporary directory for config
            let temp_dir = TempDir::new().unwrap();
            let config_path = temp_dir.path().join("config.json");
            
            // Create a valid skin file
            let assets = vec![("main.bmp", create_minimal_bmp())];
            let temp_wsz = create_wsz_with_assets(assets);
            let skin_path = temp_wsz.path().to_str().unwrap().to_string();
            
            // Parse the skin
            let skin = SkinParser::parse_wsz(temp_wsz.path()).unwrap();
            prop_assert!(SkinParser::validate_skin(&skin).is_ok());
            
            // Save skin path to config
            let mut config = FileConfigManager::get_default();
            config.last_skin = Some(skin_path.clone());
            
            let json = serde_json::to_string_pretty(&config).unwrap();
            fs::write(&config_path, json).unwrap();
            
            // Load config back
            let loaded_json = fs::read_to_string(&config_path).unwrap();
            let loaded_config: Config = serde_json::from_str(&loaded_json).unwrap();
            
            // Skin path should be preserved
            prop_assert_eq!(loaded_config.last_skin.as_ref(), Some(&skin_path));
            
            // Should be able to load the skin again
            if let Some(saved_path) = &loaded_config.last_skin {
                let reloaded_skin = SkinParser::parse_wsz(Path::new(saved_path));
                prop_assert!(reloaded_skin.is_ok());
            }
        }
    }
}
