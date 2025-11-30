# Error Handling and Logging System

> 📚 **Documentation Map**: See [docs/README.md](README.md) for all documentation  
> 📖 **Technical Spec**: See [milk_tech_spec.md](milk_tech_spec.md) for architecture details

## Overview

The milk application implements comprehensive error handling and logging to ensure:
- User-friendly error messages displayed via the farmer buddy
- Graceful degradation for non-critical failures
- Detailed logging for debugging and troubleshooting
- Automatic log file rotation to prevent disk space issues

## Error Types

All errors in the application are categorized using the `MilkError` enum, which provides:

### Error Categories

1. **File System Errors**
   - `FileSystem(io::Error)` - General I/O errors
   - `InvalidPath(String)` - Invalid or non-existent paths
   - `PermissionDenied(String)` - Access permission issues
   - `DiskFull(String)` - Disk space exhausted
   - `CorruptedFile(String)` - File corruption detected

2. **Network/API Errors**
   - `AuthenticationFailed(String)` - OAuth or API auth failures
   - `RateLimitExceeded` - API rate limits hit
   - `NetworkTimeout(String)` - Connection timeouts
   - `InvalidResponse(String)` - Malformed API responses
   - `NetworkError(String)` - General network issues

3. **Playback Errors**
   - `UnsupportedFormat(String)` - Unsupported audio formats
   - `DecodeError(String)` - Audio decoding failures
   - `AudioDeviceUnavailable` - No audio output device

4. **Configuration Errors**
   - `InvalidConfig(String)` - Invalid configuration values
   - `ConfigParseError(String)` - Configuration file parsing errors
   - `MissingConfig(String)` - Required configuration missing

5. **Skin Errors**
   - `SkinParseError(String)` - Skin file parsing errors
   - `InvalidSkinFormat(String)` - Unsupported skin format
   - `MissingSkinAssets(String)` - Required skin assets missing

6. **Metadata Errors**
   - `MetadataError(String)` - Metadata extraction failures

7. **Playlist Errors**
   - `PlaylistNotFound(String)` - Playlist doesn't exist
   - `InvalidPlaylistOperation(String)` - Invalid playlist operation

8. **Storage Errors**
   - `SecureStorageError(String)` - Secure credential storage issues

## Error Properties

Each error has the following properties:

### `is_critical() -> bool`
Indicates if the error requires immediate user attention:
- Disk full
- Permission denied
- Audio device unavailable
- Authentication failures

### `is_recoverable() -> bool`
Indicates if the error can be handled gracefully:
- Network timeouts
- Rate limit exceeded
- Corrupted files (fallback to defaults)
- Skin/metadata errors (fallback available)

### `user_message() -> String`
Returns a friendly, actionable message suitable for display via farmer:
```rust
let err = MilkError::InvalidPath("/some/path".to_string());
println!("{}", err.user_message());
// Output: "Hmm, I can't find that path: /some/path"
```

### `category() -> &'static str`
Returns the error category for logging purposes:
- "FileSystem"
- "Network"
- "Playback"
- "Configuration"
- "Skin"
- "Metadata"
- "Playlist"
- "Storage"
- "General"

## Logging System

### Log Levels

The application supports three log levels:
- `ERROR` - Critical issues requiring user attention
- `WARN` - Non-critical issues with graceful degradation
- `INFO` - Normal operational messages

### Configuration

```rust
use logging::{LoggerConfig, init_logger};

let config = LoggerConfig {
    max_file_size: 10 * 1024 * 1024,  // 10MB
    max_files: 5,                      // Keep 5 rotated files
    min_level: LogLevel::Info,         // Log INFO and above
};

init_logger(config)?;
```

### Usage

```rust
use logging::{log_error, log_warn, log_info};

// Log an error
log_error("Config", "Failed to load configuration file");

// Log a warning
log_warn("Skin", "Skin validation failed, using default");

// Log info
log_info("Startup", "Application initialized successfully");
```

### Macros

Convenience macros are available for formatted logging:

```rust
use crate::{log_err, log_warn, log_info};

log_err!("Network", "Connection failed: {}", error_msg);
log_warn!("Metadata", "Missing tag: {}", tag_name);
log_info!("Playlist", "Created playlist: {}", playlist_id);
```

### Log File Location

Logs are stored in the application's config directory:
- **Windows**: `%APPDATA%\milk\milk.log`
- **macOS**: `~/Library/Application Support/milk/milk.log`
- **Linux**: `~/.config/milk/milk.log`

### Log Rotation

When a log file reaches the configured size limit (default 10MB):
1. Current log is renamed to `milk.log.1`
2. Previous rotated logs are incremented (`milk.log.1` → `milk.log.2`)
3. Oldest log beyond the limit is deleted
4. New log file is created

This ensures logs don't consume excessive disk space while maintaining history.

## Frontend Error Handling

### Error Handler Utility

The frontend provides utilities for connecting backend errors to the farmer state:

```typescript
import { handleError, withErrorHandling, showSuccess } from '$lib/utils/errorHandler';

// Handle an error and show via farmer
try {
    await someOperation();
} catch (error) {
    handleError(error, 'Operation failed');
}

// Wrap async operations with automatic error handling
const result = await withErrorHandling(
    () => loadConfig(),
    'Failed to load configuration',
    (config) => console.log('Config loaded:', config)
);

// Show success message
showSuccess('Configuration saved!', 2000);
```

### Safe IPC Wrappers

Safe wrappers are provided for common IPC operations:

```typescript
import { 
    loadConfigSafe, 
    saveConfigSafe, 
    scanLibrarySafe,
    loadSkinSafe,
    applySkinSafe 
} from '$lib/tauri/ipc';

// These automatically handle errors and show via farmer
const config = await loadConfigSafe();
const tracks = await scanLibrarySafe('/path/to/music');
const skin = await applySkinSafe('/path/to/skin.wsz');
```

## Graceful Degradation

The application implements graceful degradation for non-critical failures:

### Skin Loading
If a custom skin fails to load or validate:
1. Error is logged as WARNING
2. Default skin is applied automatically
3. User is notified via farmer
4. Application continues normally

### Metadata Extraction
If metadata extraction fails:
1. Error is logged as WARNING
2. Fallback parsing from filename/directory is attempted
3. Basic track info is still available
4. Playback is not affected

### Configuration Corruption
If configuration file is corrupted:
1. Error is logged as ERROR
2. Default configuration is created
3. User is notified via farmer
4. Application starts with defaults

### Network Failures
If streaming service API calls fail:
1. Error is logged based on severity
2. Local playback continues unaffected
3. User is notified if authentication is needed
4. Automatic retry with exponential backoff

## Best Practices

### Backend (Rust)

1. **Always use MilkError for public APIs**
   ```rust
   pub fn some_operation() -> Result<T, MilkError> {
       // ...
   }
   ```

2. **Log before returning errors**
   ```rust
   match operation() {
       Ok(result) => Ok(result),
       Err(e) => {
           let milk_err = MilkError::from(e);
           log_error("Category", &format!("Operation failed: {}", milk_err));
           Err(milk_err.user_message())
       }
   }
   ```

3. **Use appropriate log levels**
   - ERROR: User-facing issues, critical failures
   - WARN: Degraded functionality, recoverable errors
   - INFO: Normal operations, state changes

4. **Provide context in log messages**
   ```rust
   log_info("Playlist", &format!("Created playlist: {} with {} tracks", id, count));
   ```

### Frontend (TypeScript)

1. **Use safe wrappers for IPC calls**
   ```typescript
   const config = await loadConfigSafe();
   if (!config) {
       // Handle null case (error already shown via farmer)
       return;
   }
   ```

2. **Use withErrorHandling for complex operations**
   ```typescript
   await withErrorHandling(
       async () => {
           const config = await loadConfig();
           await processConfig(config);
           await saveConfig(config);
       },
       'Failed to update configuration'
   );
   ```

3. **Show success feedback**
   ```typescript
   await saveConfig(config);
   showSuccess('Settings saved!');
   ```

## Testing

Error handling is tested at multiple levels:

### Unit Tests
```rust
#[test]
fn test_error_user_messages() {
    let err = MilkError::InvalidPath("/path".to_string());
    assert!(err.user_message().contains("can't find"));
}
```

### Integration Tests
Verify that errors propagate correctly through the IPC layer and trigger farmer state changes.

### Property-Based Tests
Ensure error handling works correctly across a wide range of inputs and error conditions.

## Requirements Validation

This implementation satisfies:
- **Requirement 6.3**: Farmer transitions to error state and displays helpful messages
- **Requirement 10.3**: Configuration corruption recovery with user notification

The error handling system ensures the application remains stable and user-friendly even when things go wrong.

## Related Documentation

- **[Technical Specification](milk_tech_spec.md)** - Architecture and design
- **[BUILD.md](BUILD.md)** - Build and packaging guide
- **[INSTALLATION_TESTING.md](INSTALLATION_TESTING.md)** - Testing procedures
- **[PERFORMANCE_OPTIMIZATIONS.md](PERFORMANCE_OPTIMIZATIONS.md)** - Performance guide

---

📚 [Back to Documentation Map](README.md)
