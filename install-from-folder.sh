#!/usr/bin/env bash
# Offline setup: run from the extracted "lyangpiler" folder (next to the lyangpiler binary).
#   bash install-from-folder.sh
set -euo pipefail
DIR="$(cd "$(dirname "$0")" && pwd)"
EXE="$DIR/lyangpiler"
if [[ ! -f "$EXE" ]]; then
  echo "Error: no 'lyangpiler' binary found in $DIR" >&2
  echo "Extract the release archive, cd into the lyangpiler folder, then run this script again." >&2
  exit 1
fi
INSTALL_ROOT="${LYANGLANG_HOME:-$HOME/.lyangpiler}"
BIN_DIR="$INSTALL_ROOT/bin"
mkdir -p "$BIN_DIR"
cp -f "$EXE" "$BIN_DIR/lyangpiler"
chmod +x "$BIN_DIR/lyangpiler"

ENABLE_SH="$INSTALL_ROOT/enable.sh"
mkdir -p "$INSTALL_ROOT"
printf '%s\n' "# Lyangpiler — add to PATH for this terminal session" "export PATH=\"\$PATH:$BIN_DIR\"" >"$ENABLE_SH"

echo "Installed: $BIN_DIR/lyangpiler"
echo ""
echo "Use it in this terminal now:"
echo "  source $ENABLE_SH"
echo "  lyangpiler --help"
echo ""
echo "To add PATH permanently, append the same export line to ~/.zshrc or ~/.bashrc"
