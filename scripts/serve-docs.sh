#!/bin/bash
# Serve the documentation site locally
# Usage: ./scripts/serve-docs.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
DOCS_DIR="$PROJECT_DIR/docs"

echo "📖 Serving rok documentation at http://localhost:8080"
echo "Press Ctrl+C to stop"
echo ""

cd "$DOCS_DIR"

# Try different methods to serve files
if command -v python3 &> /dev/null; then
    python3 -m http.server 8080
elif command -v python &> /dev/null; then
    python -m http.server 8080
elif command -v php &> /dev/null; then
    php -S localhost:8080
else
    echo "No HTTP server found. Install Python or PHP."
    exit 1
fi
