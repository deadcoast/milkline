#!/bin/bash
# Script to create portable ZIP distribution for milk

set -e

VERSION="0.1.0"
DIST_DIR="dist/portable"
ARCHIVE_NAME="milk_portable_v${VERSION}.zip"

echo "Creating portable distribution for milk v${VERSION}..."

# Clean and create distribution directory
rm -rf "$DIST_DIR"
mkdir -p "$DIST_DIR"

# Copy executable
echo "Copying executable..."
cp src-tauri/target/release/milk.exe "$DIST_DIR/"

# Create README for portable version
echo "Creating README..."
cat > "$DIST_DIR/README.txt" << 'EOF'
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
EOF

# Create assets directory with default skin info
echo "Creating assets directory..."
mkdir -p "$DIST_DIR/assets"
cat > "$DIST_DIR/assets/README.txt" << 'EOF'
Place Winamp skin files (.wsz, .wal) in this directory.

The application will use the default skin on first launch.
You can load custom skins from the application menu.
EOF

# Get executable size
EXE_SIZE=$(stat -f%z "$DIST_DIR/milk.exe" 2>/dev/null || stat -c%s "$DIST_DIR/milk.exe" 2>/dev/null || echo "unknown")
EXE_SIZE_MB=$(echo "scale=2; $EXE_SIZE / 1048576" | bc 2>/dev/null || echo "unknown")

echo "Executable size: ${EXE_SIZE_MB}MB"

# Create ZIP archive
echo "Creating ZIP archive..."
cd dist
zip -r "$ARCHIVE_NAME" portable/
cd ..

echo ""
echo "✓ Portable distribution created successfully!"
echo "  Location: dist/$ARCHIVE_NAME"
echo "  Executable size: ${EXE_SIZE_MB}MB"
echo ""
echo "To test:"
echo "  1. Extract the ZIP to a folder"
echo "  2. Run milk.exe"
echo "  3. Verify the application runs without installation"
