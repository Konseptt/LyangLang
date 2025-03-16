#!/bin/bash
# LyangLang Installer
# This script installs LyangLang on Unix-like systems

set -e

# Constants
VERSION="0.1.0"
INSTALL_DIR="$HOME/.lyangpiler"
BIN_DIR="$HOME/.local/bin"

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
echo -e "${YELLOW}LyangLang Programming Language Installer v${VERSION}${NC}"
echo

# Check if curl or wget is available
if command -v curl >/dev/null 2>&1; then
    DOWNLOAD_CMD="curl -L"
elif command -v wget >/dev/null 2>&1; then
    DOWNLOAD_CMD="wget -O-"
else
    echo -e "${RED}Error: Neither curl nor wget is available. Please install one of them.${NC}"
    exit 1
fi

# Check if Rust is installed
if ! command -v rustc >/dev/null 2>&1 || ! command -v cargo >/dev/null 2>&1; then
    echo -e "${YELLOW}Rust is not installed. It's required to build LyangLang.${NC}"
    echo -e "Would you like to install Rust now? (y/n)"
    read -r install_rust
    
    if [[ "$install_rust" =~ ^[Yy]$ ]]; then
        echo -e "${BLUE}Installing Rust...${NC}"
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
    else
        echo -e "${RED}Rust is required to build LyangLang. Installation aborted.${NC}"
        exit 1
    fi
fi

# Create installation directories
mkdir -p "$INSTALL_DIR"
mkdir -p "$BIN_DIR"

echo -e "${BLUE}Downloading LyangLang...${NC}"

# In a real scenario, we would download from a remote repository
# For now, we'll clone from GitHub or copy from current directory
if [ -d ".git" ]; then
    echo -e "${GREEN}Found existing repository. Building from current directory.${NC}"
    # Build the project
    cargo build --release
    
    # Copy binaries and scripts
    cp target/release/lyangpiler "$INSTALL_DIR/"
    cp oimugchal "$INSTALL_DIR/"
    chmod +x "$INSTALL_DIR/oimugchal"
else
    # This would be the remote download branch
    echo -e "${GREEN}Cloning from GitHub repository...${NC}"
    temp_dir=$(mktemp -d)
    git clone https://github.com/konseptt/LyangLang "$temp_dir"
    
    cd "$temp_dir"
    cargo build --release
    
    # Copy binaries and scripts
    cp target/release/lyangpiler "$INSTALL_DIR/"
    cp oimugchal "$INSTALL_DIR/"
    chmod +x "$INSTALL_DIR/oimugchal"
    
    cd -
    rm -rf "$temp_dir"
fi

# Create symlinks in bin directory
ln -sf "$INSTALL_DIR/oimugchal" "$BIN_DIR/oimugchal"

# Add bin directory to PATH if not already there
if [[ ":$PATH:" != *":$BIN_DIR:"* ]]; then
    echo -e "${YELLOW}Adding $BIN_DIR to your PATH...${NC}"
    
    # Determine shell configuration file
    SHELL_CONFIG=""
    if [ -n "$ZSH_VERSION" ]; then
        SHELL_CONFIG="$HOME/.zshrc"
    elif [ -n "$BASH_VERSION" ]; then
        if [ "$(uname)" == "Darwin" ]; then
            SHELL_CONFIG="$HOME/.bash_profile"
        else
            SHELL_CONFIG="$HOME/.bashrc"
        fi
    fi
    
    if [ -n "$SHELL_CONFIG" ]; then
        echo "export PATH=\"$BIN_DIR:\$PATH\"" >> "$SHELL_CONFIG"
        echo -e "${GREEN}Added $BIN_DIR to PATH in $SHELL_CONFIG${NC}"
        echo -e "${YELLOW}Please run 'source $SHELL_CONFIG' or start a new terminal to use LyangLang.${NC}"
    else
        echo -e "${YELLOW}Could not determine shell configuration file.${NC}"
        echo -e "${YELLOW}Please manually add $BIN_DIR to your PATH.${NC}"
    fi
fi

echo -e "${GREEN}${BOLD}LyangLang has been successfully installed!${NC}"
echo -e "You can now use ${BOLD}oimugchal${NC} to create and run LyangLang programs."
echo
echo -e "Try it out:"
echo -e "  ${BOLD}oimugchal new my-project${NC}"
echo -e "  ${BOLD}cd my-project${NC}"
echo -e "  ${BOLD}oimugchal run${NC}"
echo
echo -e "For more information, run: ${BOLD}oimugchal help${NC}"