# milk Release Process

This document outlines the complete process for building, testing, and releasing milk for Windows.

## Prerequisites

Ensure you have completed the setup described in [BUILD_WINDOWS.md](./BUILD_WINDOWS.md).

## Release Workflow

### 1. Pre-Release Preparation

#### Update Version Numbers

Update the version in all relevant files:

```powershell
# Update package.json
$version = "0.1.0"  # Change this

# package.json
(Get-Content package.json) -replace '"version": ".*"', "`"version`": `"$version`"" | Set-Content package.json

# src-tauri/Cargo.toml
(Get-Content src-tauri/Cargo.toml) -replace 'version = ".*"', "version = `"$version`"" | Set-Content src-tauri/Cargo.toml

# src-tauri/tauri.conf.json
$config = Get-Content src-tauri/tauri.conf.json | ConvertFrom-Json
$config.version = $version
$config | ConvertTo-Json -Depth 10 | Set-Content src-tauri/tauri.conf.json
```

#### Run All Tests

```powershell
# Run all tests
pnpm test

# Run Rust tests
cd src-tauri
cargo test --release
cd ..

# Verify no failing tests
```

#### Update Changelog

Create or update `CHANGELOG.md`:

```markdown
## [0.1.0] - 2024-XX-XX

### Added

- Initial release
- Local audio playback (mp3, flac, wav)
- Winamp skin support (.wsz, .wal)
- Streaming service metadata (Spotify, YouTube)
- Real-time audio visualization
- Animated companion (farmer)
- First-run setup wizard

### Fixed

- [List any bug fixes]

### Changed

- [List any changes]
```

### 2. Build Release Artifacts

#### Clean Previous Builds

```powershell
# Clean previous builds
Remove-Item -Recurse -Force src-tauri/target/release -ErrorAction SilentlyContinue
Remove-Item -Recurse -Force dist -ErrorAction SilentlyContinue
Remove-Item -Recurse -Force build -ErrorAction SilentlyContinue

# Clean node modules and reinstall
Remove-Item -Recurse -Force node_modules -ErrorAction SilentlyContinue
pnpm install
```

#### Build Application

```powershell
# Build the application
pnpm run tauri:build:windows

# This creates:
# - src-tauri/target/x86_64-pc-windows-msvc/release/milk.exe
# - src-tauri/target/x86_64-pc-windows-msvc/release/bundle/msi/milk_0.1.0_x64.msi
# - src-tauri/target/x86_64-pc-windows-msvc/release/bundle/nsis/milk_0.1.0_x64-setup.exe
```

#### Create Portable Distribution

```powershell
# Create portable ZIP
.\scripts\create-portable.ps1

# This creates:
# - dist/milk_portable_v0.1.0.zip
```

#### Verify Build

```powershell
# Run verification script
.\scripts\verify-windows-build.ps1

# Should output: "✓ All checks passed!"
```

### 3. Testing

#### Test Executable

```powershell
# Run the executable directly
.\src-tauri\target\x86_64-pc-windows-msvc\release\milk.exe

# Verify:
# - Application starts in < 2 seconds
# - First-run wizard appears (if no config)
# - UI renders correctly
# - No console errors
```

#### Test MSI Installer

```powershell
# Install
$msiPath = "src-tauri\target\x86_64-pc-windows-msvc\release\bundle\msi\milk_0.1.0_x64.msi"
Start-Process msiexec.exe -ArgumentList "/i `"$msiPath`" /qb" -Wait

# Verify installation:
# 1. Check Start Menu
Test-Path "$env:APPDATA\Microsoft\Windows\Start Menu\Programs\milk.lnk"

# 2. Check Program Files
Test-Path "$env:LOCALAPPDATA\milk\milk.exe"

# 3. Test file associations
# - Create a test .wsz file
# - Right-click → Open With → Should show "milk"

# 4. Run installed application
& "$env:LOCALAPPDATA\milk\milk.exe"

# Uninstall
$productCode = (Get-WmiObject -Class Win32_Product | Where-Object { $_.Name -eq "milk" }).IdentifyingNumber
Start-Process msiexec.exe -ArgumentList "/x $productCode /qb" -Wait

# Verify uninstallation:
# - Application removed from Program Files
# - Start Menu shortcut removed
# - File associations removed (optional)
```

#### Test Portable Distribution

```powershell
# Extract portable ZIP
$testDir = "test-portable"
Expand-Archive -Path "dist\milk_portable_v0.1.0.zip" -DestinationPath $testDir -Force

# Run portable version
cd $testDir\portable
.\milk.exe

# Verify:
# - Runs without installation
# - Creates config in portable directory
# - No registry modifications
# - Can be moved to different location

# Cleanup
cd ..\..
Remove-Item -Recurse -Force $testDir
```

#### Test File Associations

```powershell
# After MSI installation, test file associations

# Create test skin file
$testSkin = "test.wsz"
"Test content" | Out-File $testSkin

# Check association
$assoc = cmd /c assoc .wsz 2>&1
$ftype = cmd /c ftype milk.wsz 2>&1

Write-Host "Association: $assoc"
Write-Host "File type: $ftype"

# Double-click test (manual)
# - Double-click test.wsz
# - Should open in milk

# Cleanup
Remove-Item $testSkin
```

### 4. Performance Verification

#### Measure Startup Time

```powershell
# Measure startup time
$stopwatch = [System.Diagnostics.Stopwatch]::StartNew()
$process = Start-Process -FilePath ".\src-tauri\target\x86_64-pc-windows-msvc\release\milk.exe" -PassThru
Start-Sleep -Seconds 3  # Wait for window to appear
$stopwatch.Stop()

Write-Host "Startup time: $($stopwatch.Elapsed.TotalSeconds) seconds"
# Target: < 2 seconds

# Close the process
$process.Kill()
```

#### Measure Memory Usage

```powershell
# Start application
$process = Start-Process -FilePath ".\src-tauri\target\x86_64-pc-windows-msvc\release\milk.exe" -PassThru

# Wait for startup
Start-Sleep -Seconds 5

# Measure memory
$memory = (Get-Process -Id $process.Id).WorkingSet64 / 1MB
Write-Host "Memory usage: $([math]::Round($memory, 2)) MB"
# Target: < 100MB idle

# Close
$process.Kill()
```

### 5. Create Release Package

#### Organize Release Files

```powershell
# Create release directory
$releaseDir = "release-v0.1.0"
New-Item -ItemType Directory -Path $releaseDir -Force

# Copy artifacts
Copy-Item "src-tauri\target\x86_64-pc-windows-msvc\release\bundle\msi\milk_0.1.0_x64.msi" $releaseDir
Copy-Item "src-tauri\target\x86_64-pc-windows-msvc\release\bundle\nsis\milk_0.1.0_x64-setup.exe" $releaseDir -ErrorAction SilentlyContinue
Copy-Item "dist\milk_portable_v0.1.0.zip" $releaseDir

# Copy documentation
Copy-Item "README.md" $releaseDir
Copy-Item "CHANGELOG.md" $releaseDir -ErrorAction SilentlyContinue
Copy-Item "LICENSE" $releaseDir -ErrorAction SilentlyContinue
```

#### Generate Checksums

```powershell
# Generate SHA256 checksums
cd $releaseDir
Get-ChildItem -File | ForEach-Object {
    $hash = (Get-FileHash $_.Name -Algorithm SHA256).Hash
    "$hash  $($_.Name)" | Out-File -Append "SHA256SUMS.txt"
}
cd ..

Write-Host "Checksums saved to $releaseDir\SHA256SUMS.txt"
```

#### Create Release Notes

Create `release-v0.1.0/RELEASE_NOTES.md`:

```markdown
# milk v0.1.0

## Downloads

- **MSI Installer** (Recommended): `milk_0.1.0_x64.msi`
  - Standard Windows installation
  - Includes file associations for .wsz and .wal files
  - Adds Start Menu shortcut

- **NSIS Installer**: `milk_0.1.0_x64-setup.exe`
  - Alternative installer
  - Smaller download size

- **Portable ZIP**: `milk_portable_v0.1.0.zip`
  - No installation required
  - Run from any location
  - Portable configuration

## Installation

### MSI Installer

1. Download `milk_0.1.0_x64.msi`
2. Double-click to install
3. Follow the installation wizard
4. Launch from Start Menu

### Portable Version

1. Download `milk_portable_v0.1.0.zip`
2. Extract to desired location
3. Run `milk.exe`

## System Requirements

- Windows 10/11 (64-bit)
- 100MB free disk space
- Audio output device

## What's New

[Copy from CHANGELOG.md]

## Known Issues

[List any known issues]

## Checksums

See `SHA256SUMS.txt` for file verification.
```

### 6. GitHub Release

#### Create Git Tag

```powershell
# Create and push tag
git tag -a v0.1.0 -m "Release v0.1.0"
git push origin v0.1.0
```

#### Create GitHub Release

1. Go to GitHub repository
2. Click "Releases" → "Draft a new release"
3. Select tag: `v0.1.0`
4. Release title: `milk v0.1.0`
5. Description: Copy from `RELEASE_NOTES.md`
6. Upload files from `release-v0.1.0/`:
   - `milk_0.1.0_x64.msi`
   - `milk_0.1.0_x64-setup.exe` (if available)
   - `milk_portable_v0.1.0.zip`
   - `SHA256SUMS.txt`
7. Check "Set as the latest release"
8. Click "Publish release"

### 7. Post-Release

#### Verify Release

```powershell
# Download from GitHub
# Test installation from downloaded files
# Verify checksums match
```

#### Update Documentation

- Update README.md with download links
- Update website (if applicable)
- Announce on social media/forums

#### Monitor Issues

- Watch for bug reports
- Respond to user feedback
- Plan next release

## Troubleshooting

### Build Fails

```powershell
# Clean everything and rebuild
Remove-Item -Recurse -Force src-tauri/target
Remove-Item -Recurse -Force node_modules
Remove-Item -Recurse -Force build
pnpm install
pnpm run tauri:build:windows
```

### MSI Not Generated

- Ensure WiX Toolset is installed
- Check PATH includes WiX bin directory
- Verify `tauri.conf.json` includes "msi" in targets

### File Associations Don't Work

- Check `src-tauri/wix/file-associations.wxs`
- Reinstall MSI
- Check Windows file association settings

### Size Exceeds Target

```powershell
# Analyze binary size
cargo install cargo-bloat
cd src-tauri
cargo bloat --release --crates

# Check for large dependencies
# Consider removing unused features
```

## Automation

For automated releases, see the GitHub Actions workflow in `.github/workflows/release.yml`.

## Support

For issues during the release process:

1. Check this documentation
2. Review build logs
3. Check GitHub Issues
4. Contact maintainers
