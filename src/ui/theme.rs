//! Theme handling for MacHelix
//!
//! This module handles loading and applying themes.

use anyhow::Result;
use helix_core::theme::{Theme, Loader};

/// Load a theme by name
pub fn load_theme(name: &str) -> Result<Theme> {
    let loader = Loader::new();
    let theme = loader.load(name)?;
    Ok(theme)
}

/// Get the default theme
pub fn default_theme() -> Result<Theme> {
    load_theme("default")
}

/// Apply a theme to the UI
pub fn apply_theme(theme: &Theme) -> Result<()> {
    // TODO: Apply theme to UI components
    Ok(())
}

/// Get a list of available themes
pub fn available_themes() -> Result<Vec<String>> {
    let loader = Loader::new();
    let themes = loader.themes()?;
    Ok(themes)
}