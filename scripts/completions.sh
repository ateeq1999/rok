#!/usr/bin/env bash
# Generate shell completions for the rok CLI.
#
# Usage:
#   ./scripts/completions.sh [bash|zsh|fish]

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(dirname "$SCRIPT_DIR")"
cd "$ROOT"

SHELL_NAME="${1:-bash}"

case "$SHELL_NAME" in
    bash)
        DEST="$HOME/.local/share/bash-completion/completions/rok"
        mkdir -p "$(dirname "$DEST")"
        cargo run -p rok-cli --quiet -- completions bash > "$DEST"
        echo "Bash completions written to $DEST"
        ;;
    zsh)
        DEST="$HOME/.zfunc/_rok"
        mkdir -p "$(dirname "$DEST")"
        cargo run -p rok-cli --quiet -- completions zsh > "$DEST"
        echo "Zsh completions written to $DEST"
        echo "Add to ~/.zshrc if not already present: fpath=(~/.zfunc \$fpath)"
        ;;
    fish)
        DEST="$HOME/.config/fish/completions/rok.fish"
        mkdir -p "$(dirname "$DEST")"
        cargo run -p rok-cli --quiet -- completions fish > "$DEST"
        echo "Fish completions written to $DEST"
        ;;
    *)
        echo "Usage: $0 [bash|zsh|fish]" >&2
        exit 1
        ;;
esac
