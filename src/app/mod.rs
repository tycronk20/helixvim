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