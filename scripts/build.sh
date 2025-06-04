#!/bin/bash
set -euo pipefail

# Script to build HelixVim

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
REPO_ROOT="$( cd "$SCRIPT_DIR/.." && pwd )"

echo "==> Building HelixVim..."

# Check if we need to bootstrap first
if [ ! -f "$REPO_ROOT/Cargo.toml" ]; then
    echo "==> Running bootstrap script first..."
    "$SCRIPT_DIR/bootstrap.sh"
fi

# Check if just is installed
if command -v just &> /dev/null; then
    # Use just to build
    cd "$REPO_ROOT"
    just build
else
    # Fall back to cargo
    cd "$REPO_ROOT"
    cargo build --release
fi

# Create app bundle
echo "==> Creating app bundle..."
if command -v cargo-bundle &> /dev/null; then
    cd "$REPO_ROOT"
    cargo bundle --release
    
    # Create a symlink to the app bundle for convenience
    if [ -d "$REPO_ROOT/target/release/bundle/osx/HelixVim.app" ]; then
        echo "==> Creating symlink to app bundle..."
        ln -sf "$REPO_ROOT/target/release/bundle/osx/HelixVim.app" "$REPO_ROOT/HelixVim.app"
        echo "==> App bundle available at: $REPO_ROOT/HelixVim.app"
    fi
else
    echo "==> Warning: cargo-bundle not installed, skipping app bundle creation"
    echo "==> Install with: cargo install cargo-bundle"
    echo "==> Binary available at: $REPO_ROOT/target/release/helixvim"
fi

echo "==> Build complete!"