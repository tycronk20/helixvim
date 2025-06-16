# MacHelix Justfile

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
        --volname "MacHelix" \
        --volicon "assets/machelix.icns" \
        --window-pos 200 120 \
        --window-size 800 400 \
        --icon-size 100 \
        --icon "MacHelix.app" 200 190 \
        --hide-extension "MacHelix.app" \
        --app-drop-link 600 185 \
        "MacHelix.dmg" \
        "target/release/bundle/osx/MacHelix.app"

# Watch for changes and run
watch:
    cargo watch -x run

# Clean build artifacts
clean:
    cargo clean