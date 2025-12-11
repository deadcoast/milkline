# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

milk is a lightweight (~10MB) desktop audio player inspired by Winamp, built with Tauri 2.x (Rust backend) and Svelte 5 (frontend). It features local audio playback, Winamp skin compatibility, real-time visualization, streaming service metadata integration (Spotify/YouTube), and an animated "farmer" companion that guides users through setup.

**Tech Stack**: Rust + Tauri 2.x | Svelte 5 + TypeScript | pnpm | Windows (x86_64-pc-windows-msvc)

## Essential Commands

### Development

```bash
pnpm install                    # Install dependencies
pnpm tauri:dev                  # Run development server (Rust + Svelte)
pnpm dev                        # Frontend only (for quick UI iteration)
```

### Testing

```bash
pnpm test                       # Run all tests once (Vitest + proptest)
pnpm test:watch                 # Run tests in watch mode
pnpm test:ui                    # Run tests with UI viewer
pnpm check                      # TypeScript type checking

# Rust tests (run from src-tauri/)
cargo test                      # Unit + property-based tests
cargo test -- --nocapture       # Run with output
```

### Building

```bash
pnpm tauri:build                # Production build (creates MSI + NSIS installers)
pnpm tauri:build:debug          # Debug build with symbols

# Verify build
.\scripts\verify-build.ps1      # PowerShell script to check build artifacts

# Create portable distribution
.\scripts\create-portable.ps1
```

### Code Quality

```bash
# Rust
cargo clippy                    # Linting
cargo fmt                       # Format code
cargo test                      # Run tests

# TypeScript/Svelte
pnpm check                      # Type check
pnpm check:watch                # Type check in watch mode
```

## Architecture

### Backend (Rust - src-tauri/src/)

**Core Modules**:

- `lib.rs` - Main entry point, Tauri command registration, global state management
- `config.rs` - Application configuration persistence (JSON in user config dir)
- `library.rs` - Local audio file scanning and track detection
- `metadata.rs` - ID3/FLAC metadata extraction with caching
- `playlist.rs` - Playlist management with JSON persistence
- `skin.rs` - Winamp .wsz/.wal skin parser and validator
- `spotify.rs` - Spotify API bridge (OAuth 2.0, now playing metadata)
- `youtube.rs` - YouTube API bridge (OAuth 2.0 + API key, video metadata)
- `secure_storage.rs` - Platform-specific credential storage (keyring)
- `error.rs` - Unified error handling with `MilkError` enum
- `logging.rs` - Structured logging with automatic file rotation
- `performance.rs` - Performance metrics tracking

**Global State**: Uses `OnceLock` for lazy-initialized singletons (metadata extractor, playlist manager, API bridges). Access via helper functions like `get_metadata_extractor()`, `get_playlist_manager()`.

**IPC Commands**: All Tauri commands follow pattern: `#[tauri::command] fn command_name(...) -> Result<T, String>`. Commands return `Result` with `MilkError` converted to user-friendly messages via `.user_message()`.

**Error Handling**: All errors convert through `MilkError` which provides:

- `is_critical()` - Requires immediate user attention
- `is_recoverable()` - Can be handled gracefully with fallback
- `user_message()` - User-friendly error text displayed via farmer

**Testing**: Property-based tests using `proptest` for validation logic (see `src-tauri/proptest-regressions/` for regression tests).

### Frontend (Svelte - src/lib/)

**Store Architecture** (src/lib/stores/):

- `playerStore.ts` - Playback state (current track, play/pause, queue, volume)
- `playlistStore.ts` - Playlist CRUD operations
- `configStore.ts` - App configuration state
- `farmerStore.ts` - Farmer companion state machine
- `farmerPlayerSync.ts` - **Critical**: Syncs player state → farmer reactions
- `metadataCache.ts` - Frontend cache for track metadata
- `index.ts` - Store exports

**Components** (src/lib/components/):

- `Player.svelte` - Audio player controls
- `Playlist.svelte` - Playlist UI
- `Visualizer.svelte` - Audio visualization (Web Audio API + Canvas)
- `SkinRenderer.svelte` - Winamp skin rendering
- `FarmerBuddy.svelte` - Animated companion (state machine: idle | listening | prompting | celebrating | error)
- `SetupWizard.svelte` - First-run setup flow

**IPC Layer** (src/lib/tauri/):

- `ipc.ts` - Typed wrappers for all Tauri commands
- Pattern: Each command has both unsafe (`scanLibrary`) and safe (`scanLibrarySafe`) variants
- Safe variants use `handleError()` from `errorHandler.ts` for automatic error display

**Types** (src/lib/types.ts): All TypeScript interfaces for Track, Playlist, AppConfig, PlayerState, FarmerState, etc.

### Data Flow

1. **Playback Flow**: User clicks play → `playerStore.setPlaying(true)` → triggers `farmerPlayerSync` → farmer transitions to "listening" state
2. **Library Scan**: User selects folder → `scanLibrary()` IPC → Rust scans filesystem → returns Track[] → updates stores
3. **Streaming Metadata**: Background polling → `spotify_get_now_playing()` → updates current track metadata if local track matches
4. **Skin Loading**: User selects .wsz file → `load_skin()` → Rust parses ZIP → returns assets as byte arrays → `SkinRenderer` converts to data URLs

## Key Development Patterns

### Adding New Tauri Commands

1. **Define in Rust** (src-tauri/src/lib.rs or module):

   ```rust
   #[tauri::command]
   fn my_command(param: String) -> Result<MyType, String> {
       log_info("Module", "Doing something");
       match do_something(&param) {
           Ok(result) => Ok(result),
           Err(e) => {
               let milk_err = MilkError::from(e);
               log_error("Module", &format!("Failed: {}", milk_err));
               Err(milk_err.user_message())
           }
       }
   }
   ```

2. **Register command** in `run()` function's `.invoke_handler()` macro

3. **Add TypeScript wrapper** (src/lib/tauri/ipc.ts):

   ```typescript
   export async function myCommand(param: string): Promise<MyType> {
     return await invoke<MyType>("my_command", { param });
   }

   // Add safe variant with error handling
   export async function myCommandSafe(param: string): Promise<MyType | null> {
     try {
       return await myCommand(param);
     } catch (error) {
       handleError(error, "Failed to do something");
       return null;
     }
   }
   ```

### Error Handling

**Always** convert errors through `MilkError`:

- Rust: `MilkError::from(e)` then `.user_message()`
- Frontend: Use `handleError()` from `src/lib/utils/errorHandler.ts`
- Log appropriately: `log_error()` for critical, `log_warn()` for recoverable, `log_info()` for normal operations

### State Management

**Farmer-Player Sync is Critical**: If you modify `playerStore` state related to playback (isPlaying, currentTrack), ensure `farmerPlayerSync.ts` handles the transition. The sync runs automatically via subscription.

**Store Pattern**:

```typescript
function createMyStore() {
  const { subscribe, set, update } = writable<State>(initial);
  return {
    subscribe,
    myMethod: (arg: T) => update((state) => ({ ...state, field: arg })),
    reset: () => set(initial),
  };
}
```

### Testing Practices

**Frontend** (Vitest):

- Test files: `*.test.ts` next to component/module
- Mock Tauri IPC in setup: see `src/test/setup.ts`
- Use `@testing-library/svelte` for component tests

**Backend** (Rust):

- Unit tests: `#[cfg(test)] mod tests` in same file
- Property-based: Use `proptest!` macro for validation logic
- Integration: Property tests in `src-tauri/src/*_tests.rs`

## Build Configuration

**Release Profile** (Cargo.toml):

- `opt-level = "z"` - Optimize for size
- `lto = true` - Link-time optimization
- `codegen-units = 1` - Better optimization
- `strip = true` - Remove debug symbols
- Target: <15MB executable

**Tauri Config** (src-tauri/tauri.conf.json):

- Single-page app mode (SPA) via `adapter-static`
- Fixed dev server port: 1420
- Capabilities defined in `src-tauri/capabilities/default.json`

## Performance Requirements

**Hard Targets**:

- Executable size: <15MB
- RAM usage (idle): <100MB
- Startup time: <2 seconds
- Visualizer FPS: 30+

Track metrics using `get_performance_metrics()` command.

## Common Issues

**Tauri Commands Not Working**:

1. Check command is registered in `.invoke_handler()`
2. Verify parameter names match between Rust (snake_case) and TypeScript (camelCase in invoke call)
3. Check return type serialization (must impl `Serialize`)

**Farmer Not Reacting**: Check `farmerPlayerSync.ts` subscription is active. Initialize in root component with `initializeFarmerPlayerSync()`.

**Skin Not Loading**: Skins automatically fall back to default on error (graceful degradation). Check logs for actual error. Use `SkinParser::validate_skin()` to debug.

**Tests Failing with Tauri Errors**: Ensure `src/test/setup.ts` is imported in Vitest config and mocks `__TAURI_INTERNALS__`.

## Documentation

Full documentation in `docs/`:

- **docs/README.md** - Documentation map
- **docs/BUILD.md** - Comprehensive build guide
- **docs/ERROR_HANDLING.md** - Error handling patterns
- **docs/PERFORMANCE_OPTIMIZATIONS.md** - Performance optimization strategies
- **docs/milk_tech_spec.md** - Detailed technical specification

## Deployment

1. Build: `pnpm tauri:build`
2. Verify: `.\scripts\verify-build.ps1`
3. Test installation: Install MSI on clean VM
4. Release artifacts: MSI installer, NSIS installer, portable ZIP
5. Follow `docs/RELEASE_CHECKLIST.md`

## Notes for Claude

- **Streaming Services**: Spotify requires Premium for playback control. YouTube uses embedded IFrame API (ToS compliant).
- **Farmer Buddy**: Not AI - script-driven state machine. States: idle | listening | prompting | celebrating | error.
- **Winamp Compatibility**: Supports .wsz (ZIP) and .wal (skin files). Parser handles legacy formats gracefully.
- **Performance Critical**: Every feature must maintain <15MB binary size. Avoid heavy dependencies.
- **First Run**: `SetupWizard.svelte` handles initial setup flow. Check `is_first_run()` command.
