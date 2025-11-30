/// Default export settings for media operations
pub struct ExportDefaults {
    pub video_codec: &'static str,
    pub audio_codec: &'static str,
    pub image_format: &'static str,
    pub video_quality: &'static str,
}

/// Default configuration constants
pub const DEFAULT_CONFIG: ExportDefaults = ExportDefaults {
    video_codec: "libx264",
    audio_codec: "aac",
    image_format: "png",
    video_quality: "23", // CRF value
};

/// Named export preset with specific encoding parameters
pub struct ExportPreset {
    pub name: &'static str,
    pub video_codec: &'static str,
    pub crf: u8,
    pub preset: &'static str,
    pub audio_codec: &'static str,
}

/// Available export presets
pub const PRESETS: &[ExportPreset] = &[
    ExportPreset {
        name: "high_quality_video",
        video_codec: "libx264",
        crf: 18,
        preset: "slow",
        audio_codec: "aac",
    },
    ExportPreset {
        name: "mobile_video",
        video_codec: "libx265",
        crf: 28,
        preset: "fast",
        audio_codec: "aac",
    },
];
