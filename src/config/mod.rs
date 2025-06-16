//! Configuration module for MacHelix
//!
//! This module handles loading and saving configuration.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// MacHelix configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// macOS integration options
    #[serde(default)]
    pub macos: MacOSConfig,
    
    /// Window options
    #[serde(default)]
    pub window: WindowConfig,
    
    /// Font options
    #[serde(default)]
    pub font: FontConfig,
    
    /// Rendering options
    #[serde(default)]
    pub rendering: RenderingConfig,
    
    /// UI options
    #[serde(default)]
    pub ui: UIConfig,
    
    /// Performance options
    #[serde(default)]
    pub performance: PerformanceConfig,
}

/// macOS integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacOSConfig {
    /// Use native menus
    #[serde(default = "default_true")]
    pub native_menus: bool,
    
    /// Use native tabs
    #[serde(default = "default_true")]
    pub native_tabs: bool,
    
    /// Use native fullscreen
    #[serde(default = "default_true")]
    pub native_fullscreen: bool,
    
    /// Use system clipboard
    #[serde(default = "default_true")]
    pub use_system_clipboard: bool,
    
    /// Enable services menu
    #[serde(default = "default_true")]
    pub services_menu: bool,
}

/// Window configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    /// Remember window size
    #[serde(default = "default_true")]
    pub remember_size: bool,
    
    /// Remember window position
    #[serde(default = "default_true")]
    pub remember_position: bool,
    
    /// Default window width
    #[serde(default = "default_window_width")]
    pub default_width: u32,
    
    /// Default window height
    #[serde(default = "default_window_height")]
    pub default_height: u32,
}

/// Font configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontConfig {
    /// Font family
    #[serde(default = "default_font_family")]
    pub family: String,
    
    /// Font size
    #[serde(default = "default_font_size")]
    pub size: u32,
    
    /// Line height multiplier
    #[serde(default = "default_line_height")]
    pub line_height: f32,
    
    /// Use font ligatures
    #[serde(default = "default_true")]
    pub use_ligatures: bool,
}

/// Rendering configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderingConfig {
    /// Use GPU for rendering
    #[serde(default = "default_true")]
    pub use_gpu: bool,
    
    /// Enable vertical sync
    #[serde(default = "default_true")]
    pub vsync: bool,
    
    /// Text antialiasing level (0-3)
    #[serde(default = "default_antialiasing")]
    pub antialiasing: u8,
}

/// UI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIConfig {
    /// Tab style
    #[serde(default = "default_tab_style")]
    pub tab_style: String,
    
    /// Show toolbar
    #[serde(default = "default_true")]
    pub toolbar: bool,
    
    /// Toolbar items
    #[serde(default = "default_toolbar_items")]
    pub toolbar_items: Vec<String>,
}

/// Performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Frame rate when idle
    #[serde(default = "default_idle_fps")]
    pub idle_fps: u32,
    
    /// Frame rate when active
    #[serde(default = "default_active_fps")]
    pub active_fps: u32,
    
    /// Maximum background jobs
    #[serde(default = "default_background_jobs")]
    pub background_jobs: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            macos: MacOSConfig::default(),
            window: WindowConfig::default(),
            font: FontConfig::default(),
            rendering: RenderingConfig::default(),
            ui: UIConfig::default(),
            performance: PerformanceConfig::default(),
        }
    }
}

impl Default for MacOSConfig {
    fn default() -> Self {
        Self {
            native_menus: true,
            native_tabs: true,
            native_fullscreen: true,
            use_system_clipboard: true,
            services_menu: true,
        }
    }
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            remember_size: true,
            remember_position: true,
            default_width: 1024,
            default_height: 768,
        }
    }
}

impl Default for FontConfig {
    fn default() -> Self {
        Self {
            family: "SF Mono".to_string(),
            size: 13,
            line_height: 1.2,
            use_ligatures: true,
        }
    }
}

impl Default for RenderingConfig {
    fn default() -> Self {
        Self {
            use_gpu: true,
            vsync: true,
            antialiasing: 2,
        }
    }
}

impl Default for UIConfig {
    fn default() -> Self {
        Self {
            tab_style: "native".to_string(),
            toolbar: true,
            toolbar_items: vec!["new".to_string(), "open".to_string(), "save".to_string()],
        }
    }
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            idle_fps: 30,
            active_fps: 120,
            background_jobs: 4,
        }
    }
}

// Default value functions
fn default_true() -> bool { true }
fn default_window_width() -> u32 { 1024 }
fn default_window_height() -> u32 { 768 }
fn default_font_family() -> String { "SF Mono".to_string() }
fn default_font_size() -> u32 { 13 }
fn default_line_height() -> f32 { 1.2 }
fn default_antialiasing() -> u8 { 2 }
fn default_tab_style() -> String { "native".to_string() }
fn default_toolbar_items() -> Vec<String> {
    vec!["new".to_string(), "open".to_string(), "save".to_string()]
}
fn default_idle_fps() -> u32 { 30 }
fn default_active_fps() -> u32 { 120 }
fn default_background_jobs() -> u32 { 4 }

impl Config {
    /// Load configuration from file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config = toml::from_str(&content)?;
        Ok(config)
    }
    
    /// Save configuration to file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
    
    /// Get default configuration path
    pub fn default_path() -> Result<std::path::PathBuf> {
        let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
        Ok(home.join(".config").join("machelix").join("config.toml"))
    }
}