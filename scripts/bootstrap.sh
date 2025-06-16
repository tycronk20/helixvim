#!/bin/bash
set -euo pipefail

# Script to bootstrap the MacHelix development environment

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
REPO_ROOT="$( cd "$SCRIPT_DIR/.." && pwd )"

echo "==> Setting up MacHelix development environment..."

# Check for Homebrew
if ! command -v brew &> /dev/null; then
    echo "Homebrew not found. Please install Homebrew first:"
    echo "https://brew.sh/"
    exit 1
fi

# Install dependencies from Brewfile
echo "==> Installing dependencies from Brewfile..."
brew bundle --file="$REPO_ROOT/Brewfile"

# Initialize Rust if needed
if ! command -v rustup &> /dev/null; then
    echo "==> Installing Rust toolchain..."
    rustup-init -y --default-toolchain stable
    source "$HOME/.cargo/env"
fi

# Update Rust
echo "==> Updating Rust toolchain..."
rustup update stable

# Add required components
echo "==> Installing Rust components..."
rustup component add rustfmt clippy

# Install cargo tools if not already installed
if ! command -v cargo-bundle &> /dev/null; then
    echo "==> Installing cargo-bundle..."
    cargo install cargo-bundle
fi

if ! command -v cargo-watch &> /dev/null; then
    echo "==> Installing cargo-watch..."
    cargo install cargo-watch
fi

if ! command -v just &> /dev/null; then
    echo "==> Installing just..."
    cargo install just
fi

# Create project structure if it doesn't exist
if [ ! -d "$REPO_ROOT/src" ]; then
    echo "==> Creating project structure..."
    mkdir -p "$REPO_ROOT/src/app"
    mkdir -p "$REPO_ROOT/src/editor"
    mkdir -p "$REPO_ROOT/src/config"
    mkdir -p "$REPO_ROOT/src/ui"
    mkdir -p "$REPO_ROOT/src/bridge"
    mkdir -p "$REPO_ROOT/assets"
fi

# Create Justfile if it doesn't exist
if [ ! -f "$REPO_ROOT/Justfile" ]; then
    echo "==> Creating Justfile..."
    cat > "$REPO_ROOT/Justfile" << EOF
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
    create-dmg \\
        --volname "MacHelix" \\
        --volicon "assets/machelix.icns" \\
        --window-pos 200 120 \\
        --window-size 800 400 \\
        --icon-size 100 \\
        --icon "MacHelix.app" 200 190 \\
        --hide-extension "MacHelix.app" \\
        --app-drop-link 600 185 \\
        "MacHelix.dmg" \\
        "target/release/bundle/osx/MacHelix.app"

# Watch for changes and run
watch:
    cargo watch -x run

# Clean build artifacts
clean:
    cargo clean
EOF
fi

# Create initial Cargo.toml if it doesn't exist
if [ ! -f "$REPO_ROOT/Cargo.toml" ]; then
    echo "==> Creating Cargo.toml..."
    cat > "$REPO_ROOT/Cargo.toml" << EOF
[package]
name = "machelix"
version = "0.1.0"
edition = "2021"
authors = ["MacHelix Contributors"]
description = "Native macOS application for the Helix editor"
repository = "https://github.com/tycronk20/machelix"
license = "MIT"

[dependencies]
# Helix core
helix-core = "0.6.0"
helix-view = "0.6.0"
helix-term = "0.6.0"

# macOS integration
cocoa = "0.24.1"
objc = "0.2.7"
core-foundation = "0.9.3"
core-graphics = "0.22.3"

# UI and rendering
wgpu = "0.15"
winit = "0.28"
raw-window-handle = "0.5"

# Utilities
anyhow = "1.0"
log = "0.4"
env_logger = "0.10"
serde = { version = "1.0", features = ["derive"] }
toml = "0.7"
clap = { version = "4.2", features = ["derive"] }

[build-dependencies]
cargo-bundle = "0.6.0"

[package.metadata.bundle]
name = "MacHelix"
identifier = "com.machelix.app"
icon = ["assets/machelix.icns"]
version = "0.1.0"
copyright = "Copyright (c) 2025 MacHelix Contributors"
category = "public.app-category.developer-tools"
short_description = "Native macOS application for the Helix editor"
long_description = """
MacHelix is a native macOS application that provides the Helix editing experience
with deep macOS integration, built entirely in Rust.
"""
EOF
fi

# Create main.rs if it doesn't exist
if [ ! -f "$REPO_ROOT/src/main.rs" ]; then
    echo "==> Creating main.rs..."
    mkdir -p "$REPO_ROOT/src"
    cat > "$REPO_ROOT/src/main.rs" << EOF
//! MacHelix - Native macOS application for the Helix editor
//!
//! This is the main entry point for the MacHelix application.

mod app;
mod editor;
mod config;
mod ui;
mod bridge;

fn main() -> anyhow::Result<()> {
    // Initialize logging
    env_logger::init();
    
    // Start the application
    app::run()
}
EOF
fi

# Create app module if it doesn't exist
if [ ! -f "$REPO_ROOT/src/app/mod.rs" ]; then
    echo "==> Creating app module..."
    mkdir -p "$REPO_ROOT/src/app"
    cat > "$REPO_ROOT/src/app/mod.rs" << EOF
//! Application module for MacHelix
//!
//! This module handles the macOS application lifecycle and window management.

pub mod menu;
pub mod preferences;
pub mod services;

use anyhow::Result;

/// Run the application
pub fn run() -> Result<()> {
    println!("MacHelix starting up...");
    
    // TODO: Initialize window and event loop
    
    Ok(())
}
EOF
fi

echo "==> Bootstrap complete!"
echo "You can now build MacHelix with: just build"
echo "Or run it with: just run"