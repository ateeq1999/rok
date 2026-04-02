#!/usr/bin/env bash
# Serve the generated markdown documentation locally.
#
# Usage:
#   ./scripts/serve-docs.sh [--port <port>] [--dir <dir>]
#
# Generates docs first if the output directory does not exist, then serves
# them using the rok-docs binary from the workspace.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(dirname "$SCRIPT_DIR")"
cd "$ROOT"

PORT=8080
DIR="$ROOT/docs/content/crates"

while [[ $# -gt 0 ]]; do
    case "$1" in
        --port) PORT="$2"; shift 2 ;;
        --dir)  DIR="$2";  shift 2 ;;
        *) echo "Unknown argument: $1" >&2; exit 1 ;;
    esac
done

# Generate docs if the directory does not exist yet.
if [ ! -d "$DIR" ]; then
    echo "Generating docs into $DIR..."
    cargo run -p rok-docs --release -- generate --output "$DIR" --workspace "$ROOT/Cargo.toml"
fi

echo "Serving $DIR at http://localhost:$PORT"
echo "Press Ctrl-C to stop."
cargo run -p rok-docs --release -- serve --dir "$DIR" --port "$PORT"
