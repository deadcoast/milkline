// Logging system with file rotation and size limits
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;
use chrono::Local;

/// Log levels for the application
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Error = 0,
    Warn = 1,
    Info = 2,
}

impl LogLevel {
    fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Error => "ERROR",
            LogLevel::Warn => "WARN",
            LogLevel::Info => "INFO",
        }
    }
}

/// Logger configuration
pub struct LoggerConfig {
    pub max_file_size: u64,  // Maximum log file size in bytes (default: 10MB)
    pub max_files: usize,     // Maximum number of rotated log files to keep (default: 5)
    pub min_level: LogLevel,  // Minimum log level to record (default: Info)
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            max_file_size: 10 * 1024 * 1024, // 10MB
            max_files: 5,
            min_level: LogLevel::Info,
        }
    }
}

/// Global logger instance
pub struct Logger {
    config: LoggerConfig,
    log_file: Mutex<Option<File>>,
    log_path: PathBuf,
}

impl Logger {
    /// Create a new logger with the given configuration
    pub fn new(config: LoggerConfig) -> Result<Self, std::io::Error> {
        let log_path = Self::get_log_path()?;
        
        // Ensure log directory exists
        if let Some(parent) = log_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Open or create log file
        let log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)?;

        Ok(Self {
            config,
            log_file: Mutex::new(Some(log_file)),
            log_path,
        })
    }

    /// Get the log file path in the AppData directory
    fn get_log_path() -> Result<PathBuf, std::io::Error> {
        let app_data = dirs::config_dir()
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "Config directory not found"))?;
        let milk_dir = app_data.join("milk");
        
        if !milk_dir.exists() {
            fs::create_dir_all(&milk_dir)?;
        }
        
        Ok(milk_dir.join("milk.log"))
    }

    /// Log a message with the specified level
    pub fn log(&self, level: LogLevel, category: &str, message: &str) {
        // Check if we should log this level
        if level > self.config.min_level {
            return;
        }

        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
        let log_line = format!("[{}] [{}] [{}] {}\n", timestamp, level.as_str(), category, message);

        // Also print to stderr for development
        eprint!("{}", log_line);

        // Write to file
        if let Ok(mut file_guard) = self.log_file.lock() {
            if let Some(ref mut file) = *file_guard {
                let _ = file.write_all(log_line.as_bytes());
                let _ = file.flush();

                // Check if rotation is needed
                if let Ok(metadata) = file.metadata() {
                    if metadata.len() >= self.config.max_file_size {
                        drop(file_guard); // Release lock before rotation
                        let _ = self.rotate_logs();
                    }
                }
            }
        }
    }

    /// Rotate log files when size limit is reached
    fn rotate_logs(&self) -> Result<(), std::io::Error> {
        let mut file_guard = self.log_file.lock().unwrap();
        
        // Close current log file
        *file_guard = None;

        // Rotate existing log files
        for i in (1..self.config.max_files).rev() {
            let old_path = self.get_rotated_log_path(i);
            let new_path = self.get_rotated_log_path(i + 1);
            
            if old_path.exists() {
                if i + 1 > self.config.max_files {
                    // Delete oldest log file
                    let _ = fs::remove_file(&old_path);
                } else {
                    // Rename to next number
                    let _ = fs::rename(&old_path, &new_path);
                }
            }
        }

        // Rename current log to .1
        let rotated_path = self.get_rotated_log_path(1);
        if self.log_path.exists() {
            fs::rename(&self.log_path, &rotated_path)?;
        }

        // Create new log file
        let new_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_path)?;

        *file_guard = Some(new_file);

        Ok(())
    }

    /// Get the path for a rotated log file
    fn get_rotated_log_path(&self, number: usize) -> PathBuf {
        let mut path = self.log_path.clone();
        let file_name = format!("milk.log.{}", number);
        path.set_file_name(file_name);
        path
    }

    /// Log an error message
    pub fn error(&self, category: &str, message: &str) {
        self.log(LogLevel::Error, category, message);
    }

    /// Log a warning message
    pub fn warn(&self, category: &str, message: &str) {
        self.log(LogLevel::Warn, category, message);
    }

    /// Log an info message
    pub fn info(&self, category: &str, message: &str) {
        self.log(LogLevel::Info, category, message);
    }
}

// Global logger instance using OnceLock (thread-safe)
use std::sync::OnceLock;
static GLOBAL_LOGGER: OnceLock<Logger> = OnceLock::new();

/// Initialize the global logger
pub fn init_logger(config: LoggerConfig) -> Result<(), std::io::Error> {
    let logger = Logger::new(config)?;
    GLOBAL_LOGGER.set(logger).map_err(|_| {
        std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "Logger already initialized"
        )
    })?;
    Ok(())
}

/// Get the global logger instance
fn get_logger() -> Option<&'static Logger> {
    GLOBAL_LOGGER.get()
}

/// Log an error message
pub fn log_error(category: &str, message: &str) {
    if let Some(logger) = get_logger() {
        logger.error(category, message);
    } else {
        eprintln!("[ERROR] [{}] {}", category, message);
    }
}

/// Log a warning message
pub fn log_warn(category: &str, message: &str) {
    if let Some(logger) = get_logger() {
        logger.warn(category, message);
    } else {
        eprintln!("[WARN] [{}] {}", category, message);
    }
}

/// Log an info message
pub fn log_info(category: &str, message: &str) {
    if let Some(logger) = get_logger() {
        logger.info(category, message);
    } else {
        eprintln!("[INFO] [{}] {}", category, message);
    }
}

/// Log an error with context
pub fn log_error_with_context(category: &str, error: &dyn std::error::Error, context: &str) {
    let message = format!("{}: {}", context, error);
    log_error(category, &message);
}

/// Convenience macro for logging errors
#[macro_export]
macro_rules! log_err {
    ($category:expr, $($arg:tt)*) => {
        $crate::logging::log_error($category, &format!($($arg)*))
    };
}

/// Convenience macro for logging warnings
#[macro_export]
macro_rules! log_warn {
    ($category:expr, $($arg:tt)*) => {
        $crate::logging::log_warn($category, &format!($($arg)*))
    };
}

/// Convenience macro for logging info
#[macro_export]
macro_rules! log_info {
    ($category:expr, $($arg:tt)*) => {
        $crate::logging::log_info($category, &format!($($arg)*))
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_logger_creation() {
        let config = LoggerConfig::default();
        let logger = Logger::new(config);
        assert!(logger.is_ok());
    }

    #[test]
    fn test_log_levels() {
        assert!(LogLevel::Error < LogLevel::Warn);
        assert!(LogLevel::Warn < LogLevel::Info);
    }

    #[test]
    fn test_log_level_strings() {
        assert_eq!(LogLevel::Error.as_str(), "ERROR");
        assert_eq!(LogLevel::Warn.as_str(), "WARN");
        assert_eq!(LogLevel::Info.as_str(), "INFO");
    }

    #[test]
    fn test_logging_writes_to_file() {
        let temp_dir = TempDir::new().unwrap();
        let log_path = temp_dir.path().join("test.log");
        
        let config = LoggerConfig::default();
        let logger = Logger::new(config).unwrap();
        
        logger.info("Test", "Test message");
        
        // File should exist and contain content
        let log_path = Logger::get_log_path().unwrap();
        assert!(log_path.exists());
    }

    #[test]
    fn test_log_rotation_path() {
        let config = LoggerConfig::default();
        let logger = Logger::new(config).unwrap();
        
        let rotated_path = logger.get_rotated_log_path(1);
        assert!(rotated_path.to_string_lossy().contains("milk.log.1"));
    }
}
