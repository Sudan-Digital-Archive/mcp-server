#!/bin/sh
set -e

# Detect OS and Architecture
OS="$(uname -s)"
ARCH="$(uname -m)"
BIN_NAME="sda-mcp-server"

echo "Detecting system..."

if [ "$OS" = "Linux" ]; then
    if [ "$ARCH" = "x86_64" ]; then
        ASSET_SUFFIX="linux-x86_64"
    else
        echo "Error: Unsupported architecture for Linux: $ARCH"
        exit 1
    fi
elif [ "$OS" = "Darwin" ]; then
    if [ "$ARCH" = "arm64" ]; then
        ASSET_SUFFIX="macos-arm64"
    else
        echo "Error: Unsupported architecture for macOS: $ARCH. Only Apple Silicon (arm64) is currently supported."
        exit 1
    fi
else
    echo "Error: Unsupported operating system: $OS"
    exit 1
fi

ASSET_NAME="${BIN_NAME}-${ASSET_SUFFIX}.tar.gz"
DOWNLOAD_URL="https://github.com/Sudan-Digital-Archive/mcp-server/releases/latest/download/${ASSET_NAME}"

echo "Downloading latest release (${ASSET_NAME})..."
# Use a temporary directory
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

if curl -fsSL "$DOWNLOAD_URL" -o "$TMP_DIR/$ASSET_NAME"; then
    echo "Download successful."
else
    echo "Error: Failed to download from $DOWNLOAD_URL"
    exit 1
fi

echo "Extracting..."
tar -xzf "$TMP_DIR/$ASSET_NAME" -C "$TMP_DIR"

# Prefer user-local installation to avoid sudo
INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"
TARGET_PATH="$INSTALL_DIR/$BIN_NAME"

echo "Installing '$BIN_NAME' to '$INSTALL_DIR'..."

mv "$TMP_DIR/$BIN_NAME" "$TARGET_PATH"
chmod +x "$TARGET_PATH"

echo "✓ Installation complete!"

# Check if INSTALL_DIR is in PATH
case ":$PATH:" in
    *":$INSTALL_DIR:"*) ;;
    *) echo "WARNING: '$INSTALL_DIR' is not in your PATH."
       echo "You may need to add it to your shell configuration (e.g., ~/.zshrc or ~/.bashrc):"
       echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
       ;;
esac

echo "Run '$BIN_NAME --help' to get started."
