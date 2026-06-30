//! This module manages image components within a Torii project.
//!
//! It provides functionality to read image files and handle image-related operations.

use std::path::PathBuf;

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
    fn is_associated(&self, path: &PathBuf) -> bool {
        path.extension().is_some_and(|ext| ext == "png")
    }
}
