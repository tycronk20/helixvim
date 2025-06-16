//! Window management for MacHelix
//!
//! This module handles window creation and management.

use anyhow::Result;
use winit::window::{Window, WindowBuilder};
use winit::event_loop::EventLoop;
use crate::config::WindowConfig;

/// Create a new window
pub fn create_window(config: &WindowConfig) -> Result<(Window, EventLoop<()>)> {
    let event_loop = EventLoop::new();
    
    let window = WindowBuilder::new()
        .with_title("MacHelix")
        .with_inner_size(winit::dpi::LogicalSize::new(
            config.default_width,
            config.default_height,
        ))
        .build(&event_loop)?;
    
    Ok((window, event_loop))
}

/// Set window title
pub fn set_window_title(window: &Window, title: &str) {
    window.set_title(title);
}

/// Set window size
pub fn set_window_size(window: &Window, width: u32, height: u32) {
    window.set_inner_size(winit::dpi::LogicalSize::new(width, height));
}

/// Toggle fullscreen
pub fn toggle_fullscreen(window: &Window) {
    // TODO: Toggle fullscreen
}

/// Save window state
pub fn save_window_state(window: &Window) -> Result<()> {
    // TODO: Save window state
    Ok(())
}

/// Restore window state
pub fn restore_window_state(window: &Window) -> Result<()> {
    // TODO: Restore window state
    Ok(())
}