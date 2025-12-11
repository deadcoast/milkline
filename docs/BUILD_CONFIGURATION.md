# Build Configuration Summary

This document summarizes the build and packaging configuration for milk.

> **Documentation Map**: See [docs/README.md](README.md) for all documentation  
> **Build Guide**: See [BUILD.md](BUILD.md) for build instructions

## Configuration Files

### Tauri Configuration (`src-tauri/tauri.conf.json`)

**Bundle Settings:**

- **Targets**: MSI and NSIS installers for Windows
- **Publisher**: milk contributors
- **Category**: Audio
- **Identifier**: com.milk.player

**Windows-Specific:**

- **MSI Installer**: WiX-based, English language
- **NSIS Installer**: Per-user installation, no admin required
- **File Associations**: Configured via WiX fragment (see below)

### Cargo Configuration (`src-tauri/Cargo.toml`)

**Release Profile Optimizations:**

```toml
[profile.release]
opt-level = "z"     # Optimize for size
lto = true          # Enable Link Time Optimization
codegen-units = 1   # Reduce number of codegen units
panic = "abort"     # Abort on panic (smaller binary)
strip = true        # Strip symbols from binary
```

**Target Binary Size**: <15MB (Requirement 8.1)

### Package Configuration (`package.json`)

**Build Scripts:**

- `pnpm tauri:dev` - Development mode
- `pnpm tauri:build` - Production build
- `pnpm tauri:build:debug` - Debug release build

## File Associations

### WiX Fragment (`src-tauri/wix/file-associations.wxs`)

Registers file associations for:

- **.wsz** - Winamp Skin (ZIP format)
- **.wal** - Winamp Skin (WAL format)

Both file types open with milk when double-clicked.

**Implementation:**

- ProgId: `milk.wsz` and `milk.wal`
- MIME Type: `application/x-winamp-skin`
- Verb: Open with milk
- Command: `milk.exe "%1"`

## Build Artifacts

### Executable

- **Location**: `src-tauri/target/release/milk.exe`
- **Size Target**: <15MB
- **Optimizations**: Size-optimized with LTO and symbol stripping

### MSI Installer

- **Location**: `src-tauri/target/release/bundle/msi/milk_0.1.0_x64_en-US.msi`
- **Features**:
  - Program Files installation
  - Start Menu shortcuts
  - File associations (.wsz, .wal)
  - Uninstaller registration
  - Silent installation support

### NSIS Installer

- **Location**: `src-tauri/target/release/bundle/nsis/milk_0.1.0_x64-setup.exe`
- **Features**:
  - Modern installer UI
  - Per-user installation (no admin)
  - File associations
  - Uninstaller included

### Portable Distribution

- **Location**: `dist/milk_portable_v0.1.0.zip`
- **Contents**:
  - milk.exe
  - README.txt
  - assets/ folder
- **Features**:
  - No installation required
  - Configuration stored locally
  - Fully portable

## Build Process

### Prerequisites

1. Rust toolchain (1.70+)
2. Node.js (18+) and pnpm
3. WiX Toolset 3.11+ (for MSI)
4. NSIS (optional, for NSIS installer)

### Build Steps

1. **Install Dependencies**

   ```bash
   pnpm install
   ```

2. **Build Release**

   ```bash
   pnpm tauri:build
   ```

3. **Verify Build**

   ```powershell
   .\scripts\verify-build.ps1
   ```

4. **Create Portable**
   ```powershell
   .\scripts\create-portable.ps1
   ```

## Verification Checklist

- [ ] Executable size <15MB
- [ ] MSI installer generated
- [ ] NSIS installer generated (if NSIS installed)
- [ ] File associations configured
- [ ] Portable distribution created
- [ ] All tests pass

## Performance Targets

As per requirements:

- **Executable size**: <15MB (Requirement 8.1)
- **RAM usage (idle)**: <100MB (Requirement 8.2)
- **Startup time**: <2 seconds (Requirement 8.3)

## Distribution

### GitHub Release

Upload the following artifacts:

1. `milk.exe` - Standalone executable
2. `milk_0.1.0_x64_en-US.msi` - MSI installer
3. `milk_0.1.0_x64-setup.exe` - NSIS installer
4. `milk_portable_v0.1.0.zip` - Portable distribution

### Installation Methods

**MSI Installer:**

- Double-click to install
- Silent: `msiexec /i milk_0.1.0_x64_en-US.msi /quiet`

**NSIS Installer:**

- Double-click to install
- Silent: `milk_0.1.0_x64-setup.exe /S`

**Portable:**

- Extract ZIP
- Run milk.exe

## User Data Locations

### Installed Version

- **Config**: `%APPDATA%\com.milk.player\config.json`
- **Playlists**: `%APPDATA%\com.milk.player\playlists\`
- **Logs**: `%APPDATA%\com.milk.player\logs\`
- **Cache**: `%APPDATA%\com.milk.player\cache\`

### Portable Version

- **Config**: `.\config.json` (same directory as exe)
- **Playlists**: `.\playlists\`
- **Logs**: `.\logs\`
- **Cache**: `.\cache\`

## Troubleshooting

### Build Fails

- Ensure Rust toolchain is installed
- Ensure Visual Studio Build Tools installed
- Run `rustup target add x86_64-pc-windows-msvc`

### MSI Not Generated

- Install WiX Toolset 3.11+
- Add WiX to PATH

### Binary Too Large

- Verify release profile in Cargo.toml
- Check for unnecessary dependencies
- Use `cargo bloat --release` to analyze

### File Associations Not Working

- Verify WiX fragment is included
- Check tauri.conf.json references fragment
- Reinstall application

## CI/CD Integration

See `.github/workflows/build.yml.template` for automated build workflow.

## Documentation

- [BUILD.md](../BUILD.md) - Detailed build guide
- [BUILDING.md](../BUILDING.md) - Quick build reference
- [RELEASE_CHECKLIST.md](../RELEASE_CHECKLIST.md) - Release process
- [INSTALLATION_TESTING.md](../INSTALLATION_TESTING.md) - Testing guide

## Notes

- The application uses platform-native secure storage (Windows Credential Manager)
- Configuration persists across updates
- User data is preserved during uninstallation
- File associations are registered during installation
- Portable version requires no installation or admin rights

## Related Documentation

- **[BUILD.md](BUILD.md)** - Comprehensive build guide
- **[BUILDING.md](BUILDING.md)** - Quick build reference
- **[INSTALLATION_TESTING.md](INSTALLATION_TESTING.md)** - Testing procedures
- **[RELEASE_CHECKLIST.md](RELEASE_CHECKLIST.md)** - Release process
- **[Technical Specification](milk_tech_spec.md)** - Architecture and design

---

📚 [Back to Documentation Map](README.md)
