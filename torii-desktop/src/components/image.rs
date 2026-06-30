//! This module manages image components within a Torii project.
//!
//! It provides functionality to read image files and handle image-related operations.

use std::{io::ErrorKind, path::PathBuf};

use tauri::ipc::Response;

use super::ToriiComponent;

/// Represents an image component in a Torii project.
pub struct ImageComponent;

impl ToriiComponent for ImageComponent {
    /// Retrieves the name of the component, which is "image".
    ///
    /// # Example
    ///
    /// ```no_test
    /// let image_component = ImageComponent;
    /// assert_eq!(image_component.component_name(), "image");
    /// ```
    fn component_name(&self) -> &str {
        "image"
    }

    /// Reads the file path and yields wether the file is associated with the image component.
    ///
    /// The image component reads files with the ".png" extension.
    ///
    /// A file can be associated with multiple components, and multiple components can
    /// managed the same file. For example "Article" and "Brief" both read the same
    /// markdown file. For the components "Image" and "Banner" however, both read different
    /// files.
    ///
    /// When a component is detached from a record, all files associated with that component
    /// who have no other associated components should be deleted.
    ///
    /// # Example
    ///
    /// ```text
    /// Hello World.md
    /// Hello World.png
    /// ```
    ///
    /// The "Image" component oversees the file "Hello World.png" and will reject the file "Hello
    /// World.md".
    fn is_associated(&self, path: &PathBuf) -> bool {
        path.extension().is_some_and(|ext| ext == "png")
    }

    /// Reads the record file path and yields whether the record implements the image component.
    ///
    /// For example if the record is "Diana Loewe", we scan for "Diana Loewe.png" in the record's
    /// directory. If this file exists, then the record implements the image component.
    fn is_attached(&self, path: &PathBuf) -> bool {
        path.with_extension("png").exists()
    }

    /// Gets a read request to view the "Image" component data for a record. This returns a
    /// [Response][ipc::Response] containing the image data.
    fn read(&self, record: &PathBuf) -> Result<Response, String> {
        let path = record.with_extension("png");

        let file = match std::fs::read(path) {
            Ok(file) => file,
            Err(e) if e.kind() == ErrorKind::NotFound => vec![],
            Err(e) => return Err(format!("Failed to read image file: {e}")),
        };

        Ok(Response::new(file))
    }
}
