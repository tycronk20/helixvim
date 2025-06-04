//! HelixVim Bridge
//!
//! This crate provides the bridge between Helix and MacVim's Objective-C code.
//! It exposes Helix functionality through a C-compatible FFI interface.

use std::ffi::{c_char, CStr, CString};
use std::ptr;

/// FFI interface for HelixVim
#[no_mangle]
pub extern "C" fn helixvim_version() -> *const c_char {
    let version = "HelixVim 0.1.0";
    let c_str = CString::new(version).unwrap();
    c_str.into_raw()
}

/// Initialize Helix editor
#[no_mangle]
pub extern "C" fn helixvim_init() -> bool {
    // TODO: Initialize Helix core
    true
}

/// Process a keystroke
#[no_mangle]
pub extern "C" fn helixvim_process_key(key: *const c_char) -> bool {
    if key.is_null() {
        return false;
    }
    
    let key_str = unsafe { CStr::from_ptr(key) }.to_str().unwrap_or("");
    
    // TODO: Pass keystroke to Helix
    println!("Processing key: {}", key_str);
    
    true
}

/// Free a string allocated by this library
#[no_mangle]
pub extern "C" fn helixvim_free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_version() {
        let version_ptr = helixvim_version();
        let version = unsafe { CStr::from_ptr(version_ptr) }.to_str().unwrap();
        assert!(version.contains("HelixVim"));
        
        // Clean up
        helixvim_free_string(version_ptr as *mut c_char);
    }
}