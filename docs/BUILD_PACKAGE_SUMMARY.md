# Build and Package Summary

This document summarizes the build and packaging infrastructure for milk on Windows.

## Overview

The milk application is configured for Windows x64 builds with comprehensive packaging options including MSI installer, NSIS installer, and portable ZIP distribution.

## Build Configuration

### Tauri Configuration (`src-tauri/tauri.conf.json`)

- **Product Name**: milk
- **Version**: 0.1.0
- **Identifier**: com.milk.player
- **Bundle Targets**: MSI, NSIS
- **File Associations**: .wsz, .wal (Winamp skins)

### Cargo Release Profile (`src-tauri/Cargo.toml`)

Optimized for size:
- `opt-level = "z"` - Maximum size optimization
- `lto = true` - Link-time optimization enabled
- `codegen-units = 1` - Single codegen unit for better optimization
- `panic = "abort"` - Abort on panic (smaller binary)
- `strip = true` - Strip debug symbols

**Target Size**: < 15MB (Requirement 8.1)

### Package Scripts (`package.json`)

- `tauri:build` - Build for current platform
- `tauri:build:windows` - Build specifically for Windows x64
- `tauri:build:debug` - Debug build
- `test` - Run all tests

## Build Artifacts

### 1. Executable
- **Location**: `src-tauri/target/x86_64-pc-windows-msvc/release/milk.exe`
- **Target Size**: < 15MB
- **Features**: Fully optimized, stripped binary

### 2. MSI Installer
- **Location**: `src-tauri/target/x86_64-pc-windows-msvc/release/bundle/msi/milk_0.1.0_x64.msi`
- **Features**:
  - Standard Windows installation
  - Start Menu shortcuts
  - File associations (.wsz, .wal)
  - Uninstaller registration
  - WiX-based installer

### 3. NSIS Installer
- **Location**: `src-tauri/target/x86_64-pc-windows-msvc/release/bundle/nsis/milk_0.1.0_x64-setup.exe`
- **Features**:
  - Alternative installer format
  - Smaller download size
  - Per-user installation

### 4. Portable Distribution
- **Location**: `dist/milk_portable_v0.1.0.zip`
- **Contents**:
  - milk.exe
  - README.txt
  - assets/ directory
- **Features**:
  - No installation required
  - Portable configuration
  - USB-friendly

## Scripts

### Build Scripts

#### `scripts/create-portable.ps1` (PowerShell)
Creates portable ZIP distribution with:
- Executable
- README documentation
- Assets directory structure
- Size verification

#### `scripts/create-portable.sh` (Bash)
Cross-platform version for Unix-like systems.

### Verification Scripts

#### `scripts/verify-windows-build.ps1`
Comprehensive build verification:
1. Executable existence and size
2. MSI installer presence
3. NSIS installer presence
4. File association configuration
5. Icon files
6. Tauri configuration
7. Cargo optimization settings
8. Executable validity
9. Dependencies check
10. Portable script presence

#### `scripts/verify-build.ps1`
General build verification (existing).

## Documentation

### Build Documentation

#### `docs/BUILD_WINDOWS.md`
Complete guide for building on Windows:
- Prerequisites (Rust, Node.js, Visual Studio, WiX)
- Build instructions
- Cross-compilation notes
- Troubleshooting
- CI/CD integration
- Performance targets

#### `docs/RELEASE_PROCESS.md`
Step-by-step release workflow:
- Version updates
- Testing procedures
- Artifact creation
- Performance verification
- Release packaging
- GitHub release process
- Post-release tasks

#### `docs/BUILD_PACKAGE_SUMMARY.md` (this file)
Overview of build infrastructure.

## CI/CD Integration

### GitHub Actions Workflow (`.github/workflows/build-windows.yml`)

Automated build pipeline:
1. Checkout code
2. Setup Node.js, pnpm, Rust
3. Install dependencies
4. Run tests (frontend and backend)
5. Build application
6. Verify executable size
7. Create portable distribution
8. Run verification
9. Generate checksums
10. Upload artifacts
11. Create GitHub release (on tag push)

**Triggers**:
- Push to tags matching `v*`
- Manual workflow dispatch

**Artifacts**:
- MSI installer
- NSIS installer
- Portable ZIP
- SHA256 checksums

## File Associations

### Configuration (`src-tauri/wix/file-associations.wxs`)

Registers file associations for:
- `.wsz` - Winamp Skin (ZIP format)
- `.wal` - Winamp Skin (WAL format)

Both open with milk when double-clicked.

## Testing

### Build Testing

1. **Executable Test**
   - Direct execution
   - Startup time (< 2s target)
   - Memory usage (< 100MB idle)

2. **MSI Installer Test**
   - Installation
   - Start Menu shortcuts
   - File associations
   - Uninstallation

3. **Portable Test**
   - Extract and run
   - Portable configuration
   - No registry modifications

4. **File Association Test**
   - .wsz file opens in milk
   - .wal file opens in milk

### Performance Targets

- **Executable Size**: < 15MB
- **Startup Time**: < 2 seconds
- **Memory Usage (Idle)**: < 100MB
- **Build Time**: < 5 minutes

## Build Requirements

### On Windows (Native - Recommended)

1. **Rust** (stable, x86_64-pc-windows-msvc target)
2. **Node.js** (v18+)
3. **pnpm** (v8+)
4. **Visual Studio Build Tools** (C++ support)
5. **WiX Toolset** (for MSI generation)

### Cross-Compilation (macOS/Linux)

Cross-compilation from macOS/Linux to Windows is complex and requires:
- Windows SDK
- Cross-compilation toolchain
- Wine (for testing)

**Recommendation**: Use Windows VM or CI/CD for production builds.

## Build Commands

### Quick Reference

```powershell
# Install dependencies
pnpm install

# Run tests
pnpm test

# Build release
pnpm run tauri:build:windows

# Create portable
.\scripts\create-portable.ps1

# Verify build
.\scripts\verify-windows-build.ps1
```

### Full Build Process

```powershell
# Clean
Remove-Item -Recurse -Force src-tauri/target/release
Remove-Item -Recurse -Force dist

# Install
pnpm install

# Test
pnpm test
cd src-tauri && cargo test --release && cd ..

# Build
pnpm run tauri:build:windows

# Package
.\scripts\create-portable.ps1

# Verify
.\scripts\verify-windows-build.ps1
```

## Troubleshooting

### Common Issues

1. **"link.exe not found"**
   - Install Visual Studio Build Tools with C++ support

2. **MSI not generated**
   - Install WiX Toolset
   - Ensure WiX is in PATH

3. **Size exceeds 15MB**
   - Check Cargo.toml release profile
   - Run `cargo bloat --release --crates`
   - Review dependencies

4. **Cross-compilation fails**
   - Use native Windows build
   - Or use CI/CD with Windows runner

## Distribution

### Release Artifacts

For each release, provide:

1. **MSI Installer** - Standard installation
2. **NSIS Installer** - Alternative installer
3. **Portable ZIP** - No installation required
4. **SHA256SUMS.txt** - Checksums for verification

### Naming Convention

```
milk_<version>_<arch>.<ext>
milk_0.1.0_x64.msi
milk_0.1.0_x64-setup.exe
milk_portable_v0.1.0.zip
```

## Status

✅ **Completed**:
- Tauri configuration optimized for Windows x64
- Cargo release profile configured for size optimization
- MSI installer with file associations
- NSIS installer configuration
- Portable distribution scripts (PowerShell and Bash)
- Comprehensive verification script
- Build documentation
- Release process documentation
- GitHub Actions CI/CD workflow
- Icons and assets in place

⚠️ **Note**:
- Actual build requires Windows environment
- Cross-compilation from macOS has limitations
- Use CI/CD or Windows VM for production builds

## Next Steps

To complete the build and packaging:

1. **On Windows machine**:
   ```powershell
   pnpm run tauri:build:windows
   .\scripts\create-portable.ps1
   .\scripts\verify-windows-build.ps1
   ```

2. **Test all artifacts**:
   - Test executable
   - Test MSI installation
   - Test portable distribution
   - Test file associations

3. **Create release**:
   - Follow `docs/RELEASE_PROCESS.md`
   - Upload to GitHub Releases

## References

- [BUILD_WINDOWS.md](./BUILD_WINDOWS.md) - Detailed build instructions
- [RELEASE_PROCESS.md](./RELEASE_PROCESS.md) - Release workflow
- [Tauri Documentation](https://tauri.app/)
- [WiX Toolset](https://wixtoolset.org/)
- [Cargo Book - Release Profiles](https://doc.rust-lang.org/cargo/reference/profiles.html)
