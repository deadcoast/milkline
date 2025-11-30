// Media editor module for image and video editing operations
pub mod types;
pub mod config;
pub mod image_ops;
pub mod video_ops;

// Re-export commonly used types
pub use types::{CropRect, VideoMetadata, ExportConfig};
pub use config::{ExportDefaults, ExportPreset, DEFAULT_CONFIG, PRESETS};
