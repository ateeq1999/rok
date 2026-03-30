#!/bin/bash
# Development helper - build, test, lint, and check
# Usage: ./scripts/dev.sh [command]

set -e

COMMAND=${1:-"all"}

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_DIR"

echo "🔧 Running development checks for rok..."
echo ""

case "$COMMAND" in
    all)
        echo "📝 Running all checks..."
        echo ""
        echo "1️⃣  Checking formatting..."
        cargo fmt -- --check
        echo "   ✅ Formatting OK"
        echo ""
        
        echo "2️⃣  Building..."
        cargo build --release
        echo "   ✅ Build OK"
        echo ""
        
        echo "3️⃣  Running tests..."
        cargo test
        echo "   ✅ Tests OK"
        echo ""
        
        echo "4️⃣  Running clippy..."
        cargo clippy -- -D warnings
        echo "   ✅ Clippy OK"
        echo ""
        
        echo "✅ All checks passed!"
        ;;
        
    fmt|format)
        echo "📝 Checking formatting..."
        cargo fmt -- --check
        echo "✅ Formatting OK"
        ;;
        
    build)
        echo "🔨 Building..."
        cargo build --release
        echo "✅ Build OK"
        ;;
        
    test)
        echo "🧪 Running tests..."
        cargo test
        echo "✅ Tests OK"
        ;;
        
    clippy)
        echo "🔍 Running clippy..."
        cargo clippy -- -D warnings
        echo "✅ Clippy OK"
        ;;
        
    clean)
        echo "🧹 Cleaning..."
        cargo clean
        echo "✅ Clean OK"
        ;;
        
    run)
        echo "🏃 Running..."
        shift
        cargo run --release -- "$@"
        ;;
        
    *)
        echo "❌ Unknown command: $COMMAND"
        echo ""
        echo "Usage: $0 [command]"
        echo ""
        echo "Commands:"
        echo "  all      - Run all checks (default)"
        echo "  fmt      - Check formatting"
        echo "  build    - Build release"
        echo "  test     - Run tests"
        echo "  clippy   - Run clippy lints"
        echo "  clean    - Clean build artifacts"
        echo "  run      - Run the application"
        exit 1
        ;;
esac
