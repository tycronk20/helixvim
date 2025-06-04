#!/bin/bash
set -euo pipefail

# UI smoke test for HelixVim
# This script launches the app headless and checks basic functionality

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
REPO_ROOT="$( cd "$SCRIPT_DIR/.." && pwd )"

echo "==> Running UI smoke test..."

# Check if app bundle exists
APP_BUNDLE="$REPO_ROOT/HelixVim.app"
if [ ! -d "$APP_BUNDLE" ]; then
    # Check if we have a release binary instead
    BINARY="$REPO_ROOT/target/release/helixvim"
    if [ ! -f "$BINARY" ]; then
        echo "Neither app bundle nor binary found"
        echo "Run ./scripts/build.sh first"
        exit 1
    fi
    
    # Use binary for testing
    USE_BINARY=true
else
    USE_BINARY=false
fi

# Create a temporary file for testing
TEMP_FILE=$(mktemp)
echo "This is a test file for HelixVim" > "$TEMP_FILE"

# Set up a headless environment if needed
if [ -z "${DISPLAY:-}" ]; then
    if command -v xvfb-run &> /dev/null; then
        RUNNER="xvfb-run -a"
    else
        echo "Warning: No display available and xvfb-run not found."
        echo "Running in headless mode may not work."
        RUNNER=""
    fi
else
    RUNNER=""
fi

# Run the app with the test file
echo "==> Launching HelixVim with test file..."
if [ "$USE_BINARY" = true ]; then
    $RUNNER "$BINARY" --headless "$TEMP_FILE" &
else
    $RUNNER "$APP_BUNDLE/Contents/MacOS/HelixVim" --headless "$TEMP_FILE" &
fi
PID=$!

# Give it a moment to start up
sleep 2

# Check if process is still running
if ! kill -0 $PID 2>/dev/null; then
    echo "Error: HelixVim crashed on startup"
    exit 1
fi

# Clean up
kill $PID
rm "$TEMP_FILE"

echo "==> UI smoke test passed!"