# milk

A desktop audio visual media buddy inspired by Winamp.

## Description

`milk` is a lightweight (~10MB) desktop application that recreates the nostalgic experience of 2000s-era media players. Built with Tauri 2.x (Rust backend, Svelte frontend), it combines local audio playback, streaming service metadata integration, Winamp skin compatibility, and an animated companion ("farmer") that assists with configuration.

## Tech Stack

- **Frontend**: Svelte 5 + TypeScript + Vite
- **Backend**: Rust + Tauri 2.x
- **Package Manager**: pnpm
- **Build Target**: Windows (x86_64-pc-windows-msvc)

## Development

### Prerequisites

- Node.js (with npm)
- pnpm (`npm install -g pnpm`)
- Rust toolchain with `x86_64-pc-windows-msvc` target

### Setup

```bash
# Install dependencies
pnpm install

# Run development server
pnpm tauri:dev

# Build for production
pnpm tauri:build
```

For detailed build and packaging instructions, see [BUILDING.md](BUILDING.md) and [BUILD.md](BUILD.md).

## Project Structure

```
milk/
├── src/              # Svelte frontend source
├── src-tauri/        # Rust backend source
├── static/           # Static assets
├── assets/           # Application assets (skins, farmer graphics)
└── docs/             # Documentation
```

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
