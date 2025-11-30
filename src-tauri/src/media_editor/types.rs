use serde::{Deserialize, Serialize};

/// Represents a rectangular crop area with pixel coordinates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CropRect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

/// Video metadata extracted from a video file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoMetadata {
    pub duration_sec: f64,
    pub width: u32,
    pub height: u32,
}

/// Configuration for media export operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportConfig {
    pub video_codec: String,
    pub audio_codec: String,
    pub quality: String,
}
