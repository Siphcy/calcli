#!/bin/sh
set -e

# Detect OS and architecture
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Linux*)
        if [ "$ARCH" = "x86_64" ]; then
            BINARY="calcli-linux-x86_64"
        else
            echo "Unsupported architecture: $ARCH"
            exit 1
        fi
        ;;
    Darwin*)
        if [ "$ARCH" = "arm64" ]; then
            BINARY="calcli-macos-aarch64"
        else
            BINARY="calcli-macos-x86_64"
        fi
        ;;
    *)
        echo "Unsupported OS: $OS"
        exit 1
        ;;
esac

# Get latest release
echo "Fetching latest release..."
VERSION=$(curl -s https://api.github.com/repos/Siphcy/calcli/releases/latest | grep '"tag_name":' | sed -E 's/.*"v([^"]+)".*/\1/')

if [ -z "$VERSION" ]; then
    echo "Failed to fetch latest version"
    exit 1
fi

URL="https://github.com/Siphcy/calcli/releases/download/v${VERSION}/${BINARY}"

echo "Downloading calcli v${VERSION}..."
curl -L "$URL" -o /tmp/calcli

chmod +x /tmp/calcli

# Try to install to /usr/local/bin, fallback to ~/.local/bin
if [ -w /usr/local/bin ]; then
    mv /tmp/calcli /usr/local/bin/calcli
    echo "calcli installed to /usr/local/bin/calcli"
elif sudo -n true 2>/dev/null; then
    sudo mv /tmp/calcli /usr/local/bin/calcli
    echo "calcli installed to /usr/local/bin/calcli"
else
    mkdir -p ~/.local/bin
    mv /tmp/calcli ~/.local/bin/calcli
    echo "calcli installed to ~/.local/bin/calcli"
    echo ""
    echo "Make sure ~/.local/bin is in your PATH:"
    echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
fi

echo ""
echo "Installation successful!"
echo "Run 'calcli' to start"
