use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::io;

/// Track data model representing an audio file in the library
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Track {
    pub id: String,
    pub file_path: String,
    pub file_name: String,
    pub extension: String,
}

#[derive(Debug)]
pub enum ScanError {
    IoError(io::Error),
    InvalidPath,
}

impl From<io::Error> for ScanError {
    fn from(err: io::Error) -> Self {
        ScanError::IoError(err)
    }
}

impl std::fmt::Display for ScanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScanError::IoError(e) => write!(f, "IO error: {}", e),
            ScanError::InvalidPath => write!(f, "Invalid path"),
        }
    }
}

impl std::error::Error for ScanError {}

/// LibraryScanner handles scanning directories for audio files
pub struct LibraryScanner;

impl LibraryScanner {
    /// Supported audio file extensions
    const SUPPORTED_EXTENSIONS: &'static [&'static str] = &["mp3", "flac", "wav"];

    /// Scan a directory recursively for audio files
    pub fn scan_directory(path: &Path) -> Result<Vec<Track>, ScanError> {
        if !path.exists() {
            return Err(ScanError::InvalidPath);
        }

        if !path.is_dir() {
            return Err(ScanError::InvalidPath);
        }

        let mut tracks = Vec::new();
        Self::scan_recursive(path, &mut tracks)?;
        Ok(tracks)
    }

    /// Recursive helper function for directory traversal
    fn scan_recursive(path: &Path, tracks: &mut Vec<Track>) -> Result<(), ScanError> {
        let entries = fs::read_dir(path)?;

        for entry in entries {
            let entry = entry?;
            let entry_path = entry.path();

            if entry_path.is_dir() {
                // Recursively scan subdirectories
                Self::scan_recursive(&entry_path, tracks)?;
            } else if entry_path.is_file() {
                // Check if file has supported extension
                if let Some(extension) = entry_path.extension() {
                    let ext_str = extension.to_string_lossy().to_lowercase();
                    if Self::is_supported_extension(&ext_str) {
                        // Create track from file
                        if let Some(track) = Self::create_track(&entry_path) {
                            tracks.push(track);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Create a Track from a file path
    fn create_track(path: &Path) -> Option<Track> {
        let file_path = path.to_string_lossy().to_string();
        let file_name = path.file_name()?.to_string_lossy().to_string();
        let extension = path.extension()?.to_string_lossy().to_lowercase();

        // Generate a simple ID from the file path
        let id = Self::generate_id(&file_path);

        Some(Track {
            id,
            file_path,
            file_name,
            extension,
        })
    }

    /// Generate a unique ID for a track based on its file path
    fn generate_id(file_path: &str) -> String {
        // Simple hash-like ID generation using the file path
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        file_path.hash(&mut hasher);
        format!("track_{:x}", hasher.finish())
    }

    /// Check if a file extension is supported
    pub fn is_supported_extension(extension: &str) -> bool {
        let ext_lower = extension.to_lowercase();
        Self::SUPPORTED_EXTENSIONS.contains(&ext_lower.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_scan_empty_directory() {
        let temp_dir = TempDir::new().unwrap();
        let tracks = LibraryScanner::scan_directory(temp_dir.path()).unwrap();
        assert_eq!(tracks.len(), 0);
    }

    #[test]
    fn test_scan_directory_with_audio_files() {
        let temp_dir = TempDir::new().unwrap();
        
        // Create test audio files
        fs::write(temp_dir.path().join("song1.mp3"), b"fake mp3 data").unwrap();
        fs::write(temp_dir.path().join("song2.flac"), b"fake flac data").unwrap();
        fs::write(temp_dir.path().join("song3.wav"), b"fake wav data").unwrap();
        
        let tracks = LibraryScanner::scan_directory(temp_dir.path()).unwrap();
        assert_eq!(tracks.len(), 3);
    }

    #[test]
    fn test_scan_directory_filters_non_audio() {
        let temp_dir = TempDir::new().unwrap();
        
        // Create mixed files
        fs::write(temp_dir.path().join("song.mp3"), b"fake mp3 data").unwrap();
        fs::write(temp_dir.path().join("image.jpg"), b"fake jpg data").unwrap();
        fs::write(temp_dir.path().join("document.txt"), b"fake txt data").unwrap();
        
        let tracks = LibraryScanner::scan_directory(temp_dir.path()).unwrap();
        assert_eq!(tracks.len(), 1);
        assert_eq!(tracks[0].extension, "mp3");
    }

    #[test]
    fn test_scan_directory_recursive() {
        let temp_dir = TempDir::new().unwrap();
        
        // Create nested directory structure
        let subdir = temp_dir.path().join("subdir");
        fs::create_dir(&subdir).unwrap();
        
        fs::write(temp_dir.path().join("root.mp3"), b"fake mp3 data").unwrap();
        fs::write(subdir.join("nested.flac"), b"fake flac data").unwrap();
        
        let tracks = LibraryScanner::scan_directory(temp_dir.path()).unwrap();
        assert_eq!(tracks.len(), 2);
    }

    #[test]
    fn test_scan_invalid_path() {
        let result = LibraryScanner::scan_directory(Path::new("/nonexistent/path"));
        assert!(result.is_err());
    }

    #[test]
    fn test_is_supported_extension() {
        assert!(LibraryScanner::is_supported_extension("mp3"));
        assert!(LibraryScanner::is_supported_extension("MP3"));
        assert!(LibraryScanner::is_supported_extension("flac"));
        assert!(LibraryScanner::is_supported_extension("wav"));
        assert!(!LibraryScanner::is_supported_extension("jpg"));
        assert!(!LibraryScanner::is_supported_extension("txt"));
    }
}

#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;
    use tempfile::TempDir;
    use std::fs;

    // Helper to count files with supported extensions in a directory
    fn count_supported_files(path: &Path) -> usize {
        let mut count = 0;
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    count += count_supported_files(&entry_path);
                } else if entry_path.is_file() {
                    if let Some(extension) = entry_path.extension() {
                        let ext_str = extension.to_string_lossy().to_lowercase();
                        if LibraryScanner::is_supported_extension(&ext_str) {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }

    // Generator for file names with various extensions
    fn arb_file_name() -> impl Strategy<Value = (String, bool)> {
        prop::string::string_regex("[a-zA-Z0-9_-]{1,20}").unwrap()
            .prop_flat_map(|name| {
                prop_oneof![
                    Just((format!("{}.mp3", name), true)),
                    Just((format!("{}.flac", name), true)),
                    Just((format!("{}.wav", name), true)),
                    Just((format!("{}.MP3", name), true)),
                    Just((format!("{}.FLAC", name), true)),
                    Just((format!("{}.WAV", name), true)),
                    Just((format!("{}.jpg", name), false)),
                    Just((format!("{}.txt", name), false)),
                    Just((format!("{}.pdf", name), false)),
                    Just((format!("{}.doc", name), false)),
                ]
            })
    }

    // Generator for a list of files
    fn arb_file_list() -> impl Strategy<Value = Vec<(String, bool)>> {
        prop::collection::vec(arb_file_name(), 0..20)
    }

    // **Feature: milk-player, Property 2: Library scanning completeness**
    // **Validates: Requirements 1.2**
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]
        
        #[test]
        fn prop_library_scanning_completeness(files in arb_file_list()) {
            let temp_dir = TempDir::new().unwrap();
            
            // Create files in the temp directory
            for (file_name, _is_supported) in &files {
                let file_path = temp_dir.path().join(file_name);
                fs::write(&file_path, b"fake audio data").unwrap();
            }
            
            // Scan the directory
            let scanned_tracks = LibraryScanner::scan_directory(temp_dir.path()).unwrap();
            
            // Count expected supported files
            let expected_count = count_supported_files(temp_dir.path());
            
            // The scanner should find exactly the number of supported files
            prop_assert_eq!(scanned_tracks.len(), expected_count);
            
            // Verify all scanned tracks have supported extensions
            for track in &scanned_tracks {
                prop_assert!(LibraryScanner::is_supported_extension(&track.extension));
            }
            
            // Verify no duplicates
            let unique_ids: std::collections::HashSet<_> = scanned_tracks.iter().map(|t| &t.id).collect();
            prop_assert_eq!(unique_ids.len(), scanned_tracks.len());
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]
        
        #[test]
        fn prop_library_scanning_recursive(
            root_files in arb_file_list(),
            subdir_files in arb_file_list(),
        ) {
            let temp_dir = TempDir::new().unwrap();
            let subdir = temp_dir.path().join("subdir");
            fs::create_dir(&subdir).unwrap();
            
            // Create files in root directory
            for (file_name, _) in &root_files {
                let file_path = temp_dir.path().join(file_name);
                fs::write(&file_path, b"fake audio data").unwrap();
            }
            
            // Create files in subdirectory
            for (file_name, _) in &subdir_files {
                let file_path = subdir.join(file_name);
                fs::write(&file_path, b"fake audio data").unwrap();
            }
            
            // Scan the directory
            let scanned_tracks = LibraryScanner::scan_directory(temp_dir.path()).unwrap();
            
            // Count expected supported files in both directories
            let expected_count = count_supported_files(temp_dir.path());
            
            // Should find all supported files recursively
            prop_assert_eq!(scanned_tracks.len(), expected_count);
        }
    }
}
