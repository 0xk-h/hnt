#!/usr/bin/env bash
set -e

#-------------------------------------------------
# Detect the OS and architecture
#-------------------------------------------------
OS=$(uname | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

if [[ "$ARCH" == "x86_64" ]]; then
    ARCH="x86_64"
elif [[ "$ARCH" == "arm64" || "$ARCH" == "aarch64" ]]; then
    ARCH="arm64"
else
    echo "Unsupported architecture: $ARCH"
    exit 1
fi

#-------------------------------------------------
# Determining the installation path
#-------------------------------------------------
if [[ "$OS" == "darwin" && -d "/opt/homebrew/bin" ]]; then
        BIN_PATH="/opt/homebrew/bin"
else
    BIN_PATH="/usr/local/bin"
fi

#-------------------------------------------------
# installing the binary
#-------------------------------------------------
BINARY_NAME="hnt-${OS}-${ARCH}"

URL="https://github.com/kishore399/hnt/releases/latest/download/$BINARY_NAME"

echo "Downloading $BINARY_NAME from $URL ..."
curl -L "$URL" -o /tmp/hnt

#-------------------------------------------------
# making it executable and moving to bin path
#-------------------------------------------------
chmod +x /tmp/hnt
sudo mv /tmp/hnt "$BIN_PATH/hnt"

echo "✅ Installation complete! Run 'hnt guess' to verify."
echo "Installed at: $BIN_PATH/hnt"