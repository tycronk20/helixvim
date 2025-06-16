//! Editor state management for MacHelix
//!
//! This module handles the state of the editor, including document management.

use anyhow::Result;
use helix_core::Selection;
use helix_view::Document;

/// Editor state
pub struct EditorState {
    /// Current document
    pub document: Option<Document>,
    /// Current selection
    pub selection: Option<Selection>,
    /// Whether the document has been modified
    pub modified: bool,
}

impl EditorState {
    /// Create a new editor state
    pub fn new() -> Self {
        Self {
            document: None,
            selection: None,
            modified: false,
        }
    }
    
    /// Set the current document
    pub fn set_document(&mut self, document: Document) {
        self.document = Some(document);
        self.modified = false;
    }
    
    /// Set the current selection
    pub fn set_selection(&mut self, selection: Selection) {
        self.selection = Some(selection);
    }
    
    /// Mark the document as modified
    pub fn mark_modified(&mut self) {
        self.modified = true;
    }
    
    /// Check if the document has been modified
    pub fn is_modified(&self) -> bool {
        self.modified
    }
}