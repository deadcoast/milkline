# milk Player - Frontend Library

This directory contains the core frontend logic for the milk player application.

## Structure

- **`types.ts`** - TypeScript type definitions for the application
- **`stores/`** - Svelte stores for state management
  - `playerStore.ts` - Player state (current track, playback status, queue)
  - `playlistStore.ts` - Playlist management state
  - `configStore.ts` - Application configuration state
- **`tauri/`** - Tauri IPC client wrapper functions
  - `ipc.ts` - All Tauri backend command wrappers
- **`styles/`** - Global CSS and theming
  - `variables.css` - CSS custom properties for theming
  - `global.css` - Global styles and utilities

## Usage

### Stores

Import stores from the stores directory:

```typescript
import { playerStore, playlistStore, configStore } from "$lib/stores";

// Subscribe to store changes
playerStore.subscribe((state) => {
  console.log("Player state:", state);
});

// Update store
playerStore.setPlaying(true);
```

### Tauri IPC

Import IPC functions from the tauri directory:

```typescript
import { loadConfig, scanLibrary } from "$lib/tauri";

// Call backend commands
const config = await loadConfig();
const tracks = await scanLibrary("/path/to/music");
```

### Types

Import types for TypeScript support:

```typescript
import type { Track, Playlist, AppConfig } from "$lib/types";

const track: Track = {
  id: "1",
  title: "Song Title",
  artist: "Artist Name",
  album: "Album Name",
  duration: 180,
  source: "local",
  metadata: {},
};
```

## State Management

The application uses Svelte stores for reactive state management:

- **Player Store**: Manages playback state, current track, queue, and volume
- **Playlist Store**: Manages playlists and their tracks
- **Config Store**: Manages application configuration and preferences

All stores provide methods for updating state and automatically notify subscribers of changes.
