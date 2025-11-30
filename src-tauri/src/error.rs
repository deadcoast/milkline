// Comprehensive error handling for milk application
use thiserror::Error;

/// Main error type for the milk application
/// Categorizes all possible errors that can occur
#[derive(Error, Debug)]
pub enum MilkError {
    // File System Errors
    #[error("File system error: {0}")]
    FileSystem(#[from] std::io::Error),
    
    #[error("Invalid path: {0}")]
    InvalidPath(String),
    
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    #[error("Disk full: unable to save {0}")]
    DiskFull(String),
    
    #[error("Corrupted file: {0}")]
    CorruptedFile(String),
    
    // Network/API Errors
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),
    
    #[error("API rate limit exceeded")]
    RateLimitExceeded,
    
    #[error("Network timeout: {0}")]
    NetworkTimeout(String),
    
    #[error("Invalid API response: {0}")]
    InvalidResponse(String),
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    // Playback Errors
    #[error("Unsupported audio format: {0}")]
    UnsupportedFormat(String),
    
    #[error("Audio decode error: {0}")]
    DecodeError(String),
    
    #[error("Audio device unavailable")]
    AudioDeviceUnavailable,
    
    // Configuration Errors
    #[error("Invalid configuration value: {0}")]
    InvalidConfig(String),
    
    #[error("Configuration parse error: {0}")]
    ConfigParseError(String),
    
    #[error("Missing required configuration: {0}")]
    MissingConfig(String),
    
    // Skin Errors
    #[error("Skin parse error: {0}")]
    SkinParseError(String),
    
    #[error("Invalid skin format: {0}")]
    InvalidSkinFormat(String),
    
    #[error("Missing skin assets: {0}")]
    MissingSkinAssets(String),
    
    // Metadata Errors
    #[error("Metadata extraction failed: {0}")]
    MetadataError(String),
    
    // Playlist Errors
    #[error("Playlist not found: {0}")]
    PlaylistNotFound(String),
    
    #[error("Invalid playlist operation: {0}")]
    InvalidPlaylistOperation(String),
    
    // Storage Errors
    #[error("Secure storage error: {0}")]
    SecureStorageError(String),
    
    // Generic Errors
    #[error("Internal error: {0}")]
    Internal(String),
    
    #[error("{0}")]
    Other(String),
}

impl MilkError {
    /// Check if this error is critical (requires user attention)
    pub fn is_critical(&self) -> bool {
        matches!(
            self,
            MilkError::DiskFull(_)
                | MilkError::PermissionDenied(_)
                | MilkError::AudioDeviceUnavailable
                | MilkError::AuthenticationFailed(_)
        )
    }

    /// Check if this error can be recovered from gracefully
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            MilkError::NetworkTimeout(_)
                | MilkError::RateLimitExceeded
                | MilkError::CorruptedFile(_)
                | MilkError::SkinParseError(_)
                | MilkError::MetadataError(_)
        )
    }

    /// Get a user-friendly error message suitable for display via farmer
    pub fn user_message(&self) -> String {
        match self {
            // File System Errors
            MilkError::FileSystem(e) => {
                format!("Oops! I had trouble accessing a file: {}", e)
            }
            MilkError::InvalidPath(path) => {
                format!("Hmm, I can't find that path: {}", path)
            }
            MilkError::PermissionDenied(resource) => {
                format!("I don't have permission to access: {}", resource)
            }
            MilkError::DiskFull(operation) => {
                format!("Your disk is full! I couldn't save: {}", operation)
            }
            MilkError::CorruptedFile(file) => {
                format!("This file seems corrupted: {}. I'll use defaults instead.", file)
            }

            // Network/API Errors
            MilkError::AuthenticationFailed(service) => {
                format!("Authentication failed for {}. Let's try logging in again!", service)
            }
            MilkError::RateLimitExceeded => {
                "Whoa, slow down! The API rate limit was exceeded. Let's wait a moment.".to_string()
            }
            MilkError::NetworkTimeout(service) => {
                format!("Connection to {} timed out. Check your internet connection?", service)
            }
            MilkError::InvalidResponse(details) => {
                format!("Got an unexpected response: {}. Let's try again.", details)
            }
            MilkError::NetworkError(details) => {
                format!("Network hiccup: {}. Is your internet working?", details)
            }

            // Playback Errors
            MilkError::UnsupportedFormat(format) => {
                format!("Sorry, I can't play {} files. Try MP3, FLAC, or WAV!", format)
            }
            MilkError::DecodeError(details) => {
                format!("Couldn't decode this audio file: {}. It might be corrupted.", details)
            }
            MilkError::AudioDeviceUnavailable => {
                "No audio device found! Check your speakers or headphones.".to_string()
            }

            // Configuration Errors
            MilkError::InvalidConfig(field) => {
                format!("Invalid configuration for: {}. I'll use the default.", field)
            }
            MilkError::ConfigParseError(_) => {
                "Your config file got scrambled. Don't worry, I'll create a fresh one!".to_string()
            }
            MilkError::MissingConfig(field) => {
                format!("Missing configuration: {}. Let's set that up!", field)
            }

            // Skin Errors
            MilkError::SkinParseError(_) => {
                "Couldn't load that skin. I'll use the default look instead!".to_string()
            }
            MilkError::InvalidSkinFormat(format) => {
                format!("That's not a valid skin format: {}. Try a .wsz or .wal file!", format)
            }
            MilkError::MissingSkinAssets(assets) => {
                format!("This skin is missing some parts: {}. Using defaults!", assets)
            }

            // Metadata Errors
            MilkError::MetadataError(_) => {
                "Couldn't read the song info. I'll guess from the filename!".to_string()
            }

            // Playlist Errors
            MilkError::PlaylistNotFound(id) => {
                format!("Can't find that playlist: {}. Did you delete it?", id)
            }
            MilkError::InvalidPlaylistOperation(op) => {
                format!("Oops, can't do that: {}. Try something else!", op)
            }

            // Storage Errors
            MilkError::SecureStorageError(_) => {
                "Had trouble with secure storage. Your credentials might need re-entry.".to_string()
            }

            // Generic Errors
            MilkError::Internal(details) => {
                format!("Something unexpected happened: {}. Let's try again!", details)
            }
            MilkError::Other(msg) => msg.clone(),
        }
    }

    /// Get the error category for logging purposes
    pub fn category(&self) -> &'static str {
        match self {
            MilkError::FileSystem(_)
            | MilkError::InvalidPath(_)
            | MilkError::PermissionDenied(_)
            | MilkError::DiskFull(_)
            | MilkError::CorruptedFile(_) => "FileSystem",

            MilkError::AuthenticationFailed(_)
            | MilkError::RateLimitExceeded
            | MilkError::NetworkTimeout(_)
            | MilkError::InvalidResponse(_)
            | MilkError::NetworkError(_) => "Network",

            MilkError::UnsupportedFormat(_)
            | MilkError::DecodeError(_)
            | MilkError::AudioDeviceUnavailable => "Playback",

            MilkError::InvalidConfig(_)
            | MilkError::ConfigParseError(_)
            | MilkError::MissingConfig(_) => "Configuration",

            MilkError::SkinParseError(_)
            | MilkError::InvalidSkinFormat(_)
            | MilkError::MissingSkinAssets(_) => "Skin",

            MilkError::MetadataError(_) => "Metadata",

            MilkError::PlaylistNotFound(_) | MilkError::InvalidPlaylistOperation(_) => "Playlist",

            MilkError::SecureStorageError(_) => "Storage",

            MilkError::Internal(_) | MilkError::Other(_) => "General",
        }
    }
}

// Conversion implementations for existing error types
impl From<crate::config::ConfigError> for MilkError {
    fn from(err: crate::config::ConfigError) -> Self {
        match err {
            crate::config::ConfigError::IoError(e) => MilkError::FileSystem(e),
            crate::config::ConfigError::SerializationError(_) => {
                MilkError::ConfigParseError(err.to_string())
            }
            crate::config::ConfigError::InvalidPath => {
                MilkError::InvalidPath("configuration directory".to_string())
            }
        }
    }
}

impl From<crate::metadata::MetadataError> for MilkError {
    fn from(err: crate::metadata::MetadataError) -> Self {
        match err {
            crate::metadata::MetadataError::IoError(e) => MilkError::FileSystem(e),
            crate::metadata::MetadataError::Id3Error(e) => MilkError::MetadataError(e),
            crate::metadata::MetadataError::FlacError(e) => MilkError::MetadataError(e),
            crate::metadata::MetadataError::UnsupportedFormat => {
                MilkError::UnsupportedFormat("unknown".to_string())
            }
        }
    }
}

impl From<crate::spotify::ApiError> for MilkError {
    fn from(err: crate::spotify::ApiError) -> Self {
        match err {
            crate::spotify::ApiError::NetworkError(e) => MilkError::NetworkError(e),
            crate::spotify::ApiError::AuthenticationError(e) => {
                MilkError::AuthenticationFailed(format!("Spotify: {}", e))
            }
            crate::spotify::ApiError::ParseError(e) => MilkError::InvalidResponse(e),
            crate::spotify::ApiError::StorageError(e) => MilkError::SecureStorageError(e),
            crate::spotify::ApiError::TokenExpired => {
                MilkError::AuthenticationFailed("Spotify token expired".to_string())
            }
            crate::spotify::ApiError::NoActivePlayback => {
                MilkError::Other("No active playback".to_string())
            }
        }
    }
}

impl From<crate::library::ScanError> for MilkError {
    fn from(err: crate::library::ScanError) -> Self {
        match err {
            crate::library::ScanError::IoError(e) => MilkError::FileSystem(e),
            crate::library::ScanError::InvalidPath => {
                MilkError::InvalidPath("library directory".to_string())
            }
        }
    }
}

impl From<crate::skin::SkinError> for MilkError {
    fn from(err: crate::skin::SkinError) -> Self {
        match err {
            crate::skin::SkinError::IoError(e) => MilkError::FileSystem(e),
            crate::skin::SkinError::ZipError(e) => MilkError::SkinParseError(e.to_string()),
            crate::skin::SkinError::ImageError(e) => MilkError::SkinParseError(e.to_string()),
            crate::skin::SkinError::InvalidFormat(f) => MilkError::InvalidSkinFormat(f),
            crate::skin::SkinError::MissingAsset(a) => MilkError::MissingSkinAssets(a),
        }
    }
}

impl From<crate::playlist::PlaylistError> for MilkError {
    fn from(err: crate::playlist::PlaylistError) -> Self {
        match err {
            crate::playlist::PlaylistError::Io(e) => MilkError::FileSystem(e),
            crate::playlist::PlaylistError::Serialization(_) => {
                MilkError::InvalidPlaylistOperation("serialization failed".to_string())
            }
            crate::playlist::PlaylistError::NotFound(id) => MilkError::PlaylistNotFound(id),
        }
    }
}

impl From<crate::secure_storage::StorageError> for MilkError {
    fn from(err: crate::secure_storage::StorageError) -> Self {
        MilkError::SecureStorageError(err.to_string())
    }
}

/// Result type alias for milk operations
pub type MilkResult<T> = Result<T, MilkError>;
