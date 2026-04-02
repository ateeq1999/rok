#!/usr/bin/env bash
# Development checks for the rok workspace.
#
# Usage:
#   ./scripts/dev.sh [command]
#
# Commands:
#   gates    Run all acceptance gates (same checks as publish requires)
#   fmt      Check formatting across the whole workspace
#   fix      Auto-fix formatting
#   clippy   Run clippy with -D warnings
#   test     Run all workspace tests
#   doc      Build all docs
#   build    Release build of the full workspace
#   clean    Remove build artefacts
#   watch    Re-run tests on every file change (requires cargo-watch)
#
# Default (no command): runs `gates`.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(dirname "$SCRIPT_DIR")"
cd "$ROOT"

COMMAND="${1:-gates}"

step() { echo ""; echo ">> $*"; }
ok()   { echo "   [ok] $*"; }
fail() { echo ""; echo "[FAIL] $*" >&2; exit 1; }

case "$COMMAND" in

    gates)
        echo "Running workspace acceptance gates..."

        step "Formatting"
        cargo fmt --all -- --check || fail "Run: ./scripts/dev.sh fix"
        ok "cargo fmt clean"

        step "Lints"
        cargo clippy --workspace -- -D warnings
        ok "cargo clippy clean"

        step "Tests"
        cargo test --workspace
        ok "All tests pass"

        step "Documentation"
        cargo doc --workspace --no-deps
        ok "cargo doc clean"

        echo ""
        echo "All gates passed."
        ;;

    fmt|format)
        step "Checking formatting"
        cargo fmt --all -- --check
        ok "cargo fmt clean"
        ;;

    fix)
        step "Auto-fixing formatting"
        cargo fmt --all
        ok "Done"
        ;;

    clippy)
        step "Running clippy"
        cargo clippy --workspace -- -D warnings
        ok "cargo clippy clean"
        ;;

    test)
        step "Running workspace tests"
        cargo test --workspace
        ok "All tests pass"
        ;;

    doc)
        step "Building documentation"
        cargo doc --workspace --no-deps
        ok "cargo doc clean"
        ;;

    build)
        step "Release build"
        cargo build --workspace --release
        ok "Build complete"
        ;;

    clean)
        step "Cleaning build artefacts"
        cargo clean
        ok "Done"
        ;;

    watch)
        if ! command -v cargo-watch &>/dev/null; then
            fail "cargo-watch not found. Install with: cargo install cargo-watch"
        fi
        step "Watching for changes (cargo test --workspace)"
        cargo watch -x "test --workspace"
        ;;

    *)
        echo "Unknown command: $COMMAND" >&2
        echo ""
        echo "Usage: $0 [gates|fmt|fix|clippy|test|doc|build|clean|watch]"
        exit 1
        ;;
esac
