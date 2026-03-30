#!/bin/bash
# Install rok locally and make it accessible globally
# Usage: ./scripts/install.sh

set -e

echo "📦 Installing rok..."

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

# Build release version
echo "🔨 Building release..."
cd "$PROJECT_DIR"
cargo build --release

# Determine binary path
BINARY_PATH="$PROJECT_DIR/target/release/rok.exe"

if [ ! -f "$BINARY_PATH" ]; then
    # Try without .exe on Unix
    BINARY_PATH="$PROJECT_DIR/target/release/rok"
fi

if [ ! -f "$BINARY_PATH" ]; then
    echo "❌ Error: Binary not found at $BINARY_PATH"
    exit 1
fi

# Detect OS and set up global access
case "$(uname -s)" in
    Linux*)
        # Try to install to ~/.local/bin or /usr/local/bin
        if [ -w /usr/local/bin ]; then
            INSTALL_DIR="/usr/local/bin"
        elif [ -w "$HOME/.local/bin" ] || mkdir -p "$HOME/.local/bin" 2>/dev/null; then
            INSTALL_DIR="$HOME/.local/bin"
        else
            echo "❌ Error: Cannot write to /usr/local/bin or ~/.local/bin"
            echo "   Please add ~/.local/bin to your PATH manually"
            exit 1
        fi
        
        cp "$BINARY_PATH" "$INSTALL_DIR/rok"
        chmod +x "$INSTALL_DIR/rok"
        echo "✅ Installed to $INSTALL_DIR/rok"
        ;;
    Darwin*)
        # macOS
        if [ -w /usr/local/bin ]; then
            INSTALL_DIR="/usr/local/bin"
        else
            INSTALL_DIR="$HOME/.local/bin"
            mkdir -p "$INSTALL_DIR"
        fi
        
        cp "$BINARY_PATH" "$INSTALL_DIR/rok"
        chmod +x "$INSTALL_DIR/rok"
        echo "✅ Installed to $INSTALL_DIR/rok"
        ;;
    MINGW*|MSYS*|CYGWIN*)
        # Windows - copy to a persistent location
        WIN_INSTALL_DIR="$LOCALAPPDATA/rok/bin"
        mkdir -p "$WIN_INSTALL_DIR"
        cp "$BINARY_PATH" "$WIN_INSTALL_DIR/rok.exe"
        
        # Add to PATH if not already present
        if [[ ":$PATH:" != *":$WIN_INSTALL_DIR:"* ]]; then
            echo "Adding $WIN_INSTALL_DIR to PATH..."
            [ -f ~/.bashrc ] && echo "export PATH=\"\$PATH:$WIN_INSTALL_DIR\"" >> ~/.bashrc
            [ -f ~/.zshrc ] && echo "export PATH=\"\$PATH:$WIN_INSTALL_DIR\"" >> ~/.zshrc
            [ -f ~/.profile ] && echo "export PATH=\"\$PATH:$WIN_INSTALL_DIR\"" >> ~/.profile
        fi
        echo "✅ Installed to $WIN_INSTALL_DIR/rok.exe"
        echo "   (Make sure $WIN_INSTALL_DIR is in your PATH)"
        ;;
    *)
        echo "❌ Error: Unsupported OS: $(uname -s)"
        exit 1
        ;;
esac

echo ""
echo "✅ Installation complete!"
echo ""
echo "💡 Verify with:"
echo "   rok --version"
echo "   rok --help"
