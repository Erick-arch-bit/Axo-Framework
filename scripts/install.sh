#!/usr/bin/env bash
set -euo pipefail

REPO="Erick-arch-bit/Axo-Framework"
VERSION="${1:-latest}"
BOLD="\033[1m"
GREEN="\033[0;32m"
CYAN="\033[0;36m"
NC="\033[0m"

echo -e "${BOLD}Axo Framework Installer${NC}"
echo "================================"

# ── Detect platform ──
ARCH="$(uname -m)"
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
case "$OS" in
  linux)  TARGET="${ARCH}-unknown-linux-gnu" ;;
  darwin) TARGET="${ARCH}-apple-darwin" ;;
  mingw*|msys*|cygwin*) TARGET="${ARCH}-pc-windows-msvc" ;;
  *)
    echo "Unsupported OS: $OS — falling back to cargo install"
    CARGO_FALLBACK=1
    ;;
esac

# ── Prebuilt binary ──
install_prebuilt() {
  local url="https://github.com/${REPO}/releases"
  if [ "$VERSION" = "latest" ]; then
    url="${url}/latest/download/axo-cli-${TARGET}.tar.gz"
  else
    url="${url}/download/v${VERSION}/axo-cli-${TARGET}.tar.gz"
  fi
  echo "Downloading ${url} ..."
  curl -fsSL "$url" | tar xz -C /usr/local/bin 2>/dev/null || {
    echo "Prebuilt binary not available for ${TARGET}. Falling back to cargo install."
    return 1
  }
  echo -e "${GREEN}✓${NC} Installed axo-cli to /usr/local/bin"
}

# ── Cargo install ──
install_cargo() {
  if ! command -v cargo &>/dev/null; then
    echo "Rust not found. Install it first:"
    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
  fi
  local pkg="axo-cli"
  if [ "$VERSION" != "latest" ]; then
    pkg="axo-cli@${VERSION}"
  fi
  echo "Building from source (cargo install ${pkg}) ..."
  cargo install "$pkg"
  echo -e "${GREEN}✓${NC} Installed axo-cli via cargo"
}

# ── Main ──
if [ -z "${CARGO_FALLBACK:-}" ]; then
  install_prebuilt || install_cargo
else
  install_cargo
fi

echo ""
echo -e "${GREEN}${BOLD}Axo Framework installed!${NC}"
echo ""
echo "Quickstart:"
echo "  ${CYAN}axo-cli init my-app${NC}"
echo "  ${CYAN}cd my-app${NC}"
echo "  ${CYAN}axo-cli dev${NC}"
echo ""
echo "Or use the shorthand:"
echo "  ${CYAN}axo dev${NC}"
echo ""
echo "Full docs: https://github.com/${REPO}"
