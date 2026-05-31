#!/usr/bin/env bash
# LyangLang / Lyangpiler — install prebuilt binary to user directory and add to PATH.
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/Konseptt/LyangLang/main/install.sh | bash
#   bash install.sh
#
# Override repo:   LYANGLANG_REPO=owner/LyangLang bash install.sh

set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[0;33m'
NC='\033[0m'

REPO="${LYANGLANG_REPO:-Konseptt/LyangLang}"
BASE="https://github.com/${REPO}/releases/latest/download"

echo -e "${BLUE}Installing Lyangpiler (LyangLang)…${NC}"

OS_RAW="$(uname -s)"
ARCH_RAW="$(uname -m)"

case "$OS_RAW" in
  Linux*)  os=linux ;;
  Darwin*) os=macos ;;
  *)
    echo -e "${RED}This shell installer supports Linux and macOS only.${NC}" >&2
    echo "On Windows, run in PowerShell:" >&2
    echo "  iwr -useb https://raw.githubusercontent.com/${REPO}/main/install.ps1 | iex" >&2
    echo "Or use Docker / Rust (see README)." >&2
    exit 1
    ;;
esac

case "$ARCH_RAW" in
  x86_64|amd64)  arch=x86_64 ;;
  aarch64|arm64) arch=aarch64 ;;
  *)
    echo -e "${RED}Unsupported CPU architecture: ${ARCH_RAW}${NC}" >&2
    exit 1
    ;;
esac

# Legacy CI artifact names (older releases)
URL_LEGACY_LINUX_AMD64="${BASE}/lyangpiler-linux-amd64.tar.gz"
# Older CI used a single "macos-amd64" archive name for macOS builds.
URL_LEGACY_MAC="${BASE}/lyangpiler-macos-amd64.tar.gz"

INSTALL_ROOT="${LYANGLANG_HOME:-$HOME/.lyangpiler}"
BIN_DIR="${INSTALL_ROOT}/bin"
TMP_DIR="$(mktemp -d)"
cleanup() { rm -rf "$TMP_DIR"; }
trap cleanup EXIT

download_to() {
  local dest="$1" url="$2"
  rm -f "$dest"
  if command -v curl >/dev/null 2>&1; then
    curl -fsSL "$url" -o "$dest" || return 1
  elif command -v wget >/dev/null 2>&1; then
    # wget must fail on 404; still reject tiny bodies (error pages) if a mirror misbehaves
    wget -qO "$dest" "$url" || return 1
  else
    echo -e "${RED}Need curl or wget to download the binary.${NC}" >&2
    exit 1
  fi
  local sz
  sz=$(wc -c <"$dest" | tr -d ' ')
  if (( sz < 2048 )); then
    rm -f "$dest"
    return 1
  fi
  return 0
}

# Reject HTML error pages and truncated downloads (GitHub 404 is tiny and not gzip).
is_valid_release_tarball() {
  local f="$1"
  [[ -f "$f" ]] || return 1
  local sz
  sz=$(wc -c <"$f" | tr -d ' ')
  (( sz >= 4096 )) || return 1
  command -v gzip >/dev/null 2>&1 && gzip -t "$f" 2>/dev/null || return 1
  return 0
}

# Ordered list of release asset URLs (current naming first, then legacy CI names).
candidate_urls() {
  case "${os}-${arch}" in
    linux-x86_64)
      echo "${BASE}/lyangpiler-linux-x86_64.tar.gz"
      echo "${URL_LEGACY_LINUX_AMD64}"
      ;;
    linux-aarch64)
      echo "${BASE}/lyangpiler-linux-aarch64.tar.gz"
      ;;
    macos-aarch64)
      echo "${BASE}/lyangpiler-macos-aarch64.tar.gz"
      echo "${URL_LEGACY_MAC}"
      ;;
    macos-x86_64)
      echo "${BASE}/lyangpiler-macos-x86_64.tar.gz"
      echo "${URL_LEGACY_MAC}"
      ;;
    *)
      echo "${BASE}/lyangpiler-${os}-${arch}.tar.gz"
      ;;
  esac
}

ARCHIVE="${TMP_DIR}/dist.tar.gz"
rm -f "$ARCHIVE"
downloaded=0
while IFS= read -r try_url; do
  [[ -z "$try_url" ]] && continue
  echo -e "${BLUE}Downloading…${NC} ${try_url}"
  if download_to "$ARCHIVE" "$try_url" 2>/dev/null && is_valid_release_tarball "$ARCHIVE"; then
    downloaded=1
    break
  fi
  rm -f "$ARCHIVE"
  echo -e "${YELLOW}Not a valid archive (missing release or wrong URL). Trying next…${NC}"
done < <(candidate_urls)

if [[ "$downloaded" -ne 1 ]]; then
  echo -e "${RED}Could not download a valid Lyangpiler release for ${os}-${arch}.${NC}" >&2
  echo "Open the releases page and pick the .tar.gz for your Mac or Linux CPU:" >&2
  echo "  https://github.com/${REPO}/releases/latest" >&2
  echo "" >&2
  echo "Apple Silicon Macs need: lyangpiler-macos-aarch64.tar.gz" >&2
  echo "Intel Macs need: lyangpiler-macos-x86_64.tar.gz" >&2
  echo "" >&2
  echo "Or install with Rust:" >&2
  echo "  cargo install --git https://github.com/${REPO}.git --locked" >&2
  exit 1
fi

mkdir -p "$TMP_DIR/extract"
tar -xzf "$ARCHIVE" -C "$TMP_DIR/extract"

EXE=""
if [[ -f "$TMP_DIR/extract/lyangpiler/lyangpiler" ]]; then
  EXE="$TMP_DIR/extract/lyangpiler/lyangpiler"
elif [[ -f "$TMP_DIR/extract/lyangpiler" && -x "$TMP_DIR/extract/lyangpiler" ]]; then
  EXE="$TMP_DIR/extract/lyangpiler"
else
  EXE="$(find "$TMP_DIR/extract" -type f -name lyangpiler 2>/dev/null | head -1 || true)"
fi

if [[ -z "$EXE" || ! -f "$EXE" ]]; then
  echo -e "${RED}Could not find lyangpiler binary inside the archive.${NC}" >&2
  find "$TMP_DIR/extract" -maxdepth 3 -type f -ls >&2 || true
  exit 1
fi

mkdir -p "$BIN_DIR"
cp -f "$EXE" "$BIN_DIR/lyangpiler"
chmod +x "$BIN_DIR/lyangpiler"

mkdir -p "$INSTALL_ROOT"
ENABLE_SH="$INSTALL_ROOT/enable.sh"
printf '%s\n' "# Lyangpiler (LyangLang) — use in this terminal without reopening it" "export PATH=\"\$PATH:${BIN_DIR}\"" >"$ENABLE_SH"

path_line="export PATH=\"\$PATH:${BIN_DIR}\""
fish_path_line="fish_add_path ${BIN_DIR}"

append_path_snippet() {
  local file="$1"
  local line="$2"
  [[ -z "$file" ]] && return 0
  if [[ -f "$file" ]] && grep -Fq "$BIN_DIR" "$file" 2>/dev/null; then
    echo -e "${GREEN}PATH already references ${BIN_DIR} in ${file}${NC}"
    return 0
  fi
  echo "" >>"$file"
  echo "# Lyangpiler (LyangLang)" >>"$file"
  echo "$line" >>"$file"
  echo -e "${GREEN}Updated ${file}${NC}"
}

SHELL_NAME="$(basename "${SHELL:-bash}")"
case "$SHELL_NAME" in
  zsh)  append_path_snippet "${ZDOTDIR:-$HOME}/.zshrc" "$path_line" ;;
  bash) append_path_snippet "$HOME/.bashrc" "$path_line"
        [[ "$(uname -s)" == "Darwin" ]] && append_path_snippet "$HOME/.bash_profile" "$path_line"
        ;;
  fish)
    fish_cfg="$HOME/.config/fish/config.fish"
    mkdir -p "$(dirname "$fish_cfg")"
    if [[ -f "$fish_cfg" ]] && grep -Fq "$BIN_DIR" "$fish_cfg" 2>/dev/null; then
      echo -e "${GREEN}PATH already references ${BIN_DIR} in ${fish_cfg}${NC}"
    else
      mkdir -p "$(dirname "$fish_cfg")"
      echo "" >>"$fish_cfg"
      echo "# Lyangpiler (LyangLang)" >>"$fish_cfg"
      if command -v fish >/dev/null 2>&1 && fish -c 'functions -q fish_add_path' 2>/dev/null; then
        echo "$fish_path_line" >>"$fish_cfg"
      else
        echo "set -gx PATH ${BIN_DIR} \$PATH" >>"$fish_cfg"
      fi
      echo -e "${GREEN}Updated ${fish_cfg}${NC}"
    fi
    ;;
  *)
    echo -e "${YELLOW}Add this line to your shell config:${NC}"
    echo "  $path_line"
    ;;
esac

echo ""
echo -e "${GREEN}Installed to:${NC} ${BIN_DIR}/lyangpiler"
echo ""
echo -e "${YELLOW}Use Lyangpiler in this terminal right now (one line):${NC}"
echo "  source ${ENABLE_SH} && lyangpiler --help"
echo ""
echo "Or open a new terminal, then:"
echo "  lyangpiler --help"
echo "  lyangpiler new demo && cd demo && lyangpiler run main.nbh --vm"
echo ""
echo -e "${BLUE}https://github.com/${REPO}${NC}"
