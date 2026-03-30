#!/bin/bash
# Generate shell completions for rok
# Usage: ./scripts/completions.sh [bash|zsh|fish]

set -e

SHELL=${1:-bash}
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_DIR"

case "$SHELL" in
    bash)
        cargo run --quiet -- completions bash > "$HOME/.local/share/bash-completion/completions/rok" 2>/dev/null || \
        cargo run --quiet -- completions bash
        echo "Bash completions generated to ~/.local/share/bash-completion/completions/rok"
        ;;
    zsh)
        cargo run --quiet -- completions zsh > "$HOME/.zfunc/_rok" 2>/dev/null || \
        cargo run --quiet -- completions zsh
        echo "Zsh completions generated to ~/.zfunc/_rok"
        ;;
    fish)
        cargo run --quiet -- completions fish > "$HOME/.config/fish/completions/rok.fish" 2>/dev/null || \
        cargo run --quiet -- completions fish
        echo "Fish completions generated to ~/.config/fish/completions/rok.fish"
        ;;
    *)
        echo "Usage: $0 [bash|zsh|fish]"
        exit 1
        ;;
esac
