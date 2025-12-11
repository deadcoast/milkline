# milk

A desktop audio visual media buddy inspired by Winamp.

## Description

`milk` is a lightweight (~4MB) desktop application that recreates the nostalgic experience of 2000s-era media players. Built with Tauri 2.x (Rust backend, Svelte frontend), it combines local audio playback, streaming service metadata integration, Winamp skin compatibility, and an animated companion ("farmer") that assists with configuration.

**Cross-Platform**: Runs on macOS (Apple Silicon + Intel) and Windows (x86_64).

## Features

- **Local Audio Playback** - Play mp3, flac, and wav files
- **Media Editor** - Crop images and videos, trim video clips
- **Winamp Skin Support** - Load classic .wsz and .wal skins
- **Real-time Visualizer** - Audio visualization with multiple styles
- **Animated Companion** - "farmer" buddy guides you through setup
- **Streaming Integration** - Display metadata from Spotify and YouTube
- **Playlist Management** - Create and manage playlists
- **Lightweight** - <15MB executable, <100MB RAM usage
- **Fast Startup** - <2 second launch time

## Media Editor

The built-in media editor provides simple, intuitive tools for basic image and video editing:

### Capabilities

- **Image Cropping** - Drag to select and crop images
- **Video Cropping** - Remove black bars or unwanted portions from video frames
- **Video Trimming** - Extract specific segments using timeline controls
- **Combined Operations** - Crop and trim videos simultaneously

### Supported Formats

**Images**: PNG, JPG, JPEG, BMP, GIF  
**Videos**: MP4, MOV, MKV

### Requirements

The media editor requires **FFmpeg** to be installed and available in your system PATH for video operations. Image operations use the built-in Rust `image` crate.

See the **[Media Editor User Guide](docs/MEDIA_EDITOR_GUIDE.md)** for detailed usage instructions.

## Tech Stack

- **Frontend**: Svelte 5 + TypeScript + Vite
- **Backend**: Rust + Tauri 2.x
- **Media Processing**: FFmpeg (video), Rust `image` crate (images)
- **Package Manager**: pnpm
- **Platforms**: macOS (aarch64/x86_64) + Windows (x86_64-pc-windows-msvc)

## Quick Start

### Prerequisites

- Node.js 18+ (with npm)
- pnpm (`npm install -g pnpm`)
- Rust toolchain (automatically installs required targets)
- **FFmpeg** (required for media editor video features)
  - macOS: `brew install ffmpeg`
  - Windows: Download from [ffmpeg.org](https://ffmpeg.org) and add to PATH
  - Linux: `apt-get install ffmpeg` or equivalent

### Development

```bash
# Install dependencies
pnpm install

# Run development server
pnpm tauri:dev

# Run tests
pnpm test

# Build for production
pnpm tauri:build
```

## Documentation

📚 **[Complete Documentation](docs/README.md)** - Full documentation map

### Quick Links

- **[Media Editor User Guide](docs/MEDIA_EDITOR_GUIDE.md)** - How to use the media editor
- **[Cross-Platform Build Guide](docs/CROSS_PLATFORM_BUILD.md)** - Build for macOS & Windows
- **[Windows Build Guide](docs/BUILD_WINDOWS.md)** - Windows-specific build instructions
- **[Release Process](docs/RELEASE_PROCESS.md)** - Complete release workflow
- **[Quick Build Guide](docs/BUILDING.md)** - Get started building milk
- **[Build & Package Guide](docs/BUILD.md)** - Comprehensive build documentation
- **[Technical Specification](docs/milk_tech_spec.md)** - Architecture and design
- **[Release Checklist](docs/RELEASE_CHECKLIST.md)** - Release process
- **[Installation Testing](docs/INSTALLATION_TESTING.md)** - Testing guide
- **[Error Handling](docs/ERROR_HANDLING.md)** - Error handling patterns
- **[Performance Optimizations](docs/PERFORMANCE_OPTIMIZATIONS.md)** - Performance guide

## Project Structure

```
milk/
├── src/                    # Svelte frontend source
│   ├── lib/               # Shared components and utilities
│   │   ├── components/    # Svelte components
│   │   ├── stores/        # State management
│   │   └── utils/         # Utility functions
│   └── routes/            # SvelteKit routes
├── src-tauri/             # Rust backend source
│   ├── src/               # Rust source code
│   ├── icons/             # Application icons
│   └── wix/               # WiX installer configuration
├── static/                # Static assets
├── assets/                # Application assets (skins, graphics)
├── docs/                  # Documentation
├── scripts/               # Build and utility scripts
└── .kiro/specs/           # Feature specifications
```

## Building & Distribution

### Build Artifacts

#### macOS

- **Executable**: `src-tauri/target/release/milk` (3.7MB)
- **App Bundle**: `src-tauri/target/release/bundle/macos/milk.app`
- **DMG Installer**: macOS disk image installer

#### Windows

- **Executable**: `src-tauri/target/x86_64-pc-windows-msvc/release/milk.exe` (<15MB target)
- **MSI Installer**: Windows installer with file associations (.wsz, .wal)
- **NSIS Installer**: Alternative Windows installer
- **Portable ZIP**: No-installation portable distribution

See **[Windows Build Guide](docs/BUILD_WINDOWS.md)** for detailed Windows build instructions.

### Build Commands

```bash
# Development server
pnpm tauri:dev

# Build for current platform
pnpm tauri:build

# Platform-specific builds
pnpm tauri:build:macos      # macOS (Apple Silicon)
pnpm tauri:build:windows    # Windows (requires GitHub Actions)
pnpm tauri:build:all        # All platforms

# Cross-platform build script
./scripts/build-cross-platform.sh
```

**For Windows builds**: Use GitHub Actions (`.github/workflows/build.yml`) which builds on native Windows runners. Cross-compilation from macOS to Windows is not supported due to native dependencies.

See **[Cross-Platform Build Guide](docs/CROSS_PLATFORM_BUILD.md)** for detailed instructions.

## Testing

```bash
# Run all tests
pnpm test

# Run tests in watch mode
pnpm test:watch

# Run tests with UI
pnpm test:ui
```

See [docs/INSTALLATION_TESTING.md](docs/INSTALLATION_TESTING.md) for installation testing procedures.

## Performance Targets

- **Executable Size**: <15MB ✅ **Achieved: 3.7MB (macOS)**
- **RAM Usage (idle)**: <100MB ✅
- **Startup Time**: <2 seconds ✅
- **Visualizer FPS**: 30+ ✅
- **Build Time**: ~2-3 minutes per platform

See [docs/PERFORMANCE_OPTIMIZATIONS.md](docs/PERFORMANCE_OPTIMIZATIONS.md) for optimization strategies.

## Development Workflow

1. **Feature Development**: See `.kiro/specs/milk-player/` for specifications
2. **Error Handling**: Follow patterns in [docs/ERROR_HANDLING.md](docs/ERROR_HANDLING.md)
3. **Testing**: Write unit and property-based tests
4. **Building**:
   - macOS: Build locally with `pnpm tauri:build`
   - Windows: Use GitHub Actions workflow
   - See [Cross-Platform Build Guide](docs/CROSS_PLATFORM_BUILD.md)
5. **Release**: Follow [docs/RELEASE_CHECKLIST.md](docs/RELEASE_CHECKLIST.md)

### Continuous Integration

GitHub Actions automatically builds for all platforms on:

- Push to `main` branch
- Pull requests
- Manual workflow dispatch
- Tagged releases (creates GitHub Release with installers)

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Contributing

1. Check the [specifications](.kiro/specs/milk-player/) for planned features
2. Follow the [error handling guidelines](docs/ERROR_HANDLING.md)
3. Write tests for new functionality
4. Update documentation as needed

## License

MIT License - See LICENSE file for details

## Acknowledgments

Inspired by Winamp and the nostalgic era of 2000s media players.
