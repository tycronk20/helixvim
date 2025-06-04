//! Clipboard integration for HelixVim
//!
//! This module provides integration with the macOS clipboard.

use anyhow::Result;
use cocoa::base::{id, nil};
use cocoa::foundation::{NSArray, NSString};
use objc::{msg_send, sel, sel_impl};

/// Get text from clipboard
pub fn get_clipboard_text() -> Result<String> {
    unsafe {
        let pasteboard: id = msg_send![class!(NSPasteboard), generalPasteboard];
        let string: id = msg_send![pasteboard, stringForType:NSString::alloc(nil).init_str("public.utf8-plain-text")];
        
        if string == nil {
            return Ok(String::new());
        }
        
        let nsstring = NSString(string);
        let bytes = nsstring.utf8_str();
        let string = std::str::from_utf8(bytes)?;
        
        Ok(string.to_string())
    }
}

/// Set text to clipboard
pub fn set_clipboard_text(text: &str) -> Result<()> {
    unsafe {
        let pasteboard: id = msg_send![class!(NSPasteboard), generalPasteboard];
        let _: () = msg_send![pasteboard, clearContents];
        
        let nsstring = NSString::alloc(nil).init_str(text);
        let array = NSArray::arrayWithObject(nil, nsstring);
        
        let _: BOOL = msg_send![
            pasteboard,
            writeObjects:array
        ];
        
        Ok(())
    }
}

/// Check if clipboard has text
pub fn clipboard_has_text() -> bool {
    unsafe {
        let pasteboard: id = msg_send![class!(NSPasteboard), generalPasteboard];
        let types: id = msg_send![pasteboard, types];
        let string_type = NSString::alloc(nil).init_str("public.utf8-plain-text");
        let contains: BOOL = msg_send![types, containsObject:string_type];
        
        contains == YES
    }
}

/// Clear clipboard
pub fn clear_clipboard() -> Result<()> {
    unsafe {
        let pasteboard: id = msg_send![class!(NSPasteboard), generalPasteboard];
        let _: () = msg_send![pasteboard, clearContents];
        
        Ok(())
    }
}