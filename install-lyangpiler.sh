#!/bin/bash
# One-line installer for LyangLang
# Usage: curl -sSL https://github.com/konseptt/LyangLang/raw/main/install-lyangpiler.sh | bash

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
BOLD='\033[1m'
NC='\033[0m' # No Color

# Print banner
echo -e "${BLUE}${BOLD}"
echo "  _                           _                   "
echo " | |                         | |                  "
echo " | |    _   _  __ _ _ __   __| |_ __   __ _ _ __  "
echo " | |   | | | |/ _\` | '_ \ / _\` | '_ \ / _\` | '_ \ "
echo " | |___| |_| | (_| | | | | (_| | |_) | (_| | | | |"
echo " |______\__, |\__,_|_| |_|\__,_| .__/ \__, |_| |_|"
echo "         __/ |                 | |     __/ |      "
echo "        |___/                  |_|    |___/       "
echo -e "${NC}"
echo -e "${YELLOW}LyangLang Programming Language Installer${NC}"
echo

# Detect OS
detect_os() {
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        echo "linux"
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        echo "macos"
    elif [[ "$OSTYPE" == "cygwin" ]]; then
        echo "windows"
    elif [[ "$OSTYPE" == "msys" ]]; then
        echo "windows"
    elif [[ "$OSTYPE" == "win32" ]]; then
        echo "windows"
    else
        echo "unknown"
    fi
}

# Detect architecture
detect_arch() {
    local arch=$(uname -m)
    if [[ "$arch" == "x86_64" ]] || [[ "$arch" == "amd64" ]]; then
        echo "amd64"
    elif [[ "$arch" == "aarch64" ]] || [[ "$arch" == "arm64" ]]; then
        echo "arm64"
    else
        echo "unsupported"
    fi
}

OS=$(detect_os)
ARCH=$(detect_arch)

if [[ "$OS" == "unknown" ]] || [[ "$ARCH" == "unsupported" ]]; then
    echo -e "${RED}Unsupported OS or architecture: $OS $ARCH${NC}"
    exit 1
fi

# Set download URLs based on OS and architecture
GITHUB_REPO="https://github.com/konseptt/LyangLang"
VERSION="v0.1.0" # Update this as needed

if [[ "$OS" == "windows" ]]; then
    DOWNLOAD_URL="$GITHUB_REPO/releases/download/$VERSION/lyangpiler-windows-$ARCH.zip"
    ARCHIVE_TYPE="zip"
else
    DOWNLOAD_URL="$GITHUB_REPO/releases/download/$VERSION/lyangpiler-$OS-$ARCH.tar.gz"
    ARCHIVE_TYPE="tar.gz"
fi

# Temporary directory
TEMP_DIR=$(mktemp -d)
trap 'rm -rf "$TEMP_DIR"' EXIT

echo -e "${BLUE}Downloading LyangLang...${NC}"

# Download the appropriate archive
if command -v curl >/dev/null 2>&1; then
    curl -L "$DOWNLOAD_URL" -o "$TEMP_DIR/lyangpiler.$ARCHIVE_TYPE"
elif command -v wget >/dev/null 2>&1; then
    wget "$DOWNLOAD_URL" -O "$TEMP_DIR/lyangpiler.$ARCHIVE_TYPE"
else
    echo -e "${RED}Neither curl nor wget found. Please install one of them.${NC}"
    exit 1
fi

# Extract the archive
echo -e "${BLUE}Extracting...${NC}"

cd "$TEMP_DIR"
if [[ "$ARCHIVE_TYPE" == "zip" ]]; then
    if command -v unzip >/dev/null 2>&1; then
        unzip "lyangpiler.$ARCHIVE_TYPE"
    else
        echo -e "${RED}unzip command not found. Please install it.${NC}"
        exit 1
    fi
else
    tar -xzf "lyangpiler.$ARCHIVE_TYPE"
fi

# Run the appropriate installer
echo -e "${GREEN}Running installer...${NC}"

if [[ "$OS" == "windows" ]]; then
    # On Windows with WSL or similar
    cd lyangpiler
    ./install.cmd
else
    # On Unix-like systems
    cd lyangpiler
    chmod +x ./install.sh
    ./install.sh
fi

echo -e "${GREEN}${BOLD}LyangLang has been successfully installed!${NC}"
echo -e "You can now use ${BOLD}oimugchal${NC} to create and run LyangLang programs."
echo
echo -e "Try it out:"
echo -e "  ${BOLD}oimugchal new my-project${NC}"
echo -e "  ${BOLD}cd my-project${NC}"
echo -e "  ${BOLD}oimugchal run${NC}"