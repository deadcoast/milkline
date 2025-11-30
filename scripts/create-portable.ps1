# PowerShell script to create portable ZIP distribution for milk

$ErrorActionPreference = "Stop"

$VERSION = "0.1.0"
$DIST_DIR = "dist/portable"
$ARCHIVE_NAME = "milk_portable_v$VERSION.zip"

Write-Host "Creating portable distribution for milk v$VERSION..." -ForegroundColor Cyan

# Clean and create distribution directory
if (Test-Path $DIST_DIR) {
    Remove-Item -Recurse -Force $DIST_DIR
}
New-Item -ItemType Directory -Path $DIST_DIR -Force | Out-Null

# Copy executable
Write-Host "Copying executable..." -ForegroundColor Yellow
Copy-Item "src-tauri/target/release/milk.exe" "$DIST_DIR/"

# Create README for portable version
Write-Host "Creating README..." -ForegroundColor Yellow
$readmeContent = @"
milk - Portable Version
=======================

This is the portable version of milk, a desktop audio visual media buddy.

USAGE:
------
Simply run milk.exe to start the application.

Configuration and playlists will be stored in the same directory as the executable.

FEATURES:
---------
- Local audio playback (mp3, flac, wav)
- Winamp skin support (.wsz, .wal)
- Streaming service metadata (Spotify, YouTube)
- Real-time audio visualization
- Animated companion (farmer)

REQUIREMENTS:
-------------
- Windows 10/11
- Audio output device

FIRST RUN:
----------
On first launch, farmer will guide you through setup:
1. Select your music library folder
2. (Optional) Configure Spotify/YouTube credentials
3. (Optional) Load a Winamp skin

SUPPORT:
--------
For issues and updates, visit: https://github.com/milk-player/milk

LICENSE:
--------
MIT License - See LICENSE file for details
"@
Set-Content -Path "$DIST_DIR/README.txt" -Value $readmeContent

# Create assets directory with default skin info
Write-Host "Creating assets directory..." -ForegroundColor Yellow
New-Item -ItemType Directory -Path "$DIST_DIR/assets" -Force | Out-Null
$assetsReadme = @"
Place Winamp skin files (.wsz, .wal) in this directory.

The application will use the default skin on first launch.
You can load custom skins from the application menu.
"@
Set-Content -Path "$DIST_DIR/assets/README.txt" -Value $assetsReadme

# Get executable size
$exeFile = Get-Item "$DIST_DIR/milk.exe"
$exeSizeMB = [math]::Round($exeFile.Length / 1MB, 2)

Write-Host "Executable size: $exeSizeMB MB" -ForegroundColor Green

# Check if size meets requirement
if ($exeSizeMB -lt 15) {
    Write-Host "✓ Size requirement met (<15MB)" -ForegroundColor Green
} else {
    Write-Host "⚠ Warning: Executable size exceeds 15MB target" -ForegroundColor Red
}

# Create ZIP archive
Write-Host "Creating ZIP archive..." -ForegroundColor Yellow
$archivePath = "dist/$ARCHIVE_NAME"
if (Test-Path $archivePath) {
    Remove-Item $archivePath
}
Compress-Archive -Path "$DIST_DIR/*" -DestinationPath $archivePath

Write-Host ""
Write-Host "✓ Portable distribution created successfully!" -ForegroundColor Green
Write-Host "  Location: $archivePath" -ForegroundColor Cyan
Write-Host "  Executable size: $exeSizeMB MB" -ForegroundColor Cyan
Write-Host ""
Write-Host "To test:" -ForegroundColor Yellow
Write-Host "  1. Extract the ZIP to a folder"
Write-Host "  2. Run milk.exe"
Write-Host "  3. Verify the application runs without installation"
