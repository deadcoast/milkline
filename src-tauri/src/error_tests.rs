#[cfg(test)]
mod tests {
    use crate::error::MilkError;

    #[test]
    fn test_error_user_messages() {
        let err = MilkError::InvalidPath("/some/path".to_string());
        assert!(err.user_message().contains("can't find that path"));

        let err = MilkError::DiskFull("config".to_string());
        assert!(err.user_message().contains("disk is full"));

        let err = MilkError::AuthenticationFailed("Spotify".to_string());
        assert!(err.user_message().contains("Authentication failed"));
    }

    #[test]
    fn test_error_categories() {
        let err = MilkError::InvalidPath("/some/path".to_string());
        assert_eq!(err.category(), "FileSystem");

        let err = MilkError::NetworkTimeout("Spotify".to_string());
        assert_eq!(err.category(), "Network");

        let err = MilkError::UnsupportedFormat("ogg".to_string());
        assert_eq!(err.category(), "Playback");
    }

    #[test]
    fn test_error_criticality() {
        let critical = MilkError::DiskFull("config".to_string());
        assert!(critical.is_critical());

        let non_critical = MilkError::MetadataError("parse failed".to_string());
        assert!(!non_critical.is_critical());
    }

    #[test]
    fn test_error_recoverability() {
        let recoverable = MilkError::NetworkTimeout("API".to_string());
        assert!(recoverable.is_recoverable());

        let non_recoverable = MilkError::DiskFull("config".to_string());
        assert!(!non_recoverable.is_recoverable());
    }
}
