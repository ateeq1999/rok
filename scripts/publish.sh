#!/bin/bash
# Publish rok to crates.io and GitHub
# Usage: ./scripts/publish.sh [version]

set -e

VERSION=${1:-""}

echo "🚀 Publishing rok..."

# Check for uncommitted changes
if [ -n "$(git status --porcelain)" ]; then
    echo "❌ Error: You have uncommitted changes. Commit or stash them first."
    exit 1
fi

# Get current version from Cargo.toml
CURRENT_VERSION=$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')

if [ -z "$VERSION" ]; then
    VERSION=$CURRENT_VERSION
    echo "📦 Publishing version: $VERSION"
else
    echo "📦 Publishing version: $VERSION (bumping from $CURRENT_VERSION)"
    
    # Update version in Cargo.toml
    sed -i "s/^version = \"$CURRENT_VERSION\"/version = \"$VERSION\"/" Cargo.toml
    
    # Commit version bump
    git add Cargo.toml
    git commit -m "chore: Bump version to $VERSION"
fi

# Build to check for errors
echo "🔨 Building..."
cargo build --release

# Run tests
echo "🧪 Running tests..."
cargo test

# Run clippy for linting
echo "🔍 Running clippy..."
cargo clippy -- -D warnings

# Format check
echo "📝 Checking formatting..."
cargo fmt -- --check

# Publish to crates.io
echo "📤 Publishing to crates.io..."
cargo publish

# Push to GitHub
echo "📨 Pushing to GitHub..."
git push
git push --tags

echo "✅ Successfully published rok v$VERSION!"
echo ""
echo "📋 Summary:"
echo "  - crates.io: https://crates.io/crates/rok-cli"
echo "  - GitHub: https://github.com/ateeq1999/rok"
echo ""
echo "💡 To install:"
echo "  cargo install rok-cli"
