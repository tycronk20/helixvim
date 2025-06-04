# HelixVim Justfile

default:
    @just --list

# Run the application
run:
    cargo run

# Build in release mode
build:
    cargo build --release

# Create macOS app bundle
bundle:
    cargo bundle --release

# Run tests
test:
    cargo test

# Run UI tests only
test-ui:
    cargo test --test ui

# Check code formatting
fmt-check:
    cargo fmt --check

# Format code
fmt:
    cargo fmt

# Run clippy
clippy:
    cargo clippy --all-targets -- -D warnings

# Create DMG for distribution
dmg: bundle
    create-dmg \
        --volname "HelixVim" \
        --volicon "assets/helixvim.icns" \
        --window-pos 200 120 \
        --window-size 800 400 \
        --icon-size 100 \
        --icon "HelixVim.app" 200 190 \
        --hide-extension "HelixVim.app" \
        --app-drop-link 600 185 \
        "HelixVim.dmg" \
        "target/release/bundle/osx/HelixVim.app"

# Watch for changes and run
watch:
    cargo watch -x run

# Clean build artifacts
clean:
    cargo clean