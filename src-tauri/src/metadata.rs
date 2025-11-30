use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Mutex;
use lru::LruCache;
use std::num::NonZeroUsize;
use id3::TagLike;

/// Track metadata extracted from audio files
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrackMetadata {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub year: Option<u32>,
    pub genre: Option<String>,
    pub track_number: Option<u32>,
    pub duration: Option<u32>,
}

impl TrackMetadata {
    /// Check if metadata has all standard fields populated
    pub fn is_complete(&self) -> bool {
        self.title.is_some()
            && self.artist.is_some()
            && self.album.is_some()
            && self.year.is_some()
            && self.genre.is_some()
            && self.track_number.is_some()
    }

    /// Check if metadata is empty (all fields are None)
    pub fn is_empty(&self) -> bool {
        self.title.is_none()
            && self.artist.is_none()
            && self.album.is_none()
            && self.year.is_none()
            && self.genre.is_none()
            && self.track_number.is_none()
            && self.duration.is_none()
    }
}

#[derive(Debug)]
pub enum MetadataError {
    IoError(std::io::Error),
    Id3Error(String),
    FlacError(String),
    UnsupportedFormat,
}

impl From<std::io::Error> for MetadataError {
    fn from(err: std::io::Error) -> Self {
        MetadataError::IoError(err)
    }
}

impl From<id3::Error> for MetadataError {
    fn from(err: id3::Error) -> Self {
        MetadataError::Id3Error(err.to_string())
    }
}

impl std::fmt::Display for MetadataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MetadataError::IoError(e) => write!(f, "IO error: {}", e),
            MetadataError::Id3Error(e) => write!(f, "ID3 error: {}", e),
            MetadataError::FlacError(e) => write!(f, "FLAC error: {}", e),
            MetadataError::UnsupportedFormat => write!(f, "Unsupported format"),
        }
    }
}

impl std::error::Error for MetadataError {}

/// MetadataExtractor handles extracting metadata from audio files
pub struct MetadataExtractor {
    cache: Mutex<LruCache<String, TrackMetadata>>,
}

impl MetadataExtractor {
    /// Create a new MetadataExtractor with LRU cache (max 1000 entries)
    pub fn new() -> Self {
        Self {
            cache: Mutex::new(LruCache::new(NonZeroUsize::new(1000).unwrap())),
        }
    }

    /// Extract metadata from an audio file
    pub fn extract(&self, file_path: &Path) -> Result<TrackMetadata, MetadataError> {
        let path_str = file_path.to_string_lossy().to_string();

        // Check cache first
        {
            let mut cache = self.cache.lock().unwrap();
            if let Some(cached) = cache.get(&path_str) {
                // Cache hit - record for performance tracking
                #[cfg(not(test))]
                crate::performance::record_cache_hit();
                return Ok(cached.clone());
            }
        }
        
        // Cache miss - record for performance tracking
        #[cfg(not(test))]
        crate::performance::record_cache_miss();

        // Extract metadata based on file extension
        let extension = file_path
            .extension()
            .and_then(|e| e.to_str())
            .map(|s| s.to_lowercase())
            .ok_or(MetadataError::UnsupportedFormat)?;

        let mut metadata = match extension.as_str() {
            "mp3" => self.extract_id3(file_path)?,
            "flac" => self.extract_flac(file_path)?,
            "wav" => TrackMetadata {
                title: None,
                artist: None,
                album: None,
                year: None,
                genre: None,
                track_number: None,
                duration: None,
            },
            _ => return Err(MetadataError::UnsupportedFormat),
        };

        // Apply fallback parsing if metadata is incomplete
        if metadata.is_empty() || metadata.title.is_none() {
            let fallback = self.parse_fallback(file_path);
            if metadata.title.is_none() {
                metadata.title = fallback.title;
            }
            if metadata.artist.is_none() {
                metadata.artist = fallback.artist;
            }
            if metadata.album.is_none() {
                metadata.album = fallback.album;
            }
        }

        // Cache the result
        {
            let mut cache = self.cache.lock().unwrap();
            cache.put(path_str, metadata.clone());
        }

        Ok(metadata)
    }

    /// Extract ID3v2 tags from mp3 files
    fn extract_id3(&self, file_path: &Path) -> Result<TrackMetadata, MetadataError> {
        // Try to read ID3 tags, but return empty metadata if no tags exist
        match id3::Tag::read_from_path(file_path) {
            Ok(tag) => Ok(TrackMetadata {
                title: tag.title().map(|s| s.to_string()),
                artist: tag.artist().map(|s| s.to_string()),
                album: tag.album().map(|s| s.to_string()),
                year: tag.year().map(|y| y as u32),
                genre: tag.genre().map(|s| s.to_string()),
                track_number: tag.track().map(|t| t as u32),
                duration: tag.duration().map(|d| d as u32),
            }),
            Err(id3::Error {
                kind: id3::ErrorKind::NoTag,
                ..
            }) => {
                // No tag found, return empty metadata (fallback will be applied later)
                Ok(TrackMetadata {
                    title: None,
                    artist: None,
                    album: None,
                    year: None,
                    genre: None,
                    track_number: None,
                    duration: None,
                })
            }
            Err(e) => Err(MetadataError::from(e)),
        }
    }

    /// Extract FLAC/Vorbis comments from flac files
    fn extract_flac(&self, file_path: &Path) -> Result<TrackMetadata, MetadataError> {
        let tag = metaflac::Tag::read_from_path(file_path)
            .map_err(|e| MetadataError::FlacError(e.to_string()))?;

        let vorbis = tag.vorbis_comments();

        Ok(TrackMetadata {
            title: vorbis
                .and_then(|v| v.title())
                .and_then(|t| t.first())
                .map(|s| s.to_string()),
            artist: vorbis
                .and_then(|v| v.artist())
                .and_then(|a| a.first())
                .map(|s| s.to_string()),
            album: vorbis
                .and_then(|v| v.album())
                .and_then(|a| a.first())
                .map(|s| s.to_string()),
            year: vorbis
                .and_then(|v| v.get("DATE"))
                .and_then(|d| d.first())
                .and_then(|s| s.parse::<u32>().ok()),
            genre: vorbis
                .and_then(|v| v.genre())
                .and_then(|g| g.first())
                .map(|s| s.to_string()),
            track_number: vorbis
                .and_then(|v| v.track()),
            duration: None, // FLAC duration requires more complex parsing
        })
    }

    /// Parse metadata from filename and directory structure as fallback
    fn parse_fallback(&self, file_path: &Path) -> TrackMetadata {
        let file_name = file_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("");

        let parent_dir = file_path
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|s| s.to_str());

        // Try to parse "Artist - Title" format
        let (artist, title) = if file_name.contains(" - ") {
            let parts: Vec<&str> = file_name.splitn(2, " - ").collect();
            if parts.len() == 2 {
                (Some(parts[0].trim().to_string()), Some(parts[1].trim().to_string()))
            } else {
                (None, Some(file_name.to_string()))
            }
        } else {
            (None, Some(file_name.to_string()))
        };

        // Use parent directory as album if available
        let album = parent_dir.map(|s| s.to_string());

        TrackMetadata {
            title,
            artist,
            album,
            year: None,
            genre: None,
            track_number: None,
            duration: None,
        }
    }

    /// Extract album artwork from an audio file
    pub fn extract_artwork(&self, file_path: &Path) -> Result<Option<Vec<u8>>, MetadataError> {
        let extension = file_path
            .extension()
            .and_then(|e| e.to_str())
            .map(|s| s.to_lowercase())
            .ok_or(MetadataError::UnsupportedFormat)?;

        match extension.as_str() {
            "mp3" => self.extract_artwork_id3(file_path),
            "flac" => self.extract_artwork_flac(file_path),
            "wav" => Ok(None), // WAV files typically don't have embedded artwork
            _ => Err(MetadataError::UnsupportedFormat),
        }
    }

    /// Extract artwork from ID3 tags
    fn extract_artwork_id3(&self, file_path: &Path) -> Result<Option<Vec<u8>>, MetadataError> {
        let tag = id3::Tag::read_from_path(file_path)?;

        // Look for picture frames
        for picture in tag.pictures() {
            // Return the first picture found (usually the cover art)
            return Ok(Some(picture.data.clone()));
        }

        Ok(None)
    }

    /// Extract artwork from FLAC tags
    fn extract_artwork_flac(&self, file_path: &Path) -> Result<Option<Vec<u8>>, MetadataError> {
        let tag = metaflac::Tag::read_from_path(file_path)
            .map_err(|e| MetadataError::FlacError(e.to_string()))?;

        // Look for picture blocks
        for picture in tag.pictures() {
            // Return the first picture found (usually the cover art)
            return Ok(Some(picture.data.clone()));
        }

        Ok(None)
    }

    /// Check if a file path is in the cache
    pub fn is_cached(&self, file_path: &Path) -> bool {
        let path_str = file_path.to_string_lossy().to_string();
        let cache = self.cache.lock().unwrap();
        cache.contains(&path_str)
    }

    /// Clear the metadata cache
    pub fn clear_cache(&self) {
        let mut cache = self.cache.lock().unwrap();
        cache.clear();
    }
}

impl Default for MetadataExtractor {
    fn default() -> Self {
        Self::new()
    }
}


#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;
    use tempfile::TempDir;
    use std::fs;

    // Helper to create a test MP3 file with ID3 tags
    fn create_test_mp3_with_tags(
        path: &Path,
        title: &str,
        artist: &str,
        album: &str,
        year: i32,
        genre: &str,
        track: u32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Create a minimal valid MP3 file (just the header)
        let mp3_data = vec![
            0xFF, 0xFB, 0x90, 0x00, // MP3 frame header
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        fs::write(path, &mp3_data)?;

        // Add ID3 tags
        let mut tag = id3::Tag::new();
        tag.set_title(title);
        tag.set_artist(artist);
        tag.set_album(album);
        tag.set_year(year);
        tag.set_genre(genre);
        tag.set_track(track);

        tag.write_to_path(path, id3::Version::Id3v24)?;
        Ok(())
    }

    // Generator for valid metadata strings
    fn arb_metadata_string() -> impl Strategy<Value = String> {
        prop::string::string_regex("[a-zA-Z0-9 ]{1,30}").unwrap()
    }

    // Generator for year
    fn arb_year() -> impl Strategy<Value = i32> {
        1900..2100i32
    }

    // Generator for track number
    fn arb_track_number() -> impl Strategy<Value = u32> {
        1..100u32
    }

    // **Feature: milk-player, Property 25: Metadata extraction completeness**
    // **Validates: Requirements 12.1**
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_metadata_extraction_completeness(
            title in arb_metadata_string(),
            artist in arb_metadata_string(),
            album in arb_metadata_string(),
            year in arb_year(),
            genre in arb_metadata_string(),
            track in arb_track_number(),
        ) {
            let temp_dir = TempDir::new().unwrap();
            let file_path = temp_dir.path().join("test.mp3");

            // Create MP3 file with all metadata fields
            create_test_mp3_with_tags(
                &file_path,
                &title,
                &artist,
                &album,
                year,
                &genre,
                track,
            ).unwrap();

            let extractor = MetadataExtractor::new();
            let metadata = extractor.extract(&file_path).unwrap();

            // All standard fields should be extracted
            prop_assert_eq!(metadata.title.as_deref(), Some(title.as_str()));
            prop_assert_eq!(metadata.artist.as_deref(), Some(artist.as_str()));
            prop_assert_eq!(metadata.album.as_deref(), Some(album.as_str()));
            prop_assert_eq!(metadata.year, Some(year as u32));
            prop_assert_eq!(metadata.genre.as_deref(), Some(genre.as_str()));
            prop_assert_eq!(metadata.track_number, Some(track));
        }
    }

    // **Feature: milk-player, Property 26: Metadata fallback parsing**
    // **Validates: Requirements 12.2**
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_metadata_fallback_parsing(
            artist in arb_metadata_string(),
            title in arb_metadata_string(),
        ) {
            let temp_dir = TempDir::new().unwrap();
            
            // Use a fixed album directory to avoid filesystem issues
            let album_name = "TestAlbum";
            let album_dir = temp_dir.path().join(album_name);
            fs::create_dir(&album_dir).unwrap();
            
            // Create file with "Artist - Title" format
            let file_name = format!("{} - {}.mp3", artist, title);
            let file_path = album_dir.join(&file_name);
            
            // Create a minimal MP3 file without tags
            let mp3_data = vec![
                0xFF, 0xFB, 0x90, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            ];
            fs::write(&file_path, &mp3_data).unwrap();

            let extractor = MetadataExtractor::new();
            let metadata = extractor.extract(&file_path).unwrap();

            // Fallback parsing should derive information from filename and directory
            // The parser trims whitespace, so we compare with trimmed values
            prop_assert_eq!(metadata.title.as_deref(), Some(title.trim()));
            prop_assert_eq!(metadata.artist.as_deref(), Some(artist.trim()));
            // Album comes from the directory name
            prop_assert_eq!(metadata.album.as_deref(), Some(album_name));
        }
    }

    // Generator for image data (simple PNG-like data)
    fn arb_image_data() -> impl Strategy<Value = Vec<u8>> {
        prop::collection::vec(any::<u8>(), 100..1000)
    }

    // **Feature: milk-player, Property 27: Album art extraction**
    // **Validates: Requirements 12.3**
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_album_art_extraction(
            title in arb_metadata_string(),
            artwork_data in arb_image_data(),
        ) {
            let temp_dir = TempDir::new().unwrap();
            let file_path = temp_dir.path().join("test.mp3");

            // Create a minimal MP3 file
            let mp3_data = vec![
                0xFF, 0xFB, 0x90, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            ];
            fs::write(&file_path, &mp3_data).unwrap();

            // Add ID3 tag with embedded artwork
            let mut tag = id3::Tag::new();
            tag.set_title(&title);
            tag.add_frame(id3::frame::Picture {
                mime_type: "image/jpeg".to_string(),
                picture_type: id3::frame::PictureType::CoverFront,
                description: "Cover".to_string(),
                data: artwork_data.clone(),
            });
            tag.write_to_path(&file_path, id3::Version::Id3v24).unwrap();

            let extractor = MetadataExtractor::new();
            let extracted_artwork = extractor.extract_artwork(&file_path).unwrap();

            // Artwork should be successfully extracted
            prop_assert!(extracted_artwork.is_some());
            let extracted = extracted_artwork.unwrap();
            
            // The extracted artwork should match the embedded artwork
            prop_assert_eq!(extracted, artwork_data);
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_album_art_extraction_no_artwork(
            title in arb_metadata_string(),
        ) {
            let temp_dir = TempDir::new().unwrap();
            let file_path = temp_dir.path().join("test.mp3");

            // Create a minimal MP3 file
            let mp3_data = vec![
                0xFF, 0xFB, 0x90, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            ];
            fs::write(&file_path, &mp3_data).unwrap();

            // Add ID3 tag WITHOUT artwork
            let mut tag = id3::Tag::new();
            tag.set_title(&title);
            tag.write_to_path(&file_path, id3::Version::Id3v24).unwrap();

            let extractor = MetadataExtractor::new();
            let extracted_artwork = extractor.extract_artwork(&file_path).unwrap();

            // No artwork should be extracted
            prop_assert!(extracted_artwork.is_none());
        }
    }

    // **Feature: milk-player, Property 28: Metadata caching efficiency**
    // **Validates: Requirements 12.4**
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_metadata_caching_efficiency(
            title in arb_metadata_string(),
            artist in arb_metadata_string(),
            album in arb_metadata_string(),
        ) {
            let temp_dir = TempDir::new().unwrap();
            let file_path = temp_dir.path().join("test.mp3");

            // Create MP3 file with metadata
            create_test_mp3_with_tags(
                &file_path,
                &title,
                &artist,
                &album,
                2020,
                "Rock",
                1,
            ).unwrap();

            let extractor = MetadataExtractor::new();

            // First extraction - should not be cached
            prop_assert!(!extractor.is_cached(&file_path));
            let metadata1 = extractor.extract(&file_path).unwrap();

            // Second extraction - should be cached
            prop_assert!(extractor.is_cached(&file_path));
            let metadata2 = extractor.extract(&file_path).unwrap();

            // Both extractions should return the same metadata
            prop_assert_eq!(metadata1.clone(), metadata2);

            // Verify the metadata is correct
            prop_assert_eq!(metadata1.title.as_deref(), Some(title.as_str()));
            prop_assert_eq!(metadata1.artist.as_deref(), Some(artist.as_str()));
            prop_assert_eq!(metadata1.album.as_deref(), Some(album.as_str()));
        }
    }
}
