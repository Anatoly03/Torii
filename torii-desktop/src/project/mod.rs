//! The project API for the Torii project.
//!
//! This module exposes the interface to manage a Torii project.

pub mod record;
// TODO components

use std::path::PathBuf;
use tauri::ipc::Response;

/// Returns the content of a specific component for a given record.
#[tauri::command]
pub fn read_file(path: PathBuf, mime_type: String) -> Result<Response, String> {
    let binary_type = mime_type.split('/').next().unwrap();
    let file_extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");

    match (binary_type, file_extension) {
        ("image", "png") | ("image", "jpg") | ("image", "jpeg") | ("image", "gif") => {
            let content =
                std::fs::read(&path).map_err(|e| format!("Failed to read image file: {e}"))?;
            Ok(Response::new(content))
        }
        ("text", "md") => {
            let content = std::fs::read_to_string(&path)
                .map_err(|e| format!("Failed to read markdown file: {e}"))?;
            Ok(Response::new(content))
        }
        _ => Err(format!(
            "File type with extension `{}` and requested mimeType `{}` not allowed to read. Path: {}",
            file_extension,
            mime_type,
            path.display()
        )),
    }
}

// use serde::{Deserialize, Serialize};
// use std::{fs::read_dir, path::PathBuf};

// #[derive(Serialize, Deserialize, Debug)]
// pub struct MarkdownFile {
//     pub path: PathBuf,
// }

// impl MarkdownFile {
//     // /// The name of the markdown file, without the extension. This is used to display
//     // /// the file in the UI.
//     // pub fn get_name(&self) -> String {
//     //     self.path
//     //         .with_extension("")
//     //         .file_name()
//     //         .unwrap_or("<unnamed>".as_ref())
//     //         .to_string_lossy()
//     //         .to_string()
//     // }

//     /// Lists the markdown files in the given directory. This is used to populate the
//     /// file tree in the UI.
//     pub fn list(directory: PathBuf) -> Result<Vec<Self>, String> {
//         let markdown_files = read_dir(directory)
//             .map_err(|e| format!("Failed to read directory: {e}"))?
//             .filter_map(|e| e.ok())
//             .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
//             .map(|entry| MarkdownFile { path: entry.path() })
//             .collect();
//         Ok(markdown_files)
//     }
// }
