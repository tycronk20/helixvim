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