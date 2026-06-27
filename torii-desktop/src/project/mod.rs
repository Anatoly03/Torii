//! The project API for the Torii project.
//!
//! This module exposes the interface to manage a Torii project.

pub mod record;
// TODO components

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
