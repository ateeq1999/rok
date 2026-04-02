#!/usr/bin/env bash
# Publish rok crates to crates.io.
#
# Usage:
#   ./scripts/publish.sh [--dry-run] [<crate-name>]
#
#   --dry-run      Run all acceptance gates and verify packaging
#                  with `cargo publish --dry-run`. Nothing is uploaded.
#   <crate-name>   Publish a single crate by name (e.g. rok-utils).
#                  If omitted, all crates are published in dependency order.
#
# Acceptance gates (run once before any crate is published):
#   1. Clean working tree — no uncommitted changes
#   2. Formatting   — cargo fmt --all -- --check
#   3. Lints        — cargo clippy --workspace -- -D warnings
#   4. Tests        — cargo test --workspace
#   5. Docs         — cargo doc --workspace --no-deps
#
# Each published crate is tagged: <crate>-v<version>
#
# Examples:
#   ./scripts/publish.sh --dry-run
#   ./scripts/publish.sh rok-utils
#   ./scripts/publish.sh

set -euo pipefail

# ── Publish order: dependencies before dependents ────────────────────────────

PUBLISH_ORDER=(
    "rok-utils"
    "rok-config"
    "rok-orm-core"
    "rok-orm-macros"
    "rok-orm"
    "rok-migrate"
    "rok-http"
    "rok-auth"
    "rok-lint"
    "rok-test"
    "rok-generate"
    "rok-deploy"
    "rok-gen-model"
    "rok-gen-api"
    "rok-docs"
)

# ── Argument parsing ───────────────────────────────────────────────────────────

DRY_RUN=false
TARGET_CRATE=""

for arg in "$@"; do
    case "$arg" in
        --dry-run) DRY_RUN=true ;;
        -*) echo "Unknown flag: $arg" >&2; exit 1 ;;
        *)  TARGET_CRATE="$arg" ;;
    esac
done

# ── Helpers ───────────────────────────────────────────────────────────────────

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(dirname "$SCRIPT_DIR")"
cd "$ROOT"

step() { echo ""; echo ">> $*"; }
ok()   { echo "   [ok] $*"; }
fail() { echo ""; echo "[FAIL] $*" >&2; exit 1; }

# Return the newest published version of a crate from crates.io, or empty.
published_version() {
    curl -sf "https://crates.io/api/v1/crates/${1}" \
        -H "User-Agent: rok-publish-script (github.com/ateeq1999/rok)" 2>/dev/null \
        | grep -o '"newest_version":"[^"]*"' \
        | head -1 \
        | sed 's/"newest_version":"//;s/"//'
}

# Return the local version of a crate from cargo metadata.
local_version() {
    cargo metadata --no-deps --format-version 1 2>/dev/null \
        | grep -o "\"name\":\"${1}\",\"version\":\"[^\"]*\"" \
        | grep -o '"version":"[^"]*"' \
        | sed 's/"version":"//;s/"//'
}

# ── Acceptance gates ──────────────────────────────────────────────────────────

step "Gate 1/5 — clean working tree"
if [ -n "$(git status --porcelain)" ]; then
    fail "Uncommitted changes detected. Commit or stash before publishing."
fi
ok "Working tree is clean"

step "Gate 2/5 — formatting"
if ! cargo fmt --all -- --check; then
    fail "Formatting check failed. Run: cargo fmt --all"
fi
ok "cargo fmt clean"

step "Gate 3/5 — lints"
if ! cargo clippy --workspace -- -D warnings 2>&1; then
    fail "Clippy reported warnings. Fix them before publishing."
fi
ok "cargo clippy clean"

step "Gate 4/5 — tests"
if ! cargo test --workspace 2>&1; then
    fail "Tests failed. Fix them before publishing."
fi
ok "All tests pass"

step "Gate 5/5 — documentation"
if ! cargo doc --workspace --no-deps 2>&1; then
    fail "Documentation failed to build. Fix rustdoc errors before publishing."
fi
ok "cargo doc clean"

echo ""
echo "All acceptance gates passed."

# ── Build the list of crates to publish ───────────────────────────────────────

if [ -n "$TARGET_CRATE" ]; then
    found=false
    for c in "${PUBLISH_ORDER[@]}"; do
        [ "$c" = "$TARGET_CRATE" ] && found=true && break
    done
    $found || fail "Unknown crate: $TARGET_CRATE. Known: ${PUBLISH_ORDER[*]}"
    CRATES_TO_PUBLISH=("$TARGET_CRATE")
else
    CRATES_TO_PUBLISH=("${PUBLISH_ORDER[@]}")
fi

# ── Publish loop ──────────────────────────────────────────────────────────────

PUBLISH_FLAGS=""
$DRY_RUN && PUBLISH_FLAGS="--dry-run --no-verify"

published_count=0
skipped_count=0

for crate in "${CRATES_TO_PUBLISH[@]}"; do
    local_ver=$(local_version "$crate")
    if [ -z "$local_ver" ]; then
        echo "  [warn] Could not determine version for $crate — skipping"
        continue
    fi

    # Skip crates already at this version on crates.io (unless dry-run).
    if ! $DRY_RUN; then
        remote_ver=$(published_version "$crate")
        if [ "$remote_ver" = "$local_ver" ]; then
            echo "  [skip] $crate v$local_ver (already published)"
            skipped_count=$((skipped_count + 1))
            continue
        fi
    fi

    step "Publishing $crate v$local_ver${DRY_RUN:+ (dry run)}"
    if $DRY_RUN; then
        # Full dry-run works for leaf crates. For crates whose workspace deps
        # haven't been published yet, fall back to manifest+file-list check.
        if ! cargo publish -p "$crate" $PUBLISH_FLAGS 2>/dev/null; then
            if ! cargo package -p "$crate" --list --no-verify 2>/dev/null; then
                fail "cargo publish failed for $crate v$local_ver"
            fi
            echo "   (manifest check only — workspace deps not on crates.io yet)"
        fi
    else
        if ! cargo publish -p "$crate" $PUBLISH_FLAGS; then
            fail "cargo publish failed for $crate v$local_ver"
        fi
    fi
    ok "$crate v$local_ver published"

    if ! $DRY_RUN; then
        tag="${crate}-v${local_ver}"
        if ! git rev-parse "$tag" &>/dev/null 2>&1; then
            git tag "$tag"
            git push origin "$tag"
            ok "Tagged $tag and pushed"
        fi
        # crates.io enforces a brief rate-limit between consecutive publishes.
        sleep 5
    fi

    published_count=$((published_count + 1))
done

# ── Summary ───────────────────────────────────────────────────────────────────

echo ""
echo "Done."
if $DRY_RUN; then
    echo "  Dry run — nothing uploaded."
else
    echo "  Published : $published_count crate(s)"
    echo "  Skipped   : $skipped_count crate(s) already at current version"
fi
