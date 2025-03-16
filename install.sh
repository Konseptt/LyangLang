#!/bin/bash
# LyangLang Installer Script
# Usage: curl -sSf https://raw.githubusercontent.com/konseptt/LyangLang/main/install.sh | sh

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}Installing LyangLang...${NC}"

# Detect OS and architecture
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Linux*)     os=linux ;;
    Darwin*)    os=darwin ;;
    MINGW*)     os=windows ;;
    MSYS*)      os=windows ;;
    *)          echo -e "${RED}Unsupported operating system: $OS${NC}" && exit 1 ;;
esac

case "$ARCH" in
    x86_64*)    arch=x86_64 ;;
    aarch64*)   arch=aarch64 ;;
    arm64*)     arch=aarch64 ;;
    *)          echo -e "${RED}Unsupported architecture: $ARCH${NC}" && exit 1 ;;
esac

# Set installation directory
if [ "$os" = "windows" ]; then
    INSTALL_DIR="$USERPROFILE/.lyangpiler"
else
    INSTALL_DIR="$HOME/.lyangpiler"
fi

# Create installation directory
mkdir -p "$INSTALL_DIR"
mkdir -p "$INSTALL_DIR/bin"

# Download latest release
RELEASE_URL="https://github.com/konseptt/LyangLang/releases/latest/download/lyangpiler-${os}-${arch}.tar.gz"
echo -e "${BLUE}Downloading LyangLang from ${RELEASE_URL}...${NC}"

if command -v curl > /dev/null; then
    curl -L "$RELEASE_URL" | tar xz -C "$INSTALL_DIR"
elif command -v wget > /dev/null; then
    wget -qO- "$RELEASE_URL" | tar xz -C "$INSTALL_DIR"
else
    echo -e "${RED}Neither curl nor wget found. Please install one of them.${NC}"
    exit 1
fi

# Add to PATH in shell configuration
SHELL_CONFIG=""
case "$SHELL" in
    */zsh)
        SHELL_CONFIG="$HOME/.zshrc"
        ;;
    */bash)
        if [ "$os" = "darwin" ]; then
            SHELL_CONFIG="$HOME/.bash_profile"
        else
            SHELL_CONFIG="$HOME/.bashrc"
        fi
        ;;
    *)
        echo -e "${RED}Unsupported shell: $SHELL${NC}"
        echo "Please add the following to your shell configuration:"
        echo "  export PATH=\"\$PATH:$INSTALL_DIR/bin\""
        ;;
esac

if [ -n "$SHELL_CONFIG" ]; then
    if ! grep -q "$INSTALL_DIR/bin" "$SHELL_CONFIG"; then
        echo "export PATH=\"\$PATH:$INSTALL_DIR/bin\"" >> "$SHELL_CONFIG"
        echo -e "${GREEN}Added LyangLang to PATH in $SHELL_CONFIG${NC}"
    fi
fi

# Create executable symlink
if [ "$os" = "windows" ]; then
    cp "$INSTALL_DIR/lyangpiler.exe" "$INSTALL_DIR/bin/"
    cp "$INSTALL_DIR/lyangpiler.cmd" "$INSTALL_DIR/bin/"
else
    chmod +x "$INSTALL_DIR/lyangpiler"
    ln -sf "$INSTALL_DIR/lyangpiler" "$INSTALL_DIR/bin/lyangpiler"
fi

echo -e "${GREEN}LyangLang has been successfully installed!${NC}"
echo -e "You can now use ${BLUE}lyangpiler${NC} to run LyangLang programs."
echo
echo "To get started:"
echo "  lyangpiler new my-project"
echo "  cd my-project"
echo "  lyangpiler run main.nbh --vm"
echo
echo -e "${BLUE}For more information, visit: https://github.com/konseptt/LyangLang${NC}"