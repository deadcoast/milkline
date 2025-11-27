# Design Document

## Overview

`milk` is a Tauri-based desktop application that combines local audio playback, streaming service metadata integration, Winamp skin compatibility, and an animated companion system. The architecture follows a clean separation between the Svelte frontend (UI, visualization, state management) and Rust backend (file I/O, API integration, audio metadata extraction, configuration management).

The design prioritizes:
- **Lightweight footprint**: Target <15MB executable, <100MB RAM usage
- **Nostalgic authenticity**: Winamp `.wsz` skin parsing and rendering
- **Responsive UX**: Sub-2-second startup, 30+ FPS visualization, <2s metadata sync
- **Extensibility**: Plugin-ready API bridge pattern for future streaming services

## Architecture

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────┐
│                   milk (Tauri 2.x)                      │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  ┌───────────────────────────────────────────────────┐  │
│  │            Svelte Frontend Layer                  │  │
│  │                                                   │  │
│  │  ┌─────────────┐  ┌─────────────┐  ┌──────────┐   │  │
│  │  │   Player    │  │  Playlist   │  │ Visualiz │   │  │
│  │  │  Component  │  │  Component  │  │   er     │   │  │
│  │  └─────────────┘  └─────────────┘  └──────────┘   │  │
│  │                                                   │  │
│  │  ┌─────────────┐  ┌─────────────┐  ┌──────────┐   │  │
│  │  │    Skin     │  │   farmer    │  │  Stores  │   │  │
│  │  │   Renderer  │  │   Buddy     │  │  (State) │   │  │
│  │  └─────────────┘  └─────────────┘  └──────────┘   │  │
│  │                                                   │  │
│  └───────────────────────────────────────────────────┘  │
│                          │                              │
│                      Tauri IPC                          │
│                          │                              │
│  ┌───────────────────────────────────────────────────┐  │
│  │              Rust Backend Layer                   │  │
│  │                                                   │  │
│  │  ┌─────────────┐  ┌─────────────┐  ┌──────────┐   │  │
│  │  │   Config    │  │  API Bridge │  │  Local   │   │  │
│  │  │  Manager    │  │  (Spotify/  │  │ Library  │   │  │
│  │  │             │  │  YouTube)   │  │ Scanner  │   │  │
│  │  └─────────────┘  └─────────────┘  └──────────┘   │  │
│  │                                                   │  │
│  │  ┌─────────────┐  ┌─────────────┐  ┌──────────┐   │  │
│  │  │  Metadata   │  │    Skin     │  │  Secure  │   │  │
│  │  │  Extractor  │  │   Parser    │  │  Storage │   │  │
│  │  └─────────────┘  └─────────────┘  └──────────┘   │  │
│  │                                                   │  │
│  └───────────────────────────────────────────────────┘  │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

### Layer Responsibilities

**Frontend (Svelte)**
- UI rendering and user interaction
- Audio playback via HTML5 Audio API
- Real-time visualization using Web Audio API + Canvas
- State management via Svelte stores
- Skin rendering (CSS + dynamic asset loading)
- farmer animation state machine

**Backend (Rust)**
- File system operations (library scanning, config persistence)
- Audio metadata extraction (ID3, FLAC tags)
- Winamp skin parsing (`.wsz` archive extraction and asset mapping)
- External API communication (Spotify, YouTube)
- Secure credential storage (platform keyring integration)
- IPC command handlers

**Communication**
- Tauri IPC for all frontend-backend communication
- Event-driven updates (backend → frontend notifications)
- Command-response pattern for synchronous operations

## Components and Interfaces

### Frontend Components

#### Player Component
**Responsibilities:**
- Playback controls (play, pause, stop, next, previous)
- Volume control and seeking
- Display current track information
- Manage playback queue

**Interface:**
```typescript
interface PlayerState {
  currentTrack: Track | null;
  isPlaying: boolean;
  position: number;
  duration: number;
  volume: number;
  queue: Track[];
}

interface PlayerActions {
  play(track?: Track): void;
  pause(): void;
  stop(): void;
  next(): void;
  previous(): void;
  seek(position: number): void;
  setVolume(volume: number): void;
}
```

#### Playlist Component
**Responsibilities:**
- Display track list
- Handle drag-and-drop reordering
- Playlist CRUD operations
- Track selection

**Interface:**
```typescript
interface Playlist {
  id: string;
  name: string;
  tracks: Track[];
  createdAt: Date;
  modifiedAt: Date;
}

interface PlaylistActions {
  createPlaylist(name: string): Promise<Playlist>;
  addTrack(playlistId: string, track: Track): Promise<void>;
  removeTrack(playlistId: string, trackId: string): Promise<void>;
  reorderTracks(playlistId: string, newOrder: string[]): Promise<void>;
  loadPlaylist(playlistId: string): Promise<Playlist>;
}
```

#### Visualizer Component
**Responsibilities:**
- Real-time audio frequency analysis
- Canvas rendering (bars, waveform, spectrum)
- Frame rate management (target 30+ FPS)
- Visualization style switching

**Interface:**
```typescript
interface VisualizerConfig {
  style: 'bars' | 'waveform' | 'spectrum';
  fftSize: number;
  smoothing: number;
  colors: string[];
}

interface Visualizer {
  start(audioSource: MediaElementAudioSourceNode): void;
  stop(): void;
  setStyle(style: VisualizerConfig['style']): void;
  render(): void;
}
```

#### Skin Renderer
**Responsibilities:**
- Load and apply Winamp skins
- Map skin assets to UI regions
- Handle fallback to default skin
- CSS variable injection for colors

**Interface:**
```typescript
interface SkinAssets {
  main: string;        // Main window background
  titlebar: string;    // Title bar
  buttons: {
    play: string;
    pause: string;
    stop: string;
    next: string;
    previous: string;
  };
  posbar: string;      // Position bar
  volume: string;      // Volume slider
}

interface SkinRenderer {
  loadSkin(skinPath: string): Promise<SkinAssets>;
  applySkin(assets: SkinAssets): void;
  resetToDefault(): void;
}
```

#### farmer Buddy
**Responsibilities:**
- State machine management (idle, listening, prompting, celebrating, error)
- SVG animation control
- Speech bubble display
- Event-driven state transitions

**Interface:**
```typescript
type FarmerState = 'idle' | 'listening' | 'prompting' | 'celebrating' | 'error';

interface FarmerBuddy {
  state: FarmerState;
  transition(newState: FarmerState, message?: string): void;
  prompt(question: string, callback: (response: string) => void): void;
  celebrate(duration: number): void;
  showError(message: string): void;
}
```

### Backend Components

#### Config Manager
**Responsibilities:**
- Load/save application configuration
- Manage user preferences
- Handle configuration validation and defaults

**Interface:**
```rust
pub struct Config {
    pub library_path: Option<PathBuf>,
    pub last_skin: Option<String>,
    pub volume: f32,
    pub spotify_enabled: bool,
    pub youtube_enabled: bool,
}

pub trait ConfigManager {
    fn load() -> Result<Config, ConfigError>;
    fn save(&self, config: &Config) -> Result<(), ConfigError>;
    fn get_default() -> Config;
}
```

#### API Bridge
**Responsibilities:**
- Abstract external API communication
- Handle OAuth flows
- Poll for "Now Playing" updates
- Manage API rate limiting

**Interface:**
```rust
pub trait StreamingService {
    fn authenticate(&self, credentials: Credentials) -> Result<Token, ApiError>;
    fn get_now_playing(&self) -> Result<Option<TrackMetadata>, ApiError>;
    fn refresh_token(&self, token: &Token) -> Result<Token, ApiError>;
}

pub struct SpotifyBridge;
impl StreamingService for SpotifyBridge { /* ... */ }

pub struct YouTubeBridge;
impl StreamingService for YouTubeBridge { /* ... */ }
```

#### Local Library Scanner
**Responsibilities:**
- Recursive directory scanning
- File type filtering (mp3, flac, wav)
- Track indexing and caching

**Interface:**
```rust
pub struct LibraryScanner;

impl LibraryScanner {
    pub fn scan_directory(path: &Path) -> Result<Vec<Track>, ScanError>;
    pub fn watch_directory(path: &Path, callback: impl Fn(Track)) -> Result<(), ScanError>;
}
```

#### Metadata Extractor
**Responsibilities:**
- Parse ID3v2 tags (mp3)
- Parse FLAC/Vorbis comments
- Extract embedded album art
- Fallback to filename parsing

**Interface:**
```rust
pub struct MetadataExtractor;

impl MetadataExtractor {
    pub fn extract(file_path: &Path) -> Result<TrackMetadata, MetadataError>;
    pub fn extract_artwork(file_path: &Path) -> Result<Option<Vec<u8>>, MetadataError>;
}
```

#### Skin Parser
**Responsibilities:**
- Extract `.wsz` archives (ZIP format)
- Parse `region.txt` for window shaping
- Map BMP/PNG assets to UI regions
- Validate skin structure

**Interface:**
```rust
pub struct SkinParser;

impl SkinParser {
    pub fn parse_wsz(skin_path: &Path) -> Result<ParsedSkin, SkinError>;
    pub fn extract_assets(skin: &ParsedSkin) -> Result<HashMap<String, Vec<u8>>, SkinError>;
}

pub struct ParsedSkin {
    pub name: String,
    pub assets: HashMap<String, Vec<u8>>,
    pub regions: Option<RegionConfig>,
}
```

#### Secure Storage
**Responsibilities:**
- Store OAuth tokens securely
- Encrypt API keys
- Platform-native keyring integration (Windows Credential Manager)

**Interface:**
```rust
pub trait SecureStorage {
    fn store(&self, key: &str, value: &str) -> Result<(), StorageError>;
    fn retrieve(&self, key: &str) -> Result<Option<String>, StorageError>;
    fn delete(&self, key: &str) -> Result<(), StorageError>;
}
```

## Data Models

### Track
```typescript
interface Track {
  id: string;
  title: string;
  artist: string;
  album: string;
  duration: number;
  filePath?: string;        // For local files
  source: 'local' | 'spotify' | 'youtube';
  metadata: {
    year?: number;
    genre?: string;
    trackNumber?: number;
    albumArt?: string;      // Base64 or URL
  };
}
```

### Playlist
```typescript
interface Playlist {
  id: string;
  name: string;
  tracks: Track[];
  createdAt: Date;
  modifiedAt: Date;
}
```

### Configuration
```typescript
interface AppConfig {
  libraryPath: string | null;
  lastSkin: string | null;
  volume: number;
  visualizerStyle: 'bars' | 'waveform' | 'spectrum';
  spotifyEnabled: boolean;
  youtubeEnabled: boolean;
  windowPosition: { x: number; y: number };
  windowSize: { width: number; height: number };
}
```

### Skin Metadata
```typescript
interface SkinMetadata {
  name: string;
  author?: string;
  version?: string;
  assets: {
    [key: string]: string;  // Asset name → file path
  };
  regions?: {
    main: { x: number; y: number; width: number; height: number };
    // ... other regions
  };
}
```

### farmer State
```typescript
interface FarmerStateData {
  currentState: 'idle' | 'listening' | 'prompting' | 'celebrating' | 'error';
  message: string | null;
  expression: {
    eyes: 'neutral' | 'blink' | 'look-left' | 'look-right';
    mouth: 'neutral' | 'smile' | 'talk-1' | 'talk-2';
  };
}
```


## Correctness Properties

*A property is a characteristic or behavior that should hold true across all valid executions of a system—essentially, a formal statement about what the system should do. Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*

### Property 1: Supported audio format playback
*For any* valid audio file in supported formats (mp3, flac, wav), the application should successfully load and initiate playback without errors.
**Validates: Requirements 1.1**

### Property 2: Library scanning completeness
*For any* directory path containing audio files, the library scanner should index all files with supported extensions and no unsupported files.
**Validates: Requirements 1.2**

### Property 3: Playback position accuracy
*For any* audio file during playback, the reported playback position and duration should match the actual audio file properties within acceptable tolerance (±100ms).
**Validates: Requirements 1.3**

### Property 4: Volume control responsiveness
*For any* volume value in the valid range [0.0, 1.0], setting the volume should result in the audio output reflecting that volume level.
**Validates: Requirements 1.4**

### Property 5: Playback control state transitions
*For any* playback control command (play, pause, stop, next, previous), executing the command should transition the player to the corresponding state immediately.
**Validates: Requirements 1.5**

### Property 6: Streaming metadata completeness
*For any* track playing on Spotify or YouTube, the retrieved metadata should contain all required fields (title, artist/channel, album, duration) with non-empty values.
**Validates: Requirements 2.2, 3.2**

### Property 7: Streaming metadata sync timing
*For any* track change on Spotify or YouTube, the application should update the displayed metadata within 2 seconds of the change.
**Validates: Requirements 2.3, 3.3**

### Property 8: Skin parsing and asset mapping
*For any* valid `.wsz` skin file, the parser should successfully extract all assets and map them to the corresponding UI regions without missing required components.
**Validates: Requirements 4.1, 4.2**

### Property 9: Skin application completeness
*For any* successfully parsed skin, applying the skin should result in all UI components rendering with the skin's visual assets.
**Validates: Requirements 4.3**

### Property 10: Skin error fallback
*For any* corrupted or invalid skin file, the application should fall back to the default skin and display a farmer error notification.
**Validates: Requirements 4.4**

### Property 11: Skin persistence round-trip
*For any* successfully applied skin, restarting the application should load the same skin from configuration.
**Validates: Requirements 4.5**

### Property 12: Visualizer activation
*For any* audio playback (local, Spotify, YouTube), the visualizer should display real-time frequency or waveform data.
**Validates: Requirements 5.1, 5.4**

### Property 13: Visualizer frame rate
*For any* active visualization, the renderer should maintain at least 30 frames per second during continuous playback.
**Validates: Requirements 5.3**

### Property 14: Visualizer style switching
*For any* visualization style change during playback, the visualizer should switch modes without interrupting audio playback.
**Validates: Requirements 5.5**

### Property 15: farmer error state handling
*For any* invalid user input (non-existent path, invalid credentials), farmer should transition to error state and display an appropriate error message.
**Validates: Requirements 6.3**

### Property 16: farmer state machine transitions
*For any* valid state transition trigger (track start, track stop, config complete, error), farmer should transition to the appropriate state according to the state machine definition.
**Validates: Requirements 6.4, 7.1, 7.2, 7.3**

### Property 17: farmer listening state animations
*For any* audio playback, when farmer is in listening state, farmer should display animations that are synchronized with audio characteristics.
**Validates: Requirements 7.4**

### Property 18: Playlist persistence
*For any* playlist modification (create, add track, remove track, reorder), the changes should be immediately persisted to disk and retrievable after application restart.
**Validates: Requirements 9.1, 9.2, 9.5**

### Property 19: Playlist queue synchronization
*For any* playlist loaded into the player, the playback queue should contain all tracks in the same order as the playlist.
**Validates: Requirements 9.4**

### Property 20: Playlist track reordering
*For any* track reordering operation in a playlist, the playback queue should reflect the new order.
**Validates: Requirements 9.3**

### Property 21: Track removal non-destructive
*For any* track removed from a playlist, the original audio file should remain unmodified on disk.
**Validates: Requirements 9.5**

### Property 22: Configuration persistence round-trip
*For any* configuration change (volume, skin, library path), saving and then loading the configuration should produce the same values.
**Validates: Requirements 10.1, 10.2**

### Property 23: Configuration corruption recovery
*For any* corrupted or missing configuration file, the application should create a valid default configuration and notify the user via farmer.
**Validates: Requirements 10.3**

### Property 24: API bridge interface compliance
*For any* streaming service integration, the implementation should conform to the common StreamingService trait interface.
**Validates: Requirements 11.4**

### Property 25: Metadata extraction completeness
*For any* local audio file with embedded metadata, the extractor should retrieve all available standard fields (title, artist, album, year, genre, track number).
**Validates: Requirements 12.1**

### Property 26: Metadata fallback parsing
*For any* audio file with missing or incomplete metadata, the application should derive track information from the filename and directory structure.
**Validates: Requirements 12.2**

### Property 27: Album art extraction
*For any* audio file with embedded album art, the extractor should successfully retrieve and decode the artwork data.
**Validates: Requirements 12.3**

### Property 28: Metadata caching efficiency
*For any* audio file, accessing metadata a second time should use cached data without re-parsing the file.
**Validates: Requirements 12.4**

## Error Handling

### Error Categories

**File System Errors**
- Invalid paths (non-existent directories, inaccessible files)
- Permission denied (read/write restrictions)
- Disk full (configuration save failures)
- Corrupted files (invalid audio, malformed skins)

**Network/API Errors**
- Authentication failures (invalid credentials, expired tokens)
- Rate limiting (API quota exceeded)
- Network timeouts (connection failures)
- Invalid responses (malformed JSON, missing fields)

**Playback Errors**
- Unsupported formats (codecs not available)
- Corrupted audio data (decode failures)
- Audio device unavailable (no output device)

**Configuration Errors**
- Invalid configuration values (out-of-range volume, invalid paths)
- Corrupted configuration files (JSON parse errors)
- Missing required fields

### Error Handling Strategy

**Graceful Degradation**
- Fall back to default skin when custom skin fails
- Fall back to filename parsing when metadata extraction fails
- Continue with local playback when streaming services are unavailable

**User Notification via farmer**
- All errors that require user action trigger farmer error state
- Error messages are clear, actionable, and non-technical
- farmer provides guidance on how to resolve the issue

**Logging**
- All errors are logged to application log file with full context
- Log levels: ERROR (user-facing issues), WARN (degraded functionality), INFO (state changes)
- Logs include timestamps, component names, and error details

**Recovery Mechanisms**
- Automatic token refresh for expired OAuth credentials
- Retry logic for transient network failures (exponential backoff)
- Configuration reset to defaults when corruption detected
- Playlist recovery from backup when primary file is corrupted

## Testing Strategy

### Unit Testing

**Backend (Rust)**
- Config manager: load/save operations, default generation, validation
- Metadata extractor: ID3 parsing, FLAC parsing, fallback parsing, artwork extraction
- Skin parser: `.wsz` extraction, asset mapping, region parsing
- API bridge: OAuth flow, token refresh, metadata retrieval
- Library scanner: directory traversal, file filtering, indexing

**Frontend (Svelte)**
- Player component: state management, control handlers, queue management
- Playlist component: CRUD operations, reordering logic
- Skin renderer: asset loading, CSS injection, fallback handling
- farmer buddy: state machine transitions, animation control

### Property-Based Testing

**Framework**: `proptest` for Rust backend, `fast-check` for TypeScript frontend

**Configuration**:
- Minimum 100 iterations per property test
- Each property test must reference the design document property using the format: `**Feature: milk-player, Property {number}: {property_text}**`

**Test Coverage**:
- Each correctness property (1-28) should be implemented as a property-based test
- Generators should produce realistic test data (valid audio files, playlists, configurations)
- Edge cases should be covered by generators (empty playlists, maximum volume, zero duration)

**Example Property Test Structure**:
```rust
// **Feature: milk-player, Property 2: Library scanning completeness**
#[test]
fn prop_library_scanning_completeness() {
    proptest!(|(directory in arb_directory_with_audio_files())| {
        let scanned = LibraryScanner::scan_directory(&directory)?;
        let expected = count_supported_files(&directory);
        prop_assert_eq!(scanned.len(), expected);
    });
}
```

### Integration Testing

**End-to-End Scenarios**:
- First launch flow: farmer prompts → library setup → skin application → playback
- Playlist workflow: create → add tracks → reorder → save → load → play
- Streaming integration: authenticate → sync metadata → display in UI
- Skin switching: load skin → apply → verify UI update → persist → restart → verify loaded

**Performance Testing**:
- Startup time measurement (target <2s)
- Memory usage monitoring (target <100MB idle)
- Visualizer frame rate measurement (target 30+ FPS)
- Metadata sync latency (target <2s)

### Manual Testing

**Visual/UX Testing**:
- Skin rendering accuracy (compare with Winamp)
- farmer animations smoothness and timing
- Visualizer aesthetics and synchronization
- UI responsiveness and feel

**Platform Testing**:
- Windows 10/11 compatibility
- Different screen resolutions and DPI settings
- Various audio output devices

## Performance Considerations

### Memory Management

**Target**: <100MB RAM during idle playback

**Strategies**:
- Lazy loading of playlist tracks (load metadata on-demand)
- Bounded cache for metadata (LRU eviction, max 1000 entries)
- Release visualizer resources when not visible
- Minimize Tauri IPC payload sizes

### Startup Optimization

**Target**: <2 seconds to main window display

**Strategies**:
- Defer non-critical initialization (streaming service connections)
- Load configuration asynchronously
- Render default skin immediately, load custom skin in background
- Lazy load farmer animations

### Visualizer Performance

**Target**: 30+ FPS during playback

**Strategies**:
- Use requestAnimationFrame for rendering loop
- Optimize FFT size (balance between quality and performance)
- Use hardware-accelerated Canvas rendering
- Throttle updates when window is not focused

### File System Operations

**Strategies**:
- Async I/O for all file operations (Tokio runtime)
- Batch playlist saves (debounce rapid changes)
- Use memory-mapped files for large audio files
- Index library in background thread

## Security Considerations

### Credential Storage

- Use platform-native secure storage (Windows Credential Manager)
- Never log or display OAuth tokens or API keys
- Encrypt credentials at rest using platform APIs
- Clear credentials from memory after use

### API Communication

- Use HTTPS for all external API calls
- Validate SSL certificates
- Implement request signing where required (Spotify, YouTube)
- Rate limit API calls to prevent abuse

### File System Access

- Validate all user-provided paths (prevent directory traversal)
- Sanitize filenames before file operations
- Respect file system permissions
- Limit file size for skin parsing (prevent zip bombs)

### Input Validation

- Validate all IPC command parameters
- Sanitize user input before display (prevent XSS in farmer messages)
- Validate configuration values before applying
- Limit string lengths to prevent buffer issues

## Deployment

### Build Artifacts

- Windows executable: `milk.exe` (~10-15MB)
- MSI installer: `milk_x.x.x_x64.msi`
- Portable ZIP: `milk_portable.zip` (includes default skin and assets)

### Installation

- MSI installer handles:
  - Program Files installation
  - Start menu shortcuts
  - File associations (.wsz, .wal)
  - Uninstaller registration

### Updates

- Manual update check via Help menu
- Download new MSI from release page
- Configuration and playlists preserved across updates

### Distribution

- GitHub Releases (primary)
- Direct download from project website
- No auto-update mechanism (user-initiated only)
