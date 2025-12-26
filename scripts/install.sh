#!/usr/bin/env sh
set -e

# ============================================================
# Aargal Installer
# ============================================================

REPO="urwithajit9/aargal"
INSTALL_DIR="/usr/local/bin"
BINARY_NAME="aargal"
EXAMPLE_CONFIG_NAME="aargal.example.toml"

VERSION="${AARGAL_VERSION:-latest}"

ENABLE_SERVICE=0
NGINX_GROUP=""

# ------------------------------------------------------------
# Parse arguments
# ------------------------------------------------------------
for arg in "$@"; do
  case "$arg" in
    --enable-service)
      ENABLE_SERVICE=1
      ;;
    --nginx-group=*)
      NGINX_GROUP="${arg#*=}"
      ;;
    *)
      ;;
  esac
done

# ------------------------------------------------------------
# OS & Arch checks
# ------------------------------------------------------------
OS="$(uname -s)"
ARCH="$(uname -m)"

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

# ------------------------------------------------------------
# Dependencies
# ------------------------------------------------------------
if ! command -v curl >/dev/null 2>&1 && ! command -v wget >/dev/null 2>&1; then
  echo "❌ curl or wget is required"
  exit 1
fi

if [ "$(id -u)" -ne 0 ]; then
  SUDO="sudo"
else
  SUDO=""
fi

# ------------------------------------------------------------
# Resolve version
# ------------------------------------------------------------
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

# ------------------------------------------------------------
# Download
# ------------------------------------------------------------
echo "⬇️  Downloading $URL"

if command -v curl >/dev/null 2>&1; then
  curl -fL "$URL" -o "$TMP_DIR/$TARBALL"
else
  wget -O "$TMP_DIR/$TARBALL" "$URL"
fi

# ------------------------------------------------------------
# Extract
# ------------------------------------------------------------
echo "📦 Extracting"
tar -xzf "$TMP_DIR/$TARBALL" -C "$TMP_DIR"

if [ ! -f "$TMP_DIR/$BINARY_NAME" ]; then
  echo "❌ Binary not found in archive"
  exit 1
fi

chmod +x "$TMP_DIR/$BINARY_NAME"

# ------------------------------------------------------------
# Install binary
# ------------------------------------------------------------
echo "🚀 Installing binary to $INSTALL_DIR"
$SUDO mv "$TMP_DIR/$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"

# ------------------------------------------------------------
# Enable systemd service (optional)
# ------------------------------------------------------------
if [ "$ENABLE_SERVICE" -eq 1 ]; then
  echo "⚙️  Enabling system service"

  # Detect nginx group
  if [ -z "$NGINX_GROUP" ]; then
    if [ -f /var/log/nginx/access.log ]; then
      NGINX_GROUP="$(stat -c %G /var/log/nginx/access.log)"
    else
      NGINX_GROUP="www-data"
    fi
  fi

  echo "🔎 Using nginx log group: $NGINX_GROUP"

  # Create system user
  if ! id aargal >/dev/null 2>&1; then
    echo "👤 Creating system user 'aargal'"
    $SUDO useradd \
      --system \
      --no-create-home \
      --shell /usr/sbin/nologin \
      aargal
  fi

  $SUDO usermod -aG "$NGINX_GROUP" aargal

  # Config directory
  echo "📁 Setting up /etc/aargal"
  $SUDO mkdir -p /etc/aargal

  if [ ! -f /etc/aargal/aargal.toml ]; then
    if [ -f "$TMP_DIR/$EXAMPLE_CONFIG_NAME" ]; then
      echo "📄 Installing default configuration"
      $SUDO cp "$TMP_DIR/$EXAMPLE_CONFIG_NAME" /etc/aargal/aargal.toml
    else
      echo "❌ Example config not found in archive"
      exit 1
    fi
  else
    echo "ℹ️  Existing config preserved"
  fi

  $SUDO chown root:aargal /etc/aargal/aargal.toml
  $SUDO chmod 0640 /etc/aargal/aargal.toml

  # Install service
  echo "🧩 Installing systemd service"
  $SUDO cp packaging/systemd/aargal.service /etc/systemd/system/aargal.service

  $SUDO systemctl daemon-reexec
  $SUDO systemctl daemon-reload
  $SUDO systemctl enable aargal
  $SUDO systemctl start aargal

  echo "✅ Aargal service started"
  systemctl status aargal --no-pager || true
fi

# ------------------------------------------------------------
# Verify
# ------------------------------------------------------------
echo ""
echo "✅ Installation complete"
"$INSTALL_DIR/$BINARY_NAME" --version || true

echo ""
echo "Next steps:"
echo "  sudo nano /etc/aargal/aargal.toml"
echo "  aargal doctor"
echo "  journalctl -u aargal -f"
