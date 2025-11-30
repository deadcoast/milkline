# milk

A desktop audio visual media buddy inspired by Winamp.

## Description

`milk` is a lightweight (~10MB) desktop application that recreates the nostalgic experience of 2000s-era media players. Built with Tauri 2.x (Rust backend, Svelte frontend), it combines local audio playback, streaming service metadata integration, Winamp skin compatibility, and an animated companion ("farmer") that assists with configuration.

## Features

- 🎵 **Local Audio Playback** - Play mp3, flac, and wav files
- 🎨 **Winamp Skin Support** - Load classic .wsz and .wal skins
- 📊 **Real-time Visualizer** - Audio visualization with multiple styles
- 🎭 **Animated Companion** - "farmer" buddy guides you through setup
- 🔗 **Streaming Integration** - Display metadata from Spotify and YouTube
- 📝 **Playlist Management** - Create and manage playlists
- ⚡ **Lightweight** - <15MB executable, <100MB RAM usage
- 🚀 **Fast Startup** - <2 second launch time

## Tech Stack

- **Frontend**: Svelte 5 + TypeScript + Vite
- **Backend**: Rust + Tauri 2.x
- **Package Manager**: pnpm
- **Build Target**: Windows (x86_64-pc-windows-msvc)

## Quick Start

### Prerequisites

- Node.js 18+ (with npm)
- pnpm (`npm install -g pnpm`)
- Rust toolchain with `x86_64-pc-windows-msvc` target

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

- **Executable**: `src-tauri/target/release/milk.exe` (<15MB)
- **MSI Installer**: Windows installer with file associations
- **NSIS Installer**: Alternative Windows installer
- **Portable ZIP**: No-install version

### Build Commands

```bash
# Development build
pnpm tauri:dev

# Production build
pnpm tauri:build

# Verify build
.\scripts\verify-build.ps1

# Create portable distribution
.\scripts\create-portable.ps1
```

See [docs/BUILD.md](docs/BUILD.md) for detailed build instructions.

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

- **Executable Size**: <15MB
- **RAM Usage (idle)**: <100MB
- **Startup Time**: <2 seconds
- **Visualizer FPS**: 30+

See [docs/PERFORMANCE_OPTIMIZATIONS.md](docs/PERFORMANCE_OPTIMIZATIONS.md) for optimization strategies.

## Development Workflow

1. **Feature Development**: See `.kiro/specs/milk-player/` for specifications
2. **Error Handling**: Follow patterns in [docs/ERROR_HANDLING.md](docs/ERROR_HANDLING.md)
3. **Testing**: Write unit and property-based tests
4. **Building**: Use build scripts in `scripts/`
5. **Release**: Follow [docs/RELEASE_CHECKLIST.md](docs/RELEASE_CHECKLIST.md)

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
