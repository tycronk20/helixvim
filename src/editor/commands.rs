//! Editor commands for HelixVim
//!
//! This module defines commands that can be executed in the editor.

use anyhow::Result;
use crate::editor::Editor;

/// Open a file
pub fn open_file(editor: &mut Editor, path: &str) -> Result<()> {
    editor.open(path)
}

/// Save the current file
pub fn save_file(editor: &mut Editor) -> Result<()> {
    editor.save()
}

/// Save the current file with a new name
pub fn save_file_as(editor: &mut Editor, path: &str) -> Result<()> {
    // TODO: Implement save as
    Ok(())
}

/// Create a new file
pub fn new_file(editor: &mut Editor) -> Result<()> {
    // TODO: Create new file
    Ok(())
}

/// Close the current file
pub fn close_file(editor: &mut Editor) -> Result<()> {
    // TODO: Close current file
    Ok(())
}

/// Undo the last action
pub fn undo(editor: &mut Editor) -> Result<()> {
    // TODO: Implement undo
    Ok(())
}

/// Redo the last undone action
pub fn redo(editor: &mut Editor) -> Result<()> {
    // TODO: Implement redo
    Ok(())
}