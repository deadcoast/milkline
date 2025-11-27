# Requirements Document

## Introduction

`milk` is a desktop audio visual media buddy that recreates the nostalgic experience of 2000s-era media players, specifically inspired by Winamp. Built with Tauri (Rust backend, Svelte frontend), `milk` provides a lightweight (~10MB), native Windows application that combines local audio playback, streaming service metadata integration, Winamp skin compatibility, and a simple animated companion ("farmer") that assists with configuration and setup tasks.

The application prioritizes three core pillars: quality of life (streamlined functionality without bloat), visual aesthetics (authentic 2000s design with Winamp skin support), and user experience (nostalgic feel with modern reliability).

## Glossary

- **milk**: The desktop audio visual media buddy application
- **farmer**: The animated companion/buddy system within milk (not AI, script-driven state machine)
- **Tauri Application**: The compiled desktop application using Tauri runtime with Rust backend and Svelte frontend
- **Winamp Skin**: A `.wsz` or `.wal` file containing visual assets (BMPs, PNGs) and configuration for UI theming
- **Visualizer**: Real-time audio visualization component using Web Audio API and Canvas
- **Now Playing Sync**: Metadata synchronization from Spotify/YouTube showing current track information
- **Local Library**: User's collection of local audio files (mp3, flac, wav) stored on disk
- **IPC**: Inter-Process Communication between Svelte frontend and Rust backend via Tauri

## Requirements

### Requirement 1

**User Story:** As a user, I want to play local audio files from my computer, so that I can listen to my music collection through the milk interface.

#### Acceptance Criteria

1. WHEN a user selects a local audio file (mp3, flac, or wav format) THEN the Tauri Application SHALL load and play the audio file
2. WHEN a user adds a directory path to the library THEN the Tauri Application SHALL scan and index all supported audio files in that directory
3. WHEN playback is active THEN the Tauri Application SHALL provide accurate playback position and duration information
4. WHEN a user adjusts volume controls THEN the Tauri Application SHALL modify audio output volume in real-time
5. WHEN a user triggers play, pause, stop, next, or previous controls THEN the Tauri Application SHALL execute the corresponding playback command immediately

### Requirement 2

**User Story:** As a user, I want to see my currently playing track from Spotify displayed in milk, so that I can have a unified visual experience regardless of playback source.

#### Acceptance Criteria

1. WHEN a user authenticates with Spotify OAuth 2.0 THEN the Tauri Application SHALL store credentials securely and establish API connection
2. WHEN Spotify is playing a track THEN the Tauri Application SHALL retrieve and display current track metadata (title, artist, album, duration)
3. WHEN Spotify track changes THEN the Tauri Application SHALL update the displayed metadata within 2 seconds
4. WHEN Spotify playback is paused or stopped THEN the Tauri Application SHALL reflect the updated playback state in the UI
5. WHEN Spotify authentication expires or fails THEN the Tauri Application SHALL prompt the user to re-authenticate via farmer

### Requirement 3

**User Story:** As a user, I want to see my currently playing track from YouTube displayed in milk, so that I can monitor my YouTube music playback through the milk interface.

#### Acceptance Criteria

1. WHEN a user provides YouTube API credentials THEN the Tauri Application SHALL validate and store the credentials securely
2. WHEN YouTube is playing a video THEN the Tauri Application SHALL retrieve and display video metadata (title, channel, duration)
3. WHEN YouTube playback state changes THEN the Tauri Application SHALL update the displayed information within 2 seconds
4. WHEN YouTube authentication fails THEN the Tauri Application SHALL display an error state via farmer and prompt for credential re-entry

### Requirement 4

**User Story:** As a user, I want to apply Winamp skins to milk, so that I can customize the visual appearance with classic or community-created designs.

#### Acceptance Criteria

1. WHEN a user selects a `.wsz` skin file THEN the Tauri Application SHALL extract the archive and parse the skin assets
2. WHEN skin assets are parsed THEN the Tauri Application SHALL map BMP/PNG assets to corresponding UI regions (play button, pause button, position bar, etc.)
3. WHEN a skin is applied THEN the Tauri Application SHALL render all UI components using the skin's visual assets
4. WHEN a skin file is corrupted or invalid THEN the Tauri Application SHALL fall back to the default skin and notify the user via farmer
5. WHEN the application starts THEN the Tauri Application SHALL load the last successfully applied skin from configuration

### Requirement 5

**User Story:** As a user, I want to see real-time audio visualization synchronized with playback, so that I can enjoy a visual representation of the music.

#### Acceptance Criteria

1. WHEN audio is playing THEN the Visualizer SHALL display real-time frequency or waveform visualization
2. WHEN audio playback stops THEN the Visualizer SHALL return to an idle or static state
3. WHEN the Visualizer renders frames THEN the Tauri Application SHALL maintain at least 30 frames per second
4. WHEN system audio is captured THEN the Visualizer SHALL synchronize with the audio output regardless of playback source (local, Spotify, YouTube)
5. WHEN the user changes visualization style THEN the Visualizer SHALL switch rendering modes without interrupting playback

### Requirement 6

**User Story:** As a user, I want farmer to guide me through initial setup and configuration, so that I can quickly get milk running without confusion.

#### Acceptance Criteria

1. WHEN the application launches for the first time THEN farmer SHALL prompt the user for the local music library path
2. WHEN farmer prompts for input THEN farmer SHALL display the appropriate facial expression (prompting state) and speech bubble
3. WHEN a user provides invalid input (non-existent path, invalid credentials) THEN farmer SHALL transition to error state and display helpful error message
4. WHEN configuration is successfully completed THEN farmer SHALL transition to celebrating state briefly, then return to idle
5. WHEN farmer is in idle state THEN farmer SHALL display subtle animations (blinking, looking around) without user interaction

### Requirement 7

**User Story:** As a user, I want farmer to react to playback events, so that the interface feels alive and responsive.

#### Acceptance Criteria

1. WHEN a track starts playing THEN farmer SHALL transition to listening state with synchronized animations
2. WHEN playback is paused or stopped THEN farmer SHALL return to idle state
3. WHEN a track changes THEN farmer SHALL briefly react (visual acknowledgment) before returning to listening state
4. WHEN farmer is in listening state THEN farmer SHALL display animations synchronized with audio (bobbing, visualizer-reactive movements)

### Requirement 8

**User Story:** As a user, I want milk to have a small memory footprint and fast startup time, so that it feels like native software from the 2000s era, not a bloated modern application.

#### Acceptance Criteria

1. WHEN the application is compiled THEN the Tauri Application SHALL produce an executable under 15MB in size
2. WHEN the application is running with idle playback THEN the Tauri Application SHALL consume less than 100MB of RAM
3. WHEN the application launches THEN the Tauri Application SHALL display the main window within 2 seconds on modern hardware
4. WHEN the application is minimized THEN the Tauri Application SHALL reduce resource consumption to minimal levels

### Requirement 9

**User Story:** As a user, I want to manage playlists and queue tracks, so that I can organize my listening experience.

#### Acceptance Criteria

1. WHEN a user creates a playlist THEN the Tauri Application SHALL store the playlist with track references persistently
2. WHEN a user adds tracks to a playlist THEN the Tauri Application SHALL update the playlist immediately and save to disk
3. WHEN a user reorders tracks in a playlist THEN the Tauri Application SHALL reflect the new order in playback queue
4. WHEN a playlist is loaded THEN the Tauri Application SHALL populate the playback queue with all tracks in order
5. WHEN a user removes a track from a playlist THEN the Tauri Application SHALL update the playlist without affecting the original audio file

### Requirement 10

**User Story:** As a user, I want milk to persist my configuration and preferences, so that my settings are retained between sessions.

#### Acceptance Criteria

1. WHEN a user modifies settings (volume, skin, library path, credentials) THEN the Tauri Application SHALL save configuration to disk immediately
2. WHEN the application starts THEN the Tauri Application SHALL load all saved configuration and apply settings
3. WHEN configuration file is corrupted or missing THEN the Tauri Application SHALL create default configuration and notify user via farmer
4. WHEN credentials are stored THEN the Tauri Application SHALL encrypt sensitive data (OAuth tokens, API keys) using platform-native secure storage

### Requirement 11

**User Story:** As a developer, I want the codebase to maintain clear separation between frontend UI, backend logic, and external API integrations, so that the system is maintainable and extensible.

#### Acceptance Criteria

1. WHEN frontend components need backend data THEN the Tauri Application SHALL communicate exclusively via IPC commands
2. WHEN external API calls are made THEN the Tauri Application SHALL handle all API communication in the Rust backend layer
3. WHEN UI state changes THEN the Svelte frontend SHALL manage state through Svelte stores without direct backend coupling
4. WHEN new streaming services are added THEN the Tauri Application SHALL support integration through a common API bridge interface

### Requirement 12

**User Story:** As a user, I want to extract and view metadata from my local audio files, so that milk can display track information even for files without embedded tags.

#### Acceptance Criteria

1. WHEN a local audio file is loaded THEN the Tauri Application SHALL extract embedded metadata (ID3 tags, FLAC tags, etc.)
2. WHEN metadata is missing or incomplete THEN the Tauri Application SHALL derive information from filename and directory structure
3. WHEN album art is embedded in audio file THEN the Tauri Application SHALL extract and display the artwork
4. WHEN metadata is extracted THEN the Tauri Application SHALL cache the information to avoid repeated parsing
