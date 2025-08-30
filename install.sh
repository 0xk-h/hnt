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
BINARY_NAME="hnt-${OS}-Hello"

URL="https://github.com/kishore399/hnt/releases/latest/download/$BINARY_NAME"

HTTP_STATUS=$(curl -o /dev/null --silent --head --write-out "%{http_code}" --location "$URL")

if [ "$HTTP_STATUS" -ne 200 ]; then
    echo "❌ The binary for your OS/architecture is not published yet."
    echo "Please download the source manually from GitHub and compile it:"
    echo "https://github.com/kishore399/hnt"
    exit 1
fi

echo "Downloading $BINARY_NAME from $URL ..."
curl -L "$URL" -o /tmp/hnt

#-------------------------------------------------
# making it executable and moving to bin path
#-------------------------------------------------
chmod +x /tmp/hnt
sudo mv /tmp/hnt "$BIN_PATH/hnt"

echo "✅ Installation complete! Run 'hnt guess' to verify."
echo "Installed at: $BIN_PATH/hnt"