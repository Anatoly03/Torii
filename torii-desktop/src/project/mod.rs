//! The project API for the Torii project.
//!
//! This module exposes the interface to manage a Torii project.

use serde::{Deserialize, Serialize};
use std::{fs::read_dir, path::PathBuf};

#[derive(Serialize, Deserialize, Debug)]
pub struct MarkdownFile {
    pub path: PathBuf,
}

impl MarkdownFile {
    // /// The name of the markdown file, without the extension. This is used to display
    // /// the file in the UI.
    // pub fn get_name(&self) -> String {
    //     self.path
    //         .with_extension("")
    //         .file_name()
    //         .unwrap_or("<unnamed>".as_ref())
    //         .to_string_lossy()
    //         .to_string()
    // }

    /// Lists the markdown files in the given directory. This is used to populate the
    /// file tree in the UI.
    pub async fn list(directory: PathBuf) -> Result<Vec<Self>, String> {
        let markdown_files = read_dir(directory)
            .map_err(|e| format!("Failed to read directory: {e}"))?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
            .map(|entry| MarkdownFile { path: entry.path() })
            .collect();
        Ok(markdown_files)
    }
}

#[tauri::command]
pub async fn list_markdown_files(directory: PathBuf) -> Result<Vec<MarkdownFile>, String> {
    MarkdownFile::list(directory).await
}

#[tauri::command]
pub async fn get_markdown_file(path: PathBuf) -> Result<String, String> {
    // (security) validate that the file has the correct extension before trying to read it
    if path.extension().map_or(true, |ext| ext != "md") {
        let ext = path
            .extension()
            .map(|ext| ext.to_string_lossy())
            .unwrap_or("<no extension>".into());
        return Err(format!("Extension not allowed, expected `md`, got `{ext}`",));
    }

    std::fs::read_to_string(path).map_err(|e| format!("Failed to read markdown file: {e}"))
}

#[tauri::command]
pub async fn save_markdown_file(path: PathBuf, content: String) -> Result<(), String> {
    // (security) validate that the file has the correct extension before trying to read it
    if path.extension().map_or(true, |ext| ext != "md") {
        let ext = path
            .extension()
            .map(|ext| ext.to_string_lossy())
            .unwrap_or("<no extension>".into());
        return Err(format!("Extension not allowed, expected `md`, got `{ext}`",));
    }

    std::fs::write(path, content).map_err(|e| format!("Failed to write markdown file: {e}"))
}

#[tauri::command]
pub async fn remove_markdown_file(path: PathBuf) -> Result<(), String> {
    // (security) validate that the file has the correct extension before trying to read it
    if path.extension().map_or(true, |ext| ext != "md") {
        let ext = path
            .extension()
            .map(|ext| ext.to_string_lossy())
            .unwrap_or("<no extension>".into());
        return Err(format!("Extension not allowed, expected `md`, got `{ext}`",));
    }

    std::fs::remove_file(path).map_err(|e| format!("Failed to remove markdown file: {e}"))
}
