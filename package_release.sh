#!/bin/bash
# Create a distributable package of Lyangpiler
# This script creates an archive with everything needed to run Lyangpiler on another system

echo "Creating Lyangpiler release package..."

# Set directory variables
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
RELEASE_DIR="$SCRIPT_DIR/target/release"
PACKAGE_DIR="$SCRIPT_DIR/lyangpiler-package"
VERSION="0.1.0"

# First, build the release version if it doesn't exist
if [ ! -f "$RELEASE_DIR/lyangpiler" ]; then
    echo "Building release version..."
    cargo build --release
    if [ $? -ne 0 ]; then
        echo "Failed to build release version!"
        exit 1
    fi
fi

# Create and clean package directory
if [ -d "$PACKAGE_DIR" ]; then
    rm -rf "$PACKAGE_DIR"
fi
mkdir -p "$PACKAGE_DIR"

# Copy executable and required files
echo "Copying files to package..."
cp "$RELEASE_DIR/lyangpiler" "$PACKAGE_DIR/"
cp "$SCRIPT_DIR/example.nbh" "$PACKAGE_DIR/"
cp "$SCRIPT_DIR/README.md" "$PACKAGE_DIR/"
cp "$SCRIPT_DIR/lyangpiler" "$PACKAGE_DIR/"
chmod +x "$PACKAGE_DIR/lyangpiler"

# Create a Unix installer script
echo "Creating Unix installer..."
cat > "$PACKAGE_DIR/install.sh" << 'EOF'
#!/bin/bash
# Simple installer for Lyangpiler
# This adds the current directory to PATH

echo "Installing Lyangpiler..."

# Get the current directory
INSTALL_DIR="$(cd "$(dirname "$0")" && pwd)"

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
    # Check if already in PATH
    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        echo "Adding to PATH in $SHELL_CONFIG..."
        echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$SHELL_CONFIG"
        echo "Please run 'source $SHELL_CONFIG' to update your PATH."
    else
        echo "Already in PATH."
    fi
else
    echo "Could not determine shell configuration file."
    echo "Please manually add the following to your shell configuration:"
    echo "  export PATH=\"\$PATH:$INSTALL_DIR\""
fi

echo "Lyangpiler has been installed!"
echo "You can now run 'lyangpiler example.nbh' from any directory after updating your PATH."
EOF

chmod +x "$PACKAGE_DIR/install.sh"

# Create the archive
echo "Creating archive..."
cd "$SCRIPT_DIR"
ARCHIVE_NAME="lyangpiler-v${VERSION}-$(uname -s | tr '[:upper:]' '[:lower:]')-$(uname -m).tar.gz"
tar -czf "$ARCHIVE_NAME" -C "$(dirname "$PACKAGE_DIR")" "$(basename "$PACKAGE_DIR")"

echo ""
echo "Release package created: $ARCHIVE_NAME"
echo ""
echo "This package can be extracted on any $(uname -s) system and run without needing Rust installed."
echo "To install, extract the archive and run install.sh to add to PATH (optional)."
echo ""