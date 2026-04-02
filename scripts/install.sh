#!/usr/bin/env bash
# Build rok-cli from source and install it globally.
#
# Usage:
#   ./scripts/install.sh
#
# Alternatively, install from crates.io:
#   cargo install rok-cli

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(dirname "$SCRIPT_DIR")"
cd "$ROOT"

echo "Building rok-cli (release)..."
cargo build -p rok-cli --release

BINARY="$ROOT/target/release/rok"
[ -f "${BINARY}.exe" ] && BINARY="${BINARY}.exe"

if [ ! -f "$BINARY" ]; then
    echo "Error: binary not found at $BINARY" >&2
    exit 1
fi

OS="$(uname -s)"
case "$OS" in
    Linux*|Darwin*)
        if [ -w /usr/local/bin ]; then
            DEST="/usr/local/bin/rok"
        else
            DEST="$HOME/.local/bin/rok"
            mkdir -p "$(dirname "$DEST")"
        fi
        cp "$BINARY" "$DEST"
        chmod +x "$DEST"
        ;;
    MINGW*|MSYS*|CYGWIN*)
        DEST="$LOCALAPPDATA/rok/bin/rok.exe"
        mkdir -p "$(dirname "$DEST")"
        cp "$BINARY" "$DEST"
        ;;
    *)
        echo "Unsupported OS: $OS" >&2
        exit 1
        ;;
esac

echo "Installed: $DEST"
echo ""
echo "Verify: rok --version"
