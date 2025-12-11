# Building milk for Windows

This document describes how to build and package the milk application for Windows.

## Prerequisites

### On Windows (Native Build - Recommended)

1. **Install Rust**

   ```powershell
   # Download and run rustup-init.exe from https://rustup.rs/
   # Or use winget:
   winget install Rustlang.Rustup
   ```

2. **Install Node.js and pnpm**

   ```powershell
   winget install OpenJS.NodeJS
   npm install -g pnpm
   ```

3. **Install Visual Studio Build Tools**
   - Download from: https://visualstudio.microsoft.com/downloads/
   - Select "Desktop development with C++"
   - Or install via winget:

   ```powershell
   winget install Microsoft.VisualStudio.2022.BuildTools
   ```

4. **Install WiX Toolset** (for MSI installer)
   ```powershell
   # Download from https://wixtoolset.org/
   # Or use chocolatey:
   choco install wixtoolset
   ```

### On macOS/Linux (Cross-Compilation)

Cross-compiling to Windows from macOS/Linux requires:

1. Rust with Windows target: `rustup target add x86_64-pc-windows-msvc`
2. Windows SDK and cross-compilation toolchain (complex setup)
3. Wine for testing (optional)

**Note**: Native Windows builds are strongly recommended for production releases.

## Building on Windows

### 1. Clone and Setup

```powershell
git clone <repository-url>
cd milk
pnpm install
```

### 2. Build Release Executable

```powershell
# Build the application
pnpm run tauri:build:windows
```

This will:

- Build the Svelte frontend
- Compile the Rust backend with optimizations
- Create the executable at: `src-tauri/target/x86_64-pc-windows-msvc/release/milk.exe`
- Generate MSI installer at: `src-tauri/target/x86_64-pc-windows-msvc/release/bundle/msi/milk_0.1.0_x64.msi`
- Generate NSIS installer at: `src-tauri/target/x86_64-pc-windows-msvc/release/bundle/nsis/milk_0.1.0_x64-setup.exe`

### 3. Verify Executable Size

```powershell
# Check the size of the executable
$exe = Get-Item "src-tauri/target/x86_64-pc-windows-msvc/release/milk.exe"
$sizeMB = [math]::Round($exe.Length / 1MB, 2)
Write-Host "Executable size: $sizeMB MB"

# Target: <15MB
if ($sizeMB -lt 15) {
    Write-Host "✓ Size requirement met" -ForegroundColor Green
} else {
    Write-Host "⚠ Warning: Exceeds 15MB target" -ForegroundColor Yellow
}
```

### 4. Create Portable Distribution

```powershell
# Run the portable distribution script
.\scripts\create-portable.ps1
```

This creates:

- `dist/milk_portable_v0.1.0.zip` - Portable ZIP distribution
- Contains: executable, README, and assets folder

### 5. Test the Build

#### Test Executable

```powershell
# Run the executable directly
.\src-tauri\target\x86_64-pc-windows-msvc\release\milk.exe
```

#### Test MSI Installer

```powershell
# Install the MSI
msiexec /i src-tauri\target\x86_64-pc-windows-msvc\release\bundle\msi\milk_0.1.0_x64.msi

# Verify installation
# - Check Start Menu for "milk" shortcut
# - Check Program Files for installation
# - Test file associations (.wsz, .wal files)

# Uninstall
msiexec /x src-tauri\target\x86_64-pc-windows-msvc\release\bundle\msi\milk_0.1.0_x64.msi
```

#### Test Portable Distribution

```powershell
# Extract the ZIP
Expand-Archive -Path dist\milk_portable_v0.1.0.zip -DestinationPath test-portable

# Run from extracted location
cd test-portable\portable
.\milk.exe

# Verify:
# - Application runs without installation
# - Configuration is stored in portable directory
# - No registry entries created
```

## Build Configuration

### Cargo.toml Optimizations

The release profile in `src-tauri/Cargo.toml` is configured for size optimization:

```toml
[profile.release]
opt-level = "z"     # Optimize for size
lto = true          # Enable Link Time Optimization
codegen-units = 1   # Reduce number of codegen units
panic = "abort"     # Abort on panic
strip = true        # Strip symbols from binary
```

### Tauri Configuration

The bundle configuration in `src-tauri/tauri.conf.json`:

```json
{
  "bundle": {
    "active": true,
    "targets": ["msi", "nsis"],
    "publisher": "milk contributors",
    "windows": {
      "wix": {
        "language": "en-US",
        "fragmentPaths": ["wix/file-associations.wxs"]
      }
    }
  }
}
```

## File Associations

The MSI installer automatically registers file associations for:

- `.wsz` - Winamp Skin (ZIP format)
- `.wal` - Winamp Skin (WAL format)

Configuration is in: `src-tauri/wix/file-associations.wxs`

## Troubleshooting

### Build Fails with "link.exe not found"

**Solution**: Install Visual Studio Build Tools with C++ support.

### MSI Generation Fails

**Solution**: Ensure WiX Toolset is installed and in PATH.

### Executable Size Too Large

**Possible causes**:

1. Debug symbols not stripped (check `strip = true` in Cargo.toml)
2. Dependencies with large assets
3. LTO not enabled

**Solutions**:

```powershell
# Verify release profile settings
cargo build --release --verbose

# Check what's taking space
cargo bloat --release --crates
```

### Cross-Compilation Errors

If attempting to cross-compile from macOS/Linux:

- Use a Windows VM or CI/CD pipeline
- Or use Docker with Windows SDK
- Native builds are recommended

## CI/CD Integration

### GitHub Actions Example

```yaml
name: Build Windows Release

on:
  push:
    tags:
      - "v*"

jobs:
  build-windows:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v3

      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: "18"

      - name: Setup pnpm
        uses: pnpm/action-setup@v2
        with:
          version: 8

      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: x86_64-pc-windows-msvc

      - name: Install dependencies
        run: pnpm install

      - name: Build application
        run: pnpm run tauri:build:windows

      - name: Create portable distribution
        run: .\scripts\create-portable.ps1

      - name: Upload artifacts
        uses: actions/upload-artifact@v3
        with:
          name: windows-release
          path: |
            src-tauri/target/x86_64-pc-windows-msvc/release/bundle/msi/*.msi
            src-tauri/target/x86_64-pc-windows-msvc/release/bundle/nsis/*.exe
            dist/*.zip
```

## Release Checklist

Before releasing:

- [ ] All tests pass (`pnpm test`)
- [ ] Executable size < 15MB
- [ ] MSI installer installs correctly
- [ ] File associations work (.wsz, .wal)
- [ ] Application starts in < 2 seconds
- [ ] First-run setup wizard appears
- [ ] Portable version runs without installation
- [ ] Uninstaller removes all files and registry entries
- [ ] Version number updated in:
  - [ ] `package.json`
  - [ ] `src-tauri/Cargo.toml`
  - [ ] `src-tauri/tauri.conf.json`

## Distribution

### Release Artifacts

For each release, provide:

1. **MSI Installer** - `milk_x.x.x_x64.msi`
   - For standard installation
   - Includes file associations
   - Adds to Start Menu

2. **NSIS Installer** - `milk_x.x.x_x64-setup.exe`
   - Alternative installer
   - Smaller download size

3. **Portable ZIP** - `milk_portable_vx.x.x.zip`
   - No installation required
   - Portable configuration
   - For USB drives or temporary use

### Naming Convention

```
milk_<version>_<arch>.<ext>
milk_0.1.0_x64.msi
milk_0.1.0_x64-setup.exe
milk_portable_v0.1.0.zip
```

## Performance Targets

The build should meet these requirements:

- **Executable Size**: < 15MB
- **RAM Usage (Idle)**: < 100MB
- **Startup Time**: < 2 seconds
- **Build Time**: < 5 minutes (release build)

## Support

For build issues:

1. Check this documentation
2. Review GitHub Issues
3. Check Tauri documentation: https://tauri.app/
4. Open a new issue with build logs
