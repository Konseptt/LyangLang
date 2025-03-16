#!/bin/bash
# LyangLang Release Builder
# This script builds release packages for multiple platforms

set -e

VERSION="0.1.0"
RELEASE_DIR="release"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
BOLD='\033[1m'
NC='\033[0m' # No Color

echo -e "${BLUE}${BOLD}Building LyangLang Release Packages v${VERSION}${NC}"
echo

# Create release directory
mkdir -p "$RELEASE_DIR"

# Build for the current platform
echo -e "${GREEN}Building for current platform...${NC}"
cargo build --release

# Copy files to release directory
echo -e "${BLUE}Creating release package...${NC}"
mkdir -p "$RELEASE_DIR/lyangpiler-$VERSION"
cp target/release/lyangpiler "$RELEASE_DIR/lyangpiler-$VERSION/"
cp oimugchal oimugchal.cmd "$RELEASE_DIR/lyangpiler-$VERSION/"
cp install.sh install.cmd "$RELEASE_DIR/lyangpiler-$VERSION/"
cp README.md "$RELEASE_DIR/lyangpiler-$VERSION/"
cp -r examples.nbh example.nbh "$RELEASE_DIR/lyangpiler-$VERSION/" 2>/dev/null || :

# Create archives
echo -e "${BLUE}Creating archives...${NC}"
cd "$RELEASE_DIR"

# Create .tar.gz (for Linux/macOS)
tar -czf "lyangpiler-$VERSION-$(uname -s | tr '[:upper:]' '[:lower:]')-$(uname -m).tar.gz" "lyangpiler-$VERSION"

# Create .zip (for Windows)
if command -v zip >/dev/null 2>&1; then
    zip -r "lyangpiler-$VERSION-windows.zip" "lyangpiler-$VERSION"
fi

cd ..

echo -e "${GREEN}${BOLD}Release packages created in ${RELEASE_DIR}!${NC}"
echo

# Optional: Build for other platforms using cross-compilation
# This would require additional setup with tools like cross or cargo-xbuild
if command -v cross >/dev/null 2>&1; then
    echo -e "${YELLOW}Cross-compilation available. Building for other platforms...${NC}"
    
    # Build for Linux (x86_64)
    echo -e "${BLUE}Building for Linux (x86_64)...${NC}"
    cross build --target x86_64-unknown-linux-gnu --release
    
    # Build for macOS (x86_64)
    echo -e "${BLUE}Building for macOS (x86_64)...${NC}"
    cross build --target x86_64-apple-darwin --release
    
    # Build for Windows (x86_64)
    echo -e "${BLUE}Building for Windows (x86_64)...${NC}"
    cross build --target x86_64-pc-windows-gnu --release
    
    # Create platform-specific packages (simplified)
    echo -e "${GREEN}Created cross-platform builds in target/{target}/release${NC}"
else
    echo -e "${YELLOW}For cross-platform builds, install 'cross' tool:${NC}"
    echo -e "  ${BOLD}cargo install cross${NC}"
fi

echo -e "${GREEN}${BOLD}Release process completed!${NC}"