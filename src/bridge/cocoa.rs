//! Cocoa integration for MacHelix
//!
//! This module provides low-level integration with Cocoa APIs.

use anyhow::Result;
use cocoa::base::{selector, nil, NO, YES};
use cocoa::foundation::{NSString, NSRect, NSPoint, NSSize};
use cocoa::appkit::{NSApp, NSApplication, NSWindow, NSMenu, NSMenuItem};
use objc::{msg_send, sel, sel_impl};

/// Initialize Cocoa application
pub fn init_cocoa_app() -> Result<()> {
    unsafe {
        let app = NSApp();
        let _: () = msg_send![app, setActivationPolicy:0]; // NSApplicationActivationPolicyRegular
    }
    Ok(())
}

/// Create a Cocoa menu
pub fn create_menu(title: &str) -> *mut objc::runtime::Object {
    unsafe {
        let title = NSString::alloc(nil).init_str(title);
        let menu = NSMenu::alloc(nil).initWithTitle_(title);
        menu
    }
}

/// Add a menu item
pub fn add_menu_item(
    menu: *mut objc::runtime::Object,
    title: &str,
    action: Option<objc::runtime::Sel>,
    key_equivalent: &str,
) -> *mut objc::runtime::Object {
    unsafe {
        let title = NSString::alloc(nil).init_str(title);
        let key = NSString::alloc(nil).init_str(key_equivalent);
        let item = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            title,
            action.unwrap_or(selector(":")),
            key,
        );
        let _: () = msg_send![menu, addItem:item];
        item
    }
}

/// Set the application menu
pub fn set_app_menu(menu: *mut objc::runtime::Object) {
    unsafe {
        let app = NSApp();
        let _: () = msg_send![app, setMainMenu:menu];
    }
}

/// Create a Cocoa window
pub fn create_window(
    title: &str,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) -> *mut objc::runtime::Object {
    unsafe {
        let title = NSString::alloc(nil).init_str(title);
        let window = NSWindow::alloc(nil).initWithContentRect_styleMask_backing_defer_(
            NSRect::new(NSPoint::new(x, y), NSSize::new(width, height)),
            0x0F, // NSWindowStyleMaskTitled | NSWindowStyleMaskClosable | NSWindowStyleMaskMiniaturizable | NSWindowStyleMaskResizable
            2,    // NSBackingStoreBuffered
            NO,
        );
        let _: () = msg_send![window, setTitle:title];
        let _: () = msg_send![window, center];
        window
    }
}

/// Show a window
pub fn show_window(window: *mut objc::runtime::Object) {
    unsafe {
        let _: () = msg_send![window, makeKeyAndOrderFront:nil];
    }
}

/// Close a window
pub fn close_window(window: *mut objc::runtime::Object) {
    unsafe {
        let _: () = msg_send![window, close];
    }
}