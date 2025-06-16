//! File dialog integration for MacHelix
//!
//! This module provides integration with macOS file dialogs.

use anyhow::Result;
use cocoa::base::{id, nil, YES, NO};
use cocoa::foundation::{NSArray, NSString};
use objc::{msg_send, sel, sel_impl};

/// Show an open file dialog
pub fn show_open_dialog(
    title: &str,
    message: &str,
    default_path: Option<&str>,
    file_types: &[&str],
    allow_multiple: bool,
) -> Result<Vec<String>> {
    unsafe {
        let panel: id = msg_send![class!(NSOpenPanel), openPanel];
        
        let title_str = NSString::alloc(nil).init_str(title);
        let _: () = msg_send![panel, setTitle:title_str];
        
        let message_str = NSString::alloc(nil).init_str(message);
        let _: () = msg_send![panel, setMessage:message_str];
        
        let _: () = msg_send![panel, setCanChooseFiles:YES];
        let _: () = msg_send![panel, setCanChooseDirectories:NO];
        let _: () = msg_send![panel, setAllowsMultipleSelection:if allow_multiple { YES } else { NO }];
        
        if let Some(path) = default_path {
            let url: id = msg_send![
                class!(NSURL),
                fileURLWithPath:NSString::alloc(nil).init_str(path)
            ];
            let _: () = msg_send![panel, setDirectoryURL:url];
        }
        
        if !file_types.is_empty() {
            let mut types = Vec::with_capacity(file_types.len());
            for &file_type in file_types {
                types.push(NSString::alloc(nil).init_str(file_type));
            }
            
            let types_array = NSArray::arrayWithObjects(nil, &types);
            let _: () = msg_send![panel, setAllowedFileTypes:types_array];
        }
        
        let result: NSInteger = msg_send![panel, runModal];
        
        if result == 1 { // NSModalResponseOK
            let urls: id = msg_send![panel, URLs];
            let count: NSUInteger = msg_send![urls, count];
            
            let mut paths = Vec::with_capacity(count as usize);
            
            for i in 0..count {
                let url: id = msg_send![urls, objectAtIndex:i];
                let path: id = msg_send![url, path];
                
                let nsstring = NSString(path);
                let bytes = nsstring.utf8_str();
                let string = std::str::from_utf8(bytes)?;
                
                paths.push(string.to_string());
            }
            
            Ok(paths)
        } else {
            Ok(Vec::new())
        }
    }
}

/// Show a save file dialog
pub fn show_save_dialog(
    title: &str,
    message: &str,
    default_path: Option<&str>,
    default_name: Option<&str>,
    file_types: &[&str],
) -> Result<Option<String>> {
    unsafe {
        let panel: id = msg_send![class!(NSSavePanel), savePanel];
        
        let title_str = NSString::alloc(nil).init_str(title);
        let _: () = msg_send![panel, setTitle:title_str];
        
        let message_str = NSString::alloc(nil).init_str(message);
        let _: () = msg_send![panel, setMessage:message_str];
        
        if let Some(path) = default_path {
            let url: id = msg_send![
                class!(NSURL),
                fileURLWithPath:NSString::alloc(nil).init_str(path)
            ];
            let _: () = msg_send![panel, setDirectoryURL:url];
        }
        
        if let Some(name) = default_name {
            let name_str = NSString::alloc(nil).init_str(name);
            let _: () = msg_send![panel, setNameFieldStringValue:name_str];
        }
        
        if !file_types.is_empty() {
            let mut types = Vec::with_capacity(file_types.len());
            for &file_type in file_types {
                types.push(NSString::alloc(nil).init_str(file_type));
            }
            
            let types_array = NSArray::arrayWithObjects(nil, &types);
            let _: () = msg_send![panel, setAllowedFileTypes:types_array];
        }
        
        let result: NSInteger = msg_send![panel, runModal];
        
        if result == 1 { // NSModalResponseOK
            let url: id = msg_send![panel, URL];
            let path: id = msg_send![url, path];
            
            let nsstring = NSString(path);
            let bytes = nsstring.utf8_str();
            let string = std::str::from_utf8(bytes)?;
            
            Ok(Some(string.to_string()))
        } else {
            Ok(None)
        }
    }
}