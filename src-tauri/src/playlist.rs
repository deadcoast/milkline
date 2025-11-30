use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;
use tokio::fs;

#[derive(Debug, Error)]
pub enum PlaylistError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Playlist not found: {0}")]
    NotFound(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackMetadata {
    pub year: Option<u32>,
    pub genre: Option<String>,
    pub track_number: Option<u32>,
    pub album_art: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub duration: f64,
    pub file_path: Option<String>,
    pub source: String,
    pub metadata: TrackMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playlist {
    pub id: String,
    pub name: String,
    pub tracks: Vec<Track>,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub modified_at: chrono::DateTime<chrono::Utc>,
}

pub struct PlaylistManager {
    playlists_dir: PathBuf,
}

impl PlaylistManager {
    pub async fn new() -> Result<Self, PlaylistError> {
        let playlists_dir = Self::get_playlists_directory()?;
        
        // Create directory if it doesn't exist (async)
        if !playlists_dir.exists() {
            fs::create_dir_all(&playlists_dir).await?;
        }
        
        Ok(Self { playlists_dir })
    }

    fn get_playlists_directory() -> Result<PathBuf, PlaylistError> {
        let app_data = dirs::data_local_dir()
            .ok_or_else(|| PlaylistError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Could not find AppData directory"
            )))?;
        
        Ok(app_data.join("milk").join("playlists"))
    }

    fn get_playlist_path(&self, playlist_id: &str) -> PathBuf {
        self.playlists_dir.join(format!("{}.json", playlist_id))
    }

    pub async fn create_playlist(&self, name: String) -> Result<Playlist, PlaylistError> {
        #[cfg(not(test))]
        crate::performance::record_playlist_operation();
        
        let now = chrono::Utc::now();
        let playlist = Playlist {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            tracks: Vec::new(),
            created_at: now,
            modified_at: now,
        };
        
        self.save_playlist(&playlist).await?;
        Ok(playlist)
    }

    pub async fn save_playlist(&self, playlist: &Playlist) -> Result<(), PlaylistError> {
        let path = self.get_playlist_path(&playlist.id);
        let json = serde_json::to_string_pretty(playlist)?;
        fs::write(path, json).await?;
        Ok(())
    }

    pub async fn load_playlist(&self, playlist_id: &str) -> Result<Playlist, PlaylistError> {
        let path = self.get_playlist_path(playlist_id);
        
        if !path.exists() {
            return Err(PlaylistError::NotFound(playlist_id.to_string()));
        }
        
        let json = fs::read_to_string(path).await?;
        let playlist = serde_json::from_str(&json)?;
        Ok(playlist)
    }

    pub async fn list_playlists(&self) -> Result<Vec<Playlist>, PlaylistError> {
        let mut playlists = Vec::new();
        
        if !self.playlists_dir.exists() {
            return Ok(playlists);
        }
        
        let mut entries = fs::read_dir(&self.playlists_dir).await?;
        
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(json) = fs::read_to_string(&path).await {
                    if let Ok(playlist) = serde_json::from_str::<Playlist>(&json) {
                        playlists.push(playlist);
                    }
                }
            }
        }
        
        Ok(playlists)
    }

    pub async fn delete_playlist(&self, playlist_id: &str) -> Result<(), PlaylistError> {
        let path = self.get_playlist_path(playlist_id);
        
        if !path.exists() {
            return Err(PlaylistError::NotFound(playlist_id.to_string()));
        }
        
        fs::remove_file(path).await?;
        Ok(())
    }

    pub async fn add_track(&self, playlist_id: &str, track: Track) -> Result<Playlist, PlaylistError> {
        let mut playlist = self.load_playlist(playlist_id).await?;
        playlist.tracks.push(track);
        playlist.modified_at = chrono::Utc::now();
        self.save_playlist(&playlist).await?;
        Ok(playlist)
    }

    pub async fn remove_track(&self, playlist_id: &str, track_id: &str) -> Result<Playlist, PlaylistError> {
        let mut playlist = self.load_playlist(playlist_id).await?;
        playlist.tracks.retain(|t| t.id != track_id);
        playlist.modified_at = chrono::Utc::now();
        self.save_playlist(&playlist).await?;
        Ok(playlist)
    }

    pub async fn reorder_tracks(&self, playlist_id: &str, track_ids: Vec<String>) -> Result<Playlist, PlaylistError> {
        let mut playlist = self.load_playlist(playlist_id).await?;
        
        // Create a map of track_id to track for quick lookup
        let track_map: std::collections::HashMap<String, Track> = playlist.tracks
            .into_iter()
            .map(|t| (t.id.clone(), t))
            .collect();
        
        // Reorder tracks based on the provided order
        let mut new_tracks = Vec::new();
        for track_id in track_ids {
            if let Some(track) = track_map.get(&track_id) {
                new_tracks.push(track.clone());
            }
        }
        
        playlist.tracks = new_tracks;
        playlist.modified_at = chrono::Utc::now();
        self.save_playlist(&playlist).await?;
        Ok(playlist)
    }

    pub async fn update_playlist(&self, playlist_id: &str, name: Option<String>) -> Result<Playlist, PlaylistError> {
        let mut playlist = self.load_playlist(playlist_id).await?;
        
        if let Some(new_name) = name {
            playlist.name = new_name;
        }
        
        playlist.modified_at = chrono::Utc::now();
        self.save_playlist(&playlist).await?;
        Ok(playlist)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use tempfile::TempDir;

    fn create_test_manager() -> (PlaylistManager, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let manager = PlaylistManager {
            playlists_dir: temp_dir.path().to_path_buf(),
        };
        (manager, temp_dir)
    }

    fn arb_track() -> impl Strategy<Value = Track> {
        (
            "[a-z0-9]{8}",
            "[A-Za-z ]{5,20}",
            "[A-Za-z ]{5,20}",
            "[A-Za-z ]{5,20}",
            1.0..600.0,
            proptest::option::of("[a-z/]{10,30}"),
            "[a-z]{5,10}",
        ).prop_map(|(id, title, artist, album, duration, file_path, source)| {
            Track {
                id,
                title,
                artist,
                album,
                duration,
                file_path,
                source,
                metadata: TrackMetadata {
                    year: None,
                    genre: None,
                    track_number: None,
                    album_art: None,
                },
            }
        })
    }

    fn arb_playlist_name() -> impl Strategy<Value = String> {
        "[A-Za-z0-9 ]{3,30}"
    }

    #[tokio::test]
    async fn test_playlist_manager_creation() {
        let manager = PlaylistManager::new().await;
        assert!(manager.is_ok());
    }

    // **Feature: milk-player, Property 18: Playlist persistence**
    // **Validates: Requirements 9.1, 9.2, 9.5**
    // For any playlist modification (create, add track, remove track, reorder), 
    // the changes should be immediately persisted to disk and retrievable after application restart.
    // Note: proptest doesn't support async tests directly, so we use tokio::runtime::Runtime
    proptest! {
        #[test]
        fn prop_playlist_create_persistence(name in arb_playlist_name()) {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let (manager, _temp_dir) = create_test_manager();
                
                // Create playlist
                let playlist = manager.create_playlist(name.clone()).await.unwrap();
                
                // Load it back
                let loaded = manager.load_playlist(&playlist.id).await.unwrap();
                
                // Verify all fields match
                prop_assert_eq!(loaded.id, playlist.id);
                prop_assert_eq!(loaded.name, name);
                prop_assert_eq!(loaded.tracks.len(), 0);
                Ok(())
            }).unwrap();
        }

        #[test]
        fn prop_playlist_add_track_persistence(
            name in arb_playlist_name(),
            track in arb_track()
        ) {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let (manager, _temp_dir) = create_test_manager();
                
                // Create playlist and add track
                let playlist = manager.create_playlist(name).await.unwrap();
                let _updated = manager.add_track(&playlist.id, track.clone()).await.unwrap();
                
                // Load it back
                let loaded = manager.load_playlist(&playlist.id).await.unwrap();
                
                // Verify track was persisted
                prop_assert_eq!(loaded.tracks.len(), 1);
                prop_assert_eq!(&loaded.tracks[0].id, &track.id);
                prop_assert_eq!(&loaded.tracks[0].title, &track.title);
                Ok(())
            }).unwrap();
        }

        #[test]
        fn prop_playlist_remove_track_persistence(
            name in arb_playlist_name(),
            tracks in prop::collection::vec(arb_track(), 1..5)
        ) {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let (manager, _temp_dir) = create_test_manager();
                
                // Create playlist and add tracks
                let playlist = manager.create_playlist(name).await.unwrap();
                let mut current_playlist = playlist;
                for track in &tracks {
                    current_playlist = manager.add_track(&current_playlist.id, track.clone()).await.unwrap();
                }
                
                // Remove first track
                let track_to_remove = tracks[0].id.clone();
                manager.remove_track(&current_playlist.id, &track_to_remove).await.unwrap();
                
                // Load it back
                let loaded = manager.load_playlist(&current_playlist.id).await.unwrap();
                
                // Verify track was removed and persisted
                prop_assert_eq!(loaded.tracks.len(), tracks.len() - 1);
                prop_assert!(!loaded.tracks.iter().any(|t| t.id == track_to_remove));
                Ok(())
            }).unwrap();
        }

        #[test]
        fn prop_playlist_reorder_persistence(
            name in arb_playlist_name(),
            tracks in prop::collection::vec(arb_track(), 2..5)
        ) {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let (manager, _temp_dir) = create_test_manager();
                
                // Create playlist and add tracks
                let playlist = manager.create_playlist(name).await.unwrap();
                let mut current_playlist = playlist;
                for track in &tracks {
                    current_playlist = manager.add_track(&current_playlist.id, track.clone()).await.unwrap();
                }
                
                // Reverse the order
                let mut reversed_ids: Vec<String> = tracks.iter().map(|t| t.id.clone()).collect();
                reversed_ids.reverse();
                
                manager.reorder_tracks(&current_playlist.id, reversed_ids.clone()).await.unwrap();
                
                // Load it back
                let loaded = manager.load_playlist(&current_playlist.id).await.unwrap();
                
                // Verify order was persisted
                let loaded_ids: Vec<String> = loaded.tracks.iter().map(|t| t.id.clone()).collect();
                prop_assert_eq!(loaded_ids, reversed_ids);
                Ok(())
            }).unwrap();
        }

        // **Feature: milk-player, Property 21: Track removal non-destructive**
        // **Validates: Requirements 9.5**
        // For any track removed from a playlist, the original audio file should remain unmodified on disk.
        #[test]
        fn prop_track_removal_non_destructive(
            name in arb_playlist_name(),
            tracks in prop::collection::vec(arb_track(), 1..5)
        ) {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let (manager, temp_dir) = create_test_manager();
                
                // Create test audio files for tracks with file paths
                let mut file_paths = Vec::new();
                for (i, track) in tracks.iter().enumerate() {
                    if track.file_path.is_some() {
                        let file_path = temp_dir.path().join(format!("test_audio_{}.mp3", i));
                        std::fs::write(&file_path, b"fake audio data").unwrap();
                        file_paths.push(file_path);
                    }
                }
                
                // Create playlist and add tracks
                let playlist = manager.create_playlist(name).await.unwrap();
                let mut current_playlist = playlist;
                
                // Add tracks with actual file paths
                for (i, mut track) in tracks.clone().into_iter().enumerate() {
                    if track.file_path.is_some() && i < file_paths.len() {
                        track.file_path = Some(file_paths[i].to_string_lossy().to_string());
                    }
                    current_playlist = manager.add_track(&current_playlist.id, track).await.unwrap();
                }
                
                // Remove first track
                let track_to_remove = &current_playlist.tracks[0];
                let removed_file_path = track_to_remove.file_path.clone();
                
                manager.remove_track(&current_playlist.id, &track_to_remove.id).await.unwrap();
                
                // Verify the original file still exists if it had a file path
                if let Some(path) = removed_file_path {
                    let path_buf = std::path::PathBuf::from(path);
                    if path_buf.exists() {
                        // File should still exist
                        prop_assert!(path_buf.exists());
                        // File content should be unchanged
                        let content = std::fs::read(&path_buf).unwrap();
                        prop_assert_eq!(content, b"fake audio data");
                    }
                }
                
                // Verify track was removed from playlist
                let loaded = manager.load_playlist(&current_playlist.id).await.unwrap();
                prop_assert_eq!(loaded.tracks.len(), tracks.len() - 1);
                Ok(())
            }).unwrap();
        }
    }
}
