# Build and Packaging Guide

This document describes how to build and package the milk application for distribution.

> **Documentation Map**: See [docs/README.md](README.md) for all documentation  
> **Quick Start**: See [BUILDING.md](BUILDING.md) for a quick reference guide

## Prerequisites

- Node.js 18+ and pnpm
- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- Windows 10/11 (for Windows builds)
- WiX Toolset 3.11+ (for MSI installer generation)

## Build Configuration

The application is configured to produce optimized release builds with the following settings:

- **Optimization Level**: `z` (optimize for size)
- **Link Time Optimization (LTO)**: Enabled
- **Code Generation Units**: 1 (maximum optimization)
- **Panic Strategy**: Abort (reduces binary size)
- **Symbol Stripping**: Enabled (removes debug symbols)

Target binary size: **<15MB** (as per requirement 8.1)

## Building the Application

### Development Build

```bash
pnpm tauri:dev
```

### Release Build

```bash
pnpm tauri:build
```

This will:
1. Build the Svelte frontend (`pnpm build`)
2. Compile the Rust backend in release mode
3. Generate the executable and installers

### Debug Release Build

For testing the release configuration with debug symbols:

```bash
pnpm tauri:build:debug
```

## Build Artifacts

After a successful build, artifacts are located in `src-tauri/target/release/`:

### Executable
- `milk.exe` - Standalone executable (~10-15MB)

### Installers
- `bundle/msi/milk_0.1.0_x64_en-US.msi` - MSI installer for Windows
- `bundle/nsis/milk_0.1.0_x64-setup.exe` - NSIS installer for Windows

## Installer Features

### MSI Installer
- Installs to Program Files
- Creates Start Menu shortcuts
- Registers file associations (.wsz, .wal)
- Includes uninstaller
- Supports silent installation: `msiexec /i milk_0.1.0_x64_en-US.msi /quiet`

### NSIS Installer
- Modern installer UI
- Per-user installation (no admin required)
- File associations for Winamp skins
- Uninstaller included

## File Associations

The application registers handlers for:
- `.wsz` - Winamp Skin (ZIP format)
- `.wal` - Winamp Skin (WAL format)

When users double-click these files, milk will launch and load the skin.

## Portable Distribution

To create a portable ZIP distribution:

1. Build the release executable
2. Copy the following to a distribution folder:
   ```
   milk_portable/
   ├── milk.exe
   ├── README.txt
   └── assets/
       └── (default skin and assets)
   ```
3. Create a ZIP archive: `milk_portable_v0.1.0.zip`

The portable version stores configuration in the same directory as the executable.

## Verification Steps

### 1. Binary Size Check
```bash
# Windows PowerShell
(Get-Item src-tauri/target/release/milk.exe).Length / 1MB
```

Should be **<15MB**.

### 2. Installation Test
1. Run the MSI installer
2. Verify Start Menu shortcut creation
3. Launch the application
4. Verify first-run setup flow

### 3. Uninstallation Test
1. Uninstall via Windows Settings or Control Panel
2. Verify all files are removed
3. Verify Start Menu shortcuts are removed
4. Check that user data (config, playlists) is preserved in AppData

### 4. File Association Test
1. Install the application
2. Download a `.wsz` skin file
3. Double-click the skin file
4. Verify milk launches and loads the skin

### 5. Portable Version Test
1. Extract portable ZIP to a folder
2. Run `milk.exe`
3. Verify application runs without installation
4. Verify configuration is stored locally

## Performance Targets

As per requirement 8.1:
- **Executable size**: <15MB ✓
- **RAM usage (idle)**: <100MB
- **Startup time**: <2 seconds

## Troubleshooting

### Build Fails with "linker error"
- Ensure Visual Studio Build Tools are installed
- Run `rustup target add x86_64-pc-windows-msvc`

### MSI Generation Fails
- Install WiX Toolset 3.11+
- Add WiX to PATH: `C:\Program Files (x86)\WiX Toolset v3.11\bin`

### Binary Size Too Large
- Verify release profile settings in `Cargo.toml`
- Check for unnecessary dependencies
- Use `cargo bloat --release` to analyze binary size

### File Associations Not Working
- Reinstall the application
- Check Windows Default Apps settings
- Verify registry entries (advanced users)

## Distribution Checklist

Before releasing:
- [ ] Build release executable
- [ ] Verify binary size <15MB
- [ ] Test MSI installer
- [ ] Test NSIS installer
- [ ] Create portable ZIP
- [ ] Test file associations (.wsz, .wal)
- [ ] Test installation on clean Windows system
- [ ] Test uninstallation
- [ ] Verify startup time <2s
- [ ] Verify RAM usage <100MB
- [ ] Update version numbers
- [ ] Create release notes
- [ ] Tag release in git
- [ ] Upload artifacts to GitHub Releases

## Release Process

1. Update version in `package.json` and `src-tauri/Cargo.toml`
2. Update `CHANGELOG.md`
3. Commit version bump: `git commit -am "Release v0.1.0"`
4. Tag release: `git tag v0.1.0`
5. Build release: `pnpm tauri:build`
6. Verify all artifacts
7. Push to GitHub: `git push && git push --tags`
8. Create GitHub Release with artifacts
9. Update documentation

## Notes

- The application uses platform-native secure storage (Windows Credential Manager)
- Configuration is stored in `%APPDATA%\com.milk.player\`
- Logs are stored in `%APPDATA%\com.milk.player\logs\`
- User data persists across updates and reinstalls

## Related Documentation

- **[Quick Build Guide](BUILDING.md)** - Fast-track build instructions
- **[Build Configuration](BUILD_CONFIGURATION.md)** - Detailed configuration reference
- **[Installation Testing](INSTALLATION_TESTING.md)** - Testing procedures
- **[Release Checklist](RELEASE_CHECKLIST.md)** - Release process
- **[Error Handling](ERROR_HANDLING.md)** - Error handling guidelines
- **[Performance Optimizations](PERFORMANCE_OPTIMIZATIONS.md)** - Performance tuning

---

📚 [Back to Documentation Map](README.md)
