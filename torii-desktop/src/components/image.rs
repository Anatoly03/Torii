//! This module manages image components within a Torii project.
//!
//! It provides functionality to read image files and handle image-related operations.

use super::{Component, ComponentAction};
use crate::Record;
use std::{io::ErrorKind, path::PathBuf};
use tauri::ipc::Response;

/// Represents an image component in a Torii project.
pub struct ImageComponent {
    component_name: String,
    file_suffix: String,
}

impl ImageComponent {
    pub fn new(component_name: impl AsRef<str>, file_suffix: impl AsRef<str>) -> Self {
        Self {
            component_name: component_name.as_ref().into(),
            file_suffix: file_suffix.as_ref().into(),
        }
    }
}

impl Component for ImageComponent {
    /// Retrieves the name of the component, which is "image".
    ///
    /// # Example
    ///
    /// ```no_test
    /// let image_component = ImageComponent;
    /// assert_eq!(image_component.component_name(), "image");
    /// ```
    fn component_name(&self) -> &str {
        &self.component_name
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
        path.file_name()
            .is_some_and(|name| name.to_string_lossy().ends_with(&self.file_suffix))
    }

    /// Reads the record file path and yields whether the record implements the image component.
    ///
    /// For example if the record is "Diana Loewe", we scan for "Diana Loewe.png" in the record's
    /// directory. If this file exists, then the record implements the image component.
    fn is_attached(&self, record: &Record) -> bool {
        record.path().with_extension(&self.file_suffix).exists()
    }

    /// Gets a read request to view the "Image" component data for a record. This returns a
    /// [Response][ipc::Response] containing the image data.
    fn read(&self, record: &Record) -> ComponentAction<Response> {
        let path = record.path().with_extension(&self.file_suffix);

        ComponentAction::Action {
            action: Box::new(|| {
                let file = match std::fs::read(path) {
                    Ok(file) => file,
                    Err(e) if e.kind() == ErrorKind::NotFound => vec![],
                    Err(e) => return Err(format!("Failed to read image file: {e}").into()),
                };

                Ok(Response::new(file))
            }),
        }
    }

    /// Gets a write request to save the component data for a record. This takes a
    /// base64 encoded string representing the binary data to be saved.
    ///
    /// The "Image" component will interpret content as raw byte data.
    fn write(&self, record: &Record, content: Vec<u8>) -> ComponentAction<()> {
        let path = record.path().with_extension(&self.file_suffix);

        ComponentAction::Action {
            action: Box::new(move || std::fs::write(path, content).map_err(|e| e.into())),
        }
    }

    /// Gets a write request to save the "Image" component for a record, taking a local
    /// file path as the copy source. This method returns the following:
    ///
    /// - [Some(Ok)][Some]: The component successfully copied the file to the record's directory.
    /// - [Some(Err)][Some]: The component failed to copy the file to the record's directory.
    ///
    /// The "Image" component will accept a file path and copy the file to the record's directory.
    fn write_from_file(&self, record: &Record, source: &PathBuf) -> ComponentAction<()> {
        let source = source.to_owned();
        let destination = record.path().with_extension(&self.file_suffix);

        ComponentAction::Action {
            action: Box::new(move || {
                std::fs::copy(source, destination)
                    .map(|_| ())
                    .map_err(|e| e.into())
            }),
        }
    }

    /// Gets a remove request to delete the image data for a record. The return type
    /// is to be understood as follows:
    ///
    /// - [Some(Ok)][Some]: The image was successfully deleted.
    /// - [Some(Err)][Some]: The image was not deleted due to an error.
    ///
    /// Since a component can be associated with multiple files, and multiple components
    /// can be associated with the same file, this method is expected to remove all files
    /// that are solely associated with this component.
    ///
    /// The "Article" component will remove the file "<entity>.md"
    fn remove(&self, record: &Record) -> ComponentAction<()> {
        let path = record.path().with_extension(&self.file_suffix);
        ComponentAction::Action {
            action: Box::new(move || std::fs::remove_file(&path).map_err(|e| e.into())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_associated_files() {
        let image_component = ImageComponent::new("image", "png");
        // assert!(!image_component.is_associated(&PathBuf::from("Hello World.banner.png")));
        assert!(image_component.is_associated(&PathBuf::from("Hello World.png")));
        assert!(!image_component.is_associated(&PathBuf::from("Hello World.md")));

        let banner_component = ImageComponent::new("banner", "banner.png");
        assert!(banner_component.is_associated(&PathBuf::from("Hello World.banner.png")));
        assert!(!banner_component.is_associated(&PathBuf::from("Hello World.png")));
        assert!(!banner_component.is_associated(&PathBuf::from("Hello World.md")));
    }
}
