#!/usr/bin/env sh
set -e

# -----------------------------
# Configuration
# -----------------------------
REPO="urwithajit9/aargal"
INSTALL_DIR="/usr/local/bin"
BINARY_NAME="aargal"

# Default version (latest)
VERSION="${AARGAL_VERSION:-latest}"

ARCH="$(uname -m)"
OS="$(uname -s)"

# -----------------------------
# Preconditions
# -----------------------------
if [ "$OS" != "Linux" ]; then
  echo "❌ Aargal is supported only on Linux"
  exit 1
fi

case "$ARCH" in
  x86_64|amd64)
    TARGET_ARCH="x86_64"
    ;;
  *)
    echo "❌ Unsupported architecture: $ARCH"
    exit 1
    ;;
esac

if ! command -v curl >/dev/null 2>&1 && ! command -v wget >/dev/null 2>&1; then
  echo "❌ curl or wget is required"
  exit 1
fi

if [ "$(id -u)" -ne 0 ]; then
  echo "ℹ️  Installing to $INSTALL_DIR requires sudo"
  SUDO="sudo"
else
  SUDO=""
fi

# -----------------------------
# Resolve version
# -----------------------------
if [ "$VERSION" = "latest" ]; then
  VERSION="$(curl -fsSL https://api.github.com/repos/$REPO/releases/latest \
    | grep '"tag_name"' \
    | cut -d '"' -f 4)"
fi

echo "➡️  Installing Aargal $VERSION for Linux $TARGET_ARCH"

TARBALL="aargal-${VERSION}-linux-${TARGET_ARCH}.tar.gz"
URL="https://github.com/$REPO/releases/download/$VERSION/$TARBALL"

TMP_DIR="$(mktemp -d)"
cleanup() { rm -rf "$TMP_DIR"; }
trap cleanup EXIT

# -----------------------------
# Download
# -----------------------------
echo "⬇️  Downloading $URL"

if command -v curl >/dev/null 2>&1; then
  curl -fL "$URL" -o "$TMP_DIR/$TARBALL"
else
  wget -O "$TMP_DIR/$TARBALL" "$URL"
fi

# -----------------------------
# Extract
# -----------------------------
echo "📦 Extracting"
tar -xzf "$TMP_DIR/$TARBALL" -C "$TMP_DIR"

if [ ! -f "$TMP_DIR/$BINARY_NAME" ]; then
  echo "❌ Binary not found in archive"
  exit 1
fi

chmod +x "$TMP_DIR/$BINARY_NAME"

# -----------------------------
# Install
# -----------------------------
echo "🚀 Installing to $INSTALL_DIR"
$SUDO mv "$TMP_DIR/$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"

# -----------------------------
# Verify
# -----------------------------
echo "✅ Installation complete"
"$INSTALL_DIR/$BINARY_NAME" --version || true

echo ""
echo "Next steps:"
echo "  aargal --config /path/to/aargal.toml"
