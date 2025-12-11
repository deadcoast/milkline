// Error recovery mechanisms for milk application
use crate::config::{Config, ConfigManager, FileConfigManager};
use crate::error::{MilkError, MilkResult};
use crate::logging::{log_error, log_info, log_warn};
use crate::spotify::{Credentials, SpotifyBridge, StreamingService};
use crate::youtube::YouTubeBridge;
use std::time::Duration;
use tokio::time::sleep;

/// Maximum number of retry attempts for recoverable errors
const MAX_RETRIES: u32 = 3;

/// Base delay for exponential backoff (milliseconds)
const BASE_DELAY_MS: u64 = 1000;

/// Error recovery strategies
pub struct ErrorRecovery;

impl ErrorRecovery {
    /// Attempt to recover from a configuration error
    pub fn recover_config_error(error: &MilkError) -> MilkResult<Config> {
        log_warn(
            "Recovery",
            &format!("Attempting to recover from config error: {}", error),
        );

        match error {
            MilkError::ConfigParseError(_) | MilkError::CorruptedFile(_) => {
                // Config is corrupted, create a fresh default config
                log_info("Recovery", "Creating default configuration");
                let default_config = FileConfigManager::get_default();

                // Try to save the default config
                let manager = FileConfigManager;
                match manager.save(&default_config) {
                    Ok(()) => {
                        log_info("Recovery", "Default configuration saved successfully");
                        Ok(default_config)
                    }
                    Err(e) => {
                        log_error("Recovery", &format!("Failed to save default config: {}", e));
                        // Return default config anyway, even if we can't save it
                        Ok(default_config)
                    }
                }
            }
            MilkError::MissingConfig(_) => {
                // Config doesn't exist, create default
                log_info("Recovery", "Creating missing configuration");
                let default_config = FileConfigManager::get_default();
                let manager = FileConfigManager;
                let _ = manager.save(&default_config);
                Ok(default_config)
            }
            _ => Err(MilkError::ConfigParseError(error.to_string())),
        }
    }

    /// Attempt to refresh an expired token
    pub async fn recover_token_error(
        service: &str,
        credentials: Option<Credentials>,
    ) -> MilkResult<String> {
        log_info(
            "Recovery",
            &format!("Attempting to refresh {} token", service),
        );

        let credentials = credentials.ok_or_else(|| {
            MilkError::AuthenticationFailed(format!("{}: No credentials provided", service))
        })?;

        match service {
            "spotify" => {
                let bridge = SpotifyBridge::new();
                let token = bridge
                    .refresh_token(credentials)
                    .await
                    .map_err(MilkError::from)?;
                Ok(token.access_token)
            }
            "youtube" => {
                let bridge = YouTubeBridge::new();
                let token = bridge
                    .refresh_token(credentials)
                    .await
                    .map_err(MilkError::from)?;
                Ok(token.access_token)
            }
            _ => Err(MilkError::Other(format!("Unknown service: {}", service))),
        }
    }

    /// Retry an operation with exponential backoff
    pub async fn retry_with_backoff<F, T, Fut>(operation: F, operation_name: &str) -> MilkResult<T>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = MilkResult<T>>,
    {
        let mut attempts = 0;
        let mut last_error = None;

        while attempts < MAX_RETRIES {
            match operation().await {
                Ok(result) => {
                    if attempts > 0 {
                        log_info(
                            "Recovery",
                            &format!("{} succeeded after {} retries", operation_name, attempts),
                        );
                    }
                    return Ok(result);
                }
                Err(e) => {
                    attempts += 1;
                    let error_msg = e.to_string();
                    let is_recoverable = e.is_recoverable();
                    last_error = Some(error_msg.clone());

                    // Check if error is recoverable
                    if !is_recoverable {
                        log_error(
                            "Recovery",
                            &format!(
                                "{} failed with non-recoverable error: {}",
                                operation_name, e
                            ),
                        );
                        return Err(e);
                    }

                    if attempts < MAX_RETRIES {
                        // Calculate exponential backoff delay
                        let delay_ms = BASE_DELAY_MS * 2u64.pow(attempts - 1);
                        log_warn(
                            "Recovery",
                            &format!(
                                "{} failed (attempt {}/{}), retrying in {}ms: {}",
                                operation_name, attempts, MAX_RETRIES, delay_ms, e
                            ),
                        );
                        sleep(Duration::from_millis(delay_ms)).await;
                    }
                }
            }
        }

        // All retries exhausted
        let error_msg = last_error.unwrap_or_else(|| "Unknown error".to_string());
        log_error(
            "Recovery",
            &format!(
                "{} failed after {} attempts: {}",
                operation_name, MAX_RETRIES, error_msg
            ),
        );
        Err(MilkError::Internal(error_msg))
    }

    /// Handle network timeout with retry
    pub async fn handle_network_timeout<F, T, Fut>(
        operation: F,
        operation_name: &str,
    ) -> MilkResult<T>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = MilkResult<T>>,
    {
        Self::retry_with_backoff(operation, operation_name).await
    }

    /// Validate and fix invalid paths
    pub fn validate_and_fix_path(path: &str) -> MilkResult<String> {
        use std::path::Path;

        let path_obj = Path::new(path);

        // Check if path exists
        if !path_obj.exists() {
            return Err(MilkError::InvalidPath(path.to_string()));
        }

        // Check if it's a directory
        if !path_obj.is_dir() {
            return Err(MilkError::InvalidPath(format!(
                "{} is not a directory",
                path
            )));
        }

        // Check if we have read permissions
        match std::fs::read_dir(path_obj) {
            Ok(_) => Ok(path.to_string()),
            Err(_) => Err(MilkError::PermissionDenied(path.to_string())),
        }
    }

    /// Check disk space before saving
    pub fn check_disk_space(path: &std::path::Path, required_bytes: u64) -> MilkResult<()> {
        // This is a simplified check - in production, you'd use platform-specific APIs
        // For now, we'll just try to write and catch the error
        match std::fs::metadata(path) {
            Ok(_) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::Other => {
                Err(MilkError::DiskFull(path.display().to_string()))
            }
            Err(e) => Err(MilkError::FileSystem(e)),
        }
    }

    /// Recover from rate limit error by waiting
    pub async fn handle_rate_limit() -> MilkResult<()> {
        log_warn("Recovery", "Rate limit hit, waiting 60 seconds");
        sleep(Duration::from_secs(60)).await;
        log_info("Recovery", "Rate limit wait complete");
        Ok(())
    }

    /// Get user-friendly recovery suggestion
    pub fn get_recovery_suggestion(error: &MilkError) -> String {
        match error {
            MilkError::NetworkTimeout(_) | MilkError::NetworkError(_) => {
                "Check your internet connection and try again.".to_string()
            }
            MilkError::AuthenticationFailed(_) => {
                "Please log in again to refresh your credentials.".to_string()
            }
            MilkError::InvalidPath(_) => "Please select a valid directory path.".to_string(),
            MilkError::PermissionDenied(_) => {
                "Please check file permissions or run as administrator.".to_string()
            }
            MilkError::DiskFull(_) => "Free up some disk space and try again.".to_string(),
            MilkError::CorruptedFile(_) | MilkError::ConfigParseError(_) => {
                "The file is corrupted. I'll create a fresh one for you.".to_string()
            }
            MilkError::RateLimitExceeded => {
                "Too many requests. Let's wait a moment and try again.".to_string()
            }
            MilkError::AudioDeviceUnavailable => {
                "No audio device found. Please check your speakers or headphones.".to_string()
            }
            MilkError::SkinParseError(_) | MilkError::InvalidSkinFormat(_) => {
                "That skin file didn't work. I'll use the default look instead.".to_string()
            }
            _ => "Let's try that again.".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recovery_suggestions() {
        let network_error = MilkError::NetworkTimeout("test".to_string());
        let suggestion = ErrorRecovery::get_recovery_suggestion(&network_error);
        assert!(suggestion.contains("internet"));

        let auth_error = MilkError::AuthenticationFailed("test".to_string());
        let suggestion = ErrorRecovery::get_recovery_suggestion(&auth_error);
        assert!(suggestion.contains("log in"));

        let path_error = MilkError::InvalidPath("test".to_string());
        let suggestion = ErrorRecovery::get_recovery_suggestion(&path_error);
        assert!(suggestion.contains("directory"));
    }

    #[test]
    fn test_config_recovery() {
        let corrupted_error = MilkError::ConfigParseError("test".to_string());
        let result = ErrorRecovery::recover_config_error(&corrupted_error);
        assert!(result.is_ok());

        let config = result.unwrap();
        assert_eq!(config.volume, 0.7); // Default value
    }

    #[tokio::test]
    async fn test_retry_with_backoff_success() {
        use std::sync::atomic::{AtomicU32, Ordering};
        use std::sync::Arc;

        let attempt = Arc::new(AtomicU32::new(0));
        let attempt_clone = Arc::clone(&attempt);

        let operation = move || {
            let attempt = Arc::clone(&attempt_clone);
            async move {
                let current = attempt.fetch_add(1, Ordering::SeqCst);
                if current < 1 {
                    Err(MilkError::NetworkTimeout("test".to_string()))
                } else {
                    Ok("success")
                }
            }
        };

        let result = ErrorRecovery::retry_with_backoff(operation, "test_operation").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "success");
    }

    #[tokio::test]
    async fn test_retry_with_backoff_non_recoverable() {
        let operation = || async { Err::<(), _>(MilkError::AudioDeviceUnavailable) };

        let result = ErrorRecovery::retry_with_backoff(operation, "test_operation").await;
        assert!(result.is_err());
    }
}
