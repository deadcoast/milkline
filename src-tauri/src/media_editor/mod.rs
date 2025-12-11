// Media editor module for image and video editing operations
pub mod config;
pub mod image_ops;
pub mod types;
pub mod video_ops;

// Re-export commonly used types
pub use config::{ExportDefaults, ExportPreset, DEFAULT_CONFIG, PRESETS};
pub use types::{CropRect, ExportConfig, VideoMetadata};
