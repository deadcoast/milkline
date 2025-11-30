# Cross-Platform Building Guide

This document explains how to build milk for both macOS and Windows.

## Quick Start

### Local Development (macOS only)
```bash
# Build for your current platform
pnpm tauri:build

# Or specifically for macOS
pnpm tauri:build:macos
```

### Cross-Platform Builds (GitHub Actions)
For Windows builds, use GitHub Actions which builds on native Windows runners:

1. Push your code to GitHub
2. The workflow automatically builds for macOS (both Intel and Apple Silicon) and Windows
3. Download artifacts from the Actions tab

## Build Commands

| Command | Description |
|---------|-------------|
| `pnpm tauri:build` | Build for current platform (auto-detects) |
| `pnpm tauri:build:macos` | Build for macOS (aarch64) |
| `pnpm tauri:build:windows` | Build for Windows (requires Windows or CI) |
| `pnpm tauri:build:all` | Build for all platforms (requires setup) |

## Build Artifacts

After building, find your artifacts here:

### macOS
- **Executable**: `src-tauri/target/aarch64-apple-darwin/release/milk`
- **App Bundle**: `src-tauri/target/aarch64-apple-darwin/release/bundle/macos/milk.app`
- **DMG Installer**: `src-tauri/target/aarch64-apple-darwin/release/bundle/dmg/milk_*.dmg`

### Windows (via GitHub Actions)
- **Executable**: `src-tauri/target/x86_64-pc-windows-msvc/release/milk.exe`
- **MSI Installer**: `src-tauri/target/x86_64-pc-windows-msvc/release/bundle/msi/milk_*.msi`
- **NSIS Installer**: `src-tauri/target/x86_64-pc-windows-msvc/release/bundle/nsis/milk_*-setup.exe`

## Why Use GitHub Actions for Windows?

Cross-compiling Tauri apps from macOS to Windows is **not recommended** because:

1. **Native Dependencies**: Libraries like `ring` (cryptography) require platform-specific SDKs
2. **WebView2**: Windows-specific component needs Windows build environment
3. **Installer Creation**: MSI/NSIS installers require Windows tools

**GitHub Actions solves this** by building on actual Windows machines, ensuring 100% compatibility.

## GitHub Actions Workflow

The workflow (`.github/workflows/build.yml`) automatically:

1. **Builds on push/PR** to main branch
2. **Creates artifacts** for all platforms:
   - macOS Apple Silicon (aarch64)
   - macOS Intel (x86_64)
   - Windows (x86_64)
3. **Creates releases** when you push a git tag (e.g., `v0.1.0`)

### Manual Trigger

You can manually trigger a build:
1. Go to **Actions** tab on GitHub
2. Select **Build milk** workflow
3. Click **Run workflow**

## Configuration Files

### `.cargo/config.toml`
Cross-compilation settings for Cargo:
```toml
[target.x86_64-pc-windows-msvc]
linker = "cargo-xwin"
runner = "wine64"

[target.aarch64-apple-darwin]
rustflags = ["-C", "link-arg=-Wl,-rpath,@loader_path"]
```

### `tauri.conf.json`
Bundle configuration:
```json
{
  "bundle": {
    "targets": ["msi", "nsis"],  // Windows installers
    ...
  }
}
```

## Development Workflow

### For macOS Development
```bash
# Run in development mode
pnpm tauri:dev

# Build release version
pnpm tauri:build

# Test the built app
open src-tauri/target/release/bundle/macos/milk.app
```

### For Windows Testing
1. Build via GitHub Actions
2. Download the artifact
3. Test on a Windows machine or VM

### For Both Platforms
```bash
# Run all tests
pnpm test

# Type check
pnpm check

# Build for release
git push origin main
# Then download artifacts from GitHub Actions
```

## Creating a Release

To create a new release with installers:

```bash
# 1. Update version in package.json and src-tauri/Cargo.toml
# 2. Commit changes
git add .
git commit -m "chore: bump version to v0.1.0"

# 3. Create and push tag
git tag v0.1.0
git push origin v0.1.0

# 4. GitHub Actions will automatically:
#    - Build for all platforms
#    - Create GitHub Release
#    - Attach installers to release
```

## Troubleshooting

### "Failed to build for Windows" locally
**Expected behavior**. Build for Windows using GitHub Actions instead.

### GitHub Actions fails
- Check the Actions logs for specific errors
- Ensure all dependencies are listed in `package.json` and `Cargo.toml`
- Verify secrets are set (if using code signing)

### Build size too large
Current target: <15MB

Check sizes:
```bash
# macOS
ls -lh src-tauri/target/release/milk

# Windows (after download from Actions)
ls -lh milk.exe
```

Optimize in `Cargo.toml`:
```toml
[profile.release]
opt-level = "z"     # Optimize for size
lto = true          # Link-time optimization
strip = true        # Remove debug symbols
```

## Tools Installed

- **cargo-xwin**: Cross-compilation support (for future use)
- **rustup targets**: Windows MSVC target added
- **pnpm scripts**: Platform-specific build commands

## Next Steps

1. **Push to GitHub**: Enable Actions workflow
2. **Set up code signing** (optional but recommended):
   - macOS: Apple Developer certificate
   - Windows: Code signing certificate
3. **Configure auto-updates**: Use Tauri's updater feature
4. **Add CI tests**: Run tests in GitHub Actions before building

## Performance Targets

- ✅ **Executable size**: <15MB (currently 3.7MB macOS, TBD Windows)
- ✅ **RAM usage**: <100MB idle
- ✅ **Startup time**: <2 seconds
- ✅ **Build time**: ~2-3 minutes per platform

## Resources

- [Tauri Building Guide](https://v2.tauri.app/develop/building/)
- [GitHub Actions for Tauri](https://tauri.app/v1/guides/building/cross-platform)
- [cargo-xwin Documentation](https://github.com/rust-cross/cargo-xwin)
