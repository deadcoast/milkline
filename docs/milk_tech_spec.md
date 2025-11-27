# `milk` Technical Specification

> Companion document to `README.md` — this describes the build, not the soul.

---

## Stack

| Layer | Technology | Rationale |
|-------|------------|-----------|
| Runtime | **Tauri 2.x** | Rust backend, lightweight (~10MB vs Electron's 150MB+), native Windows feel |
| Frontend | **Svelte** | Minimal boilerplate, reactive, compiles to vanilla JS — keeps bundle small |
| Styling | **CSS3 + Winamp Skin Parser** | Native CSS for base, custom parser for `.wsz` skin compatibility |
| Audio Viz | **Web Audio API + Canvas** | Hardware-accelerated, no dependencies |
| State | **Svelte Stores + Tauri IPC** | Frontend state in Svelte, persistent config via Rust fs |
| farmer | **Lottie / SVG + CSS Animations** | Vector-based, frame-controlled expressions |

---

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                      milk (Tauri)                       │
├─────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────┐    │
│  │               Svelte Frontend                   │    │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────────┐   │    │
│  │  │ Player   │  │ Playlist │  │ Visualizer   │   │    │
│  │  │ Controls │  │ View     │  │ (Canvas)     │   │    │
│  │  └──────────┘  └──────────┘  └──────────────┘   │    │
│  │  ┌──────────────────────────────────────────┐   │    │
│  │  │              farmer (buddy)              │   │    │
│  │  │         SVG + state machine              │   │    │
│  │  └──────────────────────────────────────────┘   │    │
│  └─────────────────────────────────────────────────┘    │
│                         │ IPC                           │
│  ┌─────────────────────────────────────────────────┐    │
│  │                Rust Backend                     │    │
│  │  ┌────────────┐  ┌────────────┐  ┌───────────┐  │    │
│  │  │ Config     │  │ API Bridge │  │ Local     │  │    │
│  │  │ Manager    │  │ (YT/Spot)  │  │ Library   │  │    │
│  │  └────────────┘  └────────────┘  └───────────┘  │    │
│  └─────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────┘
```

---

## API Integration

### Spotify

| Feature | Endpoint | Auth |
|---------|----------|------|
| Playback Control | Web Playback SDK | OAuth 2.0 (Premium required) |
| Now Playing | `/v1/me/player/currently-playing` | OAuth 2.0 |
| Library Sync | `/v1/me/tracks` | OAuth 2.0 |

**Limitation**: Web Playback SDK requires Spotify Premium. Free tier = metadata only, no playback control.

### YouTube

| Feature | Endpoint | Auth |
|---------|----------|------|
| Playlist Data | YouTube Data API v3 | API Key + OAuth |
| Playback | IFrame API (embedded) | None |
| Watch History | `/activities` | OAuth 2.0 |

**Limitation**: YouTube ToS prohibits audio-only extraction. Playback must use embedded player (can be hidden/styled over).

---

## farmer (Buddy System)

Not AI. Script-driven state machine.

```
States: idle | listening | prompting | celebrating | error

Triggers:
  - app_launch      → celebrating (2s) → idle
  - user_inactive   → idle animations (blink, look around)
  - prompt_needed   → prompting (speech bubble + expression)
  - track_change    → listening (bobbing, visualizer sync)
  - error_state     → error (concerned expression + helper text)
```

Asset structure:
```
/assets/farmer/
├── vectors/
│   ├── face-base.svg
│   ├── eyes/
│   │   ├── neutral.svg
│   │   ├── blink.svg
│   │   ├── look-left.svg
│   │   └── look-right.svg
│   └── mouth/
│       ├── neutral.svg
│       ├── smile.svg
│       ├── talk-1.svg
│       └── talk-2.svg
└── animations/
    └── farmer.lottie (optional compiled)
```

---

## Build

### Requirements

- Rust 1.70+
- Node 18+
- pnpm (preferred) or npm

### Commands

```bash
# dev
pnpm tauri dev

# build (Windows)
pnpm tauri build --target x86_64-pc-windows-msvc

# output
./src-tauri/target/release/milk.exe
./src-tauri/target/release/bundle/msi/milk_x.x.x_x64.msi
```

---

## Directory Structure

```
milk/
├── src/                    # Svelte frontend
│   ├── lib/
│   │   ├── components/
│   │   │   ├── Player.svelte
│   │   │   ├── Playlist.svelte
│   │   │   ├── Visualizer.svelte
│   │   │   └── Farmer.svelte
│   │   ├── stores/
│   │   └── api/
│   │       ├── spotify.ts
│   │       └── youtube.ts
│   ├── App.svelte
│   └── main.ts
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── main.rs
│   │   ├── config.rs
│   │   ├── library.rs
│   │   └── api_bridge.rs
│   ├── Cargo.toml
│   └── tauri.conf.json
├── assets/
│   ├── farmer/
│   ├── skins/
│   └── sounds/             # UI sounds (ICQ nostalgia)
├── README.md               # The soul
├── TECH.md                 # This document
└── package.json
```

---

## Skin System

Target: Winamp `.wsz` compatibility (stretch goal)

Phase 1: Custom JSON skin format
```json
{
  "name": "classic",
  "colors": {
    "bg": "#1e1e1e",
    "accent": "#00ff00",
    "text": "#ffffff"
  },
  "assets": {
    "play": "skins/classic/play.png",
    "pause": "skins/classic/pause.png"
  }
}
```

Phase 2: `.wsz` parser (Winamp skin archive = renamed .zip with BMPs)

---

## MVP Scope

1. Local audio playback (mp3, flac, wav)
2. Basic Winamp-style UI (single skin)
3. farmer buddy (idle + prompting states)
4. Audio visualizer (bars or waveform)
5. Spotify "Now Playing" sync (metadata display)

---

> milk line audio


```

```