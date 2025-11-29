# Implementation Plan

- [x] 1. Initialize Tauri project structure and core configuration





  - Set up Tauri 2.x project with Rust backend and Svelte frontend
  - Configure build targets for Windows (x86_64-pc-windows-msvc)
  - Set up pnpm workspace and dependencies
  - Create directory structure (src/, src-tauri/, assets/)
  - Configure Tauri window properties (size, resizable, decorations)
  - _Requirements: 8.1, 8.3_

- [x] 2. Implement configuration management system





  - Create Rust Config struct with all application settings
  - Implement ConfigManager trait with load/save/default methods
  - Add JSON serialization/deserialization for configuration
  - Implement configuration file path resolution (AppData directory)
  - Create Tauri IPC commands for config operations (load_config, save_config)
  - _Requirements: 10.1, 10.2, 10.3_

- [x] 2.1 Write property test for configuration round-trip

  - **Property 22: Configuration persistence round-trip**
  - **Validates: Requirements 10.1, 10.2**


- [x] 2.2 Write property test for configuration corruption recovery

  - **Property 23: Configuration corruption recovery**
  - **Validates: Requirements 10.3**

- [x] 3. Implement secure credential storage




  - Integrate platform-native secure storage library (keyring-rs)
  - Create SecureStorage trait with store/retrieve/delete methods
  - Implement Windows Credential Manager integration
  - Add encryption for sensitive data (OAuth tokens, API keys)
  - Create Tauri IPC commands for credential operations
  - _Requirements: 2.1, 3.1, 10.4_

- [x] 4. Build local audio library scanner and indexer





  - Create LibraryScanner struct with directory scanning logic
  - Implement recursive directory traversal with file filtering
  - Add support for mp3, flac, wav file detection
  - Create Track data model with file path and basic info
  - Implement Tauri IPC command for library scanning (scan_library)
  - _Requirements: 1.2_

- [x] 4.1 Write property test for library scanning completeness


  - **Property 2: Library scanning completeness**
  - **Validates: Requirements 1.2**

- [x] 5. Implement audio metadata extraction





  - Integrate metadata parsing libraries (id3, metaflac)
  - Create MetadataExtractor with extract and extract_artwork methods
  - Implement ID3v2 tag parsing for mp3 files
  - Implement FLAC/Vorbis comment parsing
  - Add fallback filename/directory parsing for missing metadata
  - Implement metadata caching with LRU eviction (max 1000 entries)
  - _Requirements: 12.1, 12.2, 12.3, 12.4_

- [x] 5.1 Write property test for metadata extraction completeness


  - **Property 25: Metadata extraction completeness**
  - **Validates: Requirements 12.1**


- [x] 5.2 Write property test for metadata fallback parsing

  - **Property 26: Metadata fallback parsing**
  - **Validates: Requirements 12.2**

- [x] 5.3 Write property test for album art extraction


  - **Property 27: Album art extraction**
  - **Validates: Requirements 12.3**


- [x] 5.4 Write property test for metadata caching efficiency

  - **Property 28: Metadata caching efficiency**
  - **Validates: Requirements 12.4**

- [x] 6. Create Svelte frontend foundation and state management





  - Set up Svelte project with TypeScript
  - Create Svelte stores for player state, playlist state, config state
  - Implement Tauri IPC client wrapper functions
  - Create base App.svelte with layout structure
  - Set up CSS foundation and variables for theming
  - _Requirements: 11.3_

- [x] 7. Build Player component with playback controls





  - Create Player.svelte component with UI controls
  - Implement HTML5 Audio element integration
  - Add playback control handlers (play, pause, stop, next, previous)
  - Implement volume control with real-time adjustment
  - Add seek functionality with position bar
  - Display current track information (title, artist, album, duration)
  - Implement playback queue management
  - _Requirements: 1.1, 1.3, 1.4, 1.5_

- [x] 7.1 Write property test for supported audio format playback


  - **Property 1: Supported audio format playback**
  - **Validates: Requirements 1.1**

- [x] 7.2 Write property test for playback position accuracy


  - **Property 3: Playback position accuracy**
  - **Validates: Requirements 1.3**

- [x] 7.3 Write property test for volume control responsiveness


  - **Property 4: Volume control responsiveness**
  - **Validates: Requirements 1.4**

- [x] 7.4 Write property test for playback control state transitions


  - **Property 5: Playback control state transitions**
  - **Validates: Requirements 1.5**

- [x] 8. Implement playlist management system





  - Create Playlist data model with id, name, tracks, timestamps
  - Implement Rust playlist persistence (JSON files in AppData)
  - Create Tauri IPC commands for playlist CRUD operations
  - Build Playlist.svelte component with track list display
  - Add drag-and-drop reordering functionality
  - Implement playlist loading into playback queue
  - _Requirements: 9.1, 9.2, 9.3, 9.4, 9.5_

- [x] 8.1 Write property test for playlist persistence


  - **Property 18: Playlist persistence**
  - **Validates: Requirements 9.1, 9.2, 9.5**

- [x] 8.2 Write property test for playlist queue synchronization


  - **Property 19: Playlist queue synchronization**
  - **Validates: Requirements 9.4**

- [x] 8.3 Write property test for playlist track reordering


  - **Property 20: Playlist track reordering**
  - **Validates: Requirements 9.3**

- [x] 8.4 Write property test for track removal non-destructive


  - **Property 21: Track removal non-destructive**
  - **Validates: Requirements 9.5**

- [x] 9. Checkpoint - Ensure all tests pass





  - Ensure all tests pass, ask the user if questions arise.

- [x] 10. Build Winamp skin parser and renderer





  - Create SkinParser struct for .wsz file handling
  - Implement ZIP archive extraction for .wsz files
  - Parse skin assets (BMP/PNG) and map to UI regions
  - Implement region.txt parsing for window shaping
  - Create SkinRenderer in Svelte for applying skin assets
  - Add CSS variable injection for skin colors
  - Implement default skin fallback mechanism
  - Add Tauri IPC commands for skin operations (load_skin, apply_skin)
  - _Requirements: 4.1, 4.2, 4.3, 4.4, 4.5_

- [x] 10.1 Write property test for skin parsing and asset mapping


  - **Property 8: Skin parsing and asset mapping**
  - **Validates: Requirements 4.1, 4.2**

- [x] 10.2 Write property test for skin application completeness


  - **Property 9: Skin application completeness**
  - **Validates: Requirements 4.3**

- [x] 10.3 Write property test for skin error fallback


  - **Property 10: Skin error fallback**
  - **Validates: Requirements 4.4**

- [x] 10.4 Write property test for skin persistence round-trip


  - **Property 11: Skin persistence round-trip**
  - **Validates: Requirements 4.5**

- [x] 11. Implement audio visualizer with Web Audio API





  - Create Visualizer.svelte component with Canvas element
  - Set up Web Audio API context and analyzer node
  - Implement FFT-based frequency analysis
  - Create rendering functions for bars, waveform, and spectrum styles
  - Implement requestAnimationFrame rendering loop (target 30+ FPS)
  - Add visualization style switching without playback interruption
  - Connect visualizer to audio source (local playback)
  - _Requirements: 5.1, 5.2, 5.3, 5.5_

- [x] 11.1 Write property test for visualizer activation


  - **Property 12: Visualizer activation**
  - **Validates: Requirements 5.1, 5.4**

- [x] 11.2 Write property test for visualizer frame rate


  - **Property 13: Visualizer frame rate**
  - **Validates: Requirements 5.3**

- [x] 11.3 Write property test for visualizer style switching


  - **Property 14: Visualizer style switching**
  - **Validates: Requirements 5.5**

- [x] 12. Create farmer buddy system with state machine





  - Create FarmerBuddy.svelte component with SVG container
  - Implement state machine (idle, listening, prompting, celebrating, error)
  - Load farmer SVG assets (face, eyes, mouth variations)
  - Create animation controller for state transitions
  - Implement idle animations (blinking, looking around)
  - Add speech bubble component for prompts and messages
  - Create Svelte store for farmer state management
  - _Requirements: 6.1, 6.2, 6.3, 6.4, 6.5_

- [x] 12.1 Write property test for farmer error state handling


  - **Property 15: farmer error state handling**
  - **Validates: Requirements 6.3**



- [x] 12.2 Write property test for farmer state machine transitions





  - **Property 16: farmer state machine transitions**
  - **Validates: Requirements 6.4, 7.1, 7.2, 7.3**

- [x] 13. Implement farmer playback event reactions





  - Connect farmer to player state changes (track start, stop, pause)
  - Implement listening state animations synchronized with audio
  - Add track change reaction animations
  - Create visualizer-reactive movements for farmer
  - _Requirements: 7.1, 7.2, 7.3, 7.4_

- [x] 13.1 Write property test for farmer listening state animations


  - **Property 17: farmer listening state animations**
  - **Validates: Requirements 7.4**

- [x] 14. Build Spotify API integration





  - Create SpotifyBridge struct implementing StreamingService trait
  - Implement OAuth 2.0 authentication flow
  - Add token storage and refresh logic
  - Implement "Now Playing" metadata retrieval (/v1/me/player/currently-playing)
  - Create polling mechanism for track changes (2-second update target)
  - Add Tauri IPC commands for Spotify operations (auth, get_now_playing)
  - Handle authentication failures with farmer error prompts
  - _Requirements: 2.1, 2.2, 2.3, 2.4, 2.5_

- [x] 14.1 Write property test for streaming metadata completeness


  - **Property 6: Streaming metadata completeness**
  - **Validates: Requirements 2.2, 3.2**

- [x] 14.2 Write property test for streaming metadata sync timing


  - **Property 7: Streaming metadata sync timing**
  - **Validates: Requirements 2.3, 3.3**

- [x] 15. Build YouTube API integration





  - Create YouTubeBridge struct implementing StreamingService trait
  - Implement API key and OAuth authentication
  - Add credential validation and storage
  - Implement video metadata retrieval (YouTube Data API v3)
  - Create polling mechanism for playback state changes (2-second update target)
  - Add Tauri IPC commands for YouTube operations (auth, get_now_playing)
  - Handle authentication failures with farmer error prompts
  - _Requirements: 3.1, 3.2, 3.3, 3.4_

- [x] 15.1 Write property test for API bridge interface compliance


  - **Property 24: API bridge interface compliance**
  - **Validates: Requirements 11.4**

- [ ] 16. Integrate streaming service metadata into UI
  - Update Player component to display streaming service metadata
  - Add source indicator (local, Spotify, YouTube)
  - Implement metadata update handlers for streaming services
  - Connect visualizer to system audio capture for streaming playback
  - _Requirements: 2.2, 2.3, 2.4, 3.2, 3.3, 5.4_

- [ ] 17. Implement first-run setup flow with farmer
  - Detect first launch (no configuration file)
  - Trigger farmer prompting state for library path
  - Create setup wizard UI with farmer guidance
  - Implement library path validation and selection
  - Add optional Spotify/YouTube credential setup
  - Transition farmer to celebrating state on completion
  - Save initial configuration
  - _Requirements: 6.1, 6.2, 6.3, 6.4_

- [ ] 18. Optimize performance and resource usage
  - Implement lazy loading for playlist tracks
  - Add bounded metadata cache with LRU eviction
  - Optimize visualizer rendering (throttle when not focused)
  - Defer non-critical initialization (streaming services)
  - Implement async I/O for all file operations
  - Add memory profiling and optimization
  - Measure and optimize startup time (target <2s)
  - _Requirements: 8.1, 8.2, 8.3, 8.4_

- [ ] 19. Add comprehensive error handling and logging
  - Implement error types for all error categories
  - Add graceful degradation for non-critical failures
  - Connect all error paths to farmer error state
  - Implement application logging (ERROR, WARN, INFO levels)
  - Add log file rotation and size limits
  - Create user-friendly error messages for farmer
  - _Requirements: 6.3, 10.3_

- [ ] 20. Build and package application
  - Configure Tauri build settings for Windows
  - Create application icon and assets
  - Build release executable (verify <15MB size)
  - Generate MSI installer with proper metadata
  - Create portable ZIP distribution
  - Test installation and uninstallation
  - Verify file associations (.wsz, .wal)
  - _Requirements: 8.1_

- [ ] 21. Final checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.
