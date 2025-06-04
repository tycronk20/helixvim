#!/bin/bash
set -euo pipefail

# Script to run tests for HelixVim

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
REPO_ROOT="$( cd "$SCRIPT_DIR/.." && pwd )"

echo "==> Running HelixVim tests..."

# Check if we need to bootstrap first
if [ ! -f "$REPO_ROOT/Cargo.toml" ]; then
    echo "==> Running bootstrap script first..."
    "$SCRIPT_DIR/bootstrap.sh"
fi

# Check if just is installed
if command -v just &> /dev/null; then
    # Use just to run tests
    cd "$REPO_ROOT"
    just test
else
    # Fall back to cargo
    cd "$REPO_ROOT"
    cargo test
fi

# Run specific test categories if requested
if [ "${1:-}" = "ui" ]; then
    echo "==> Running UI tests only..."
    cd "$REPO_ROOT"
    if command -v just &> /dev/null; then
        just test-ui
    else
        cargo test --test ui
    fi
fi

echo "==> All tests passed!"