#!/usr/bin/env bash
# Cross-platform build script for milk player

set -e

echo "🥛 milk Cross-Platform Build Script"
echo "===================================="
echo ""

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if cargo-xwin is installed
if ! command -v cargo-xwin &> /dev/null; then
    echo -e "${YELLOW}⚠️  cargo-xwin not found. Installing...${NC}"
    cargo install cargo-xwin
fi

# Check if Windows target is added
if ! rustup target list --installed | grep -q "x86_64-pc-windows-msvc"; then
    echo -e "${YELLOW}⚠️  Windows target not found. Installing...${NC}"
    rustup target add x86_64-pc-windows-msvc
fi

echo -e "${BLUE}📦 Building frontend...${NC}"
pnpm build

echo ""
echo -e "${BLUE}🍎 Building for macOS (aarch64-apple-darwin)...${NC}"
pnpm tauri build --target aarch64-apple-darwin

echo ""
echo -e "${BLUE}🪟 Building for Windows (x86_64-pc-windows-msvc)...${NC}"
pnpm tauri build --target x86_64-pc-windows-msvc

echo ""
echo -e "${GREEN}✅ Cross-platform build complete!${NC}"
echo ""
echo "Build artifacts:"
echo "  macOS:   src-tauri/target/aarch64-apple-darwin/release/milk"
echo "  Windows: src-tauri/target/x86_64-pc-windows-msvc/release/milk.exe"
echo ""
echo "Installers:"
find src-tauri/target -name "*.dmg" -o -name "*.app" -o -name "*.msi" -o -name "*.exe" 2>/dev/null | grep -E "release|bundle" || echo "  No installers found (may need to be built separately)"
