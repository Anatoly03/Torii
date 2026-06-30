//! This module manages article components within a Torii project.
//!
//! It provides functionality to read article files and handle article-related operations.

use std::{io::ErrorKind, path::PathBuf};

use tauri::ipc::Response;

use super::ToriiComponent;

/// Represents an article component in a Torii project.
pub struct ArticleComponent;

impl ToriiComponent for ArticleComponent {
    /// Retrieves the name of the component, which is "article".
    ///
    /// # Example
    ///
    /// ```no_test
    /// let article_component = ArticleComponent;
    /// assert_eq!(article_component.component_name(), "article");
    /// ```
    fn component_name(&self) -> &str {
        "article"
    }

    /// Reads the file path and yields wether the file is associated with the article component.
    ///
    /// The article component reads files with the ".md" extension.
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
    /// The "Article" component oversees the file "Hello World.md" and will reject the file "Hello
    /// World.png".
    fn is_associated(&self, path: &PathBuf) -> bool {
        path.extension().is_some_and(|ext| ext == "md")
    }

    /// Reads the record file path and yields whether the record implements the article component.
    ///
    /// For example if the record is "Diana Loewe", we scan for "Diana Loewe.md" in the record's
    /// directory. If this file exists, then the record implements the article component.
    fn is_attached(&self, path: &PathBuf) -> bool {
        path.with_extension("md").exists()
    }

    /// Gets a read request to view the "Article" component data for a record. This returns a
    /// [Response][ipc::Response] containing the markdown string of the article.
    fn read(&self, record: &PathBuf) -> Result<Response, String> {
        let path = record.with_extension("md");

        let file = match std::fs::read(path) {
            Ok(file) => file,
            Err(e) if e.kind() == ErrorKind::NotFound => vec![],
            Err(e) => return Err(format!("Failed to read markdown file: {e}")),
        };

        Ok(Response::new(file))
    }

    /// Gets a write request to save the component data for a record. This takes a
    /// base64 encoded string representing the binary data to be saved.
    ///
    /// The "Article" component will interpret the resulting binary as a markdown
    /// string.
    fn write(&self, record: &PathBuf, content: &[u8]) -> Result<(), String> {
        let path = record.with_extension("md");
        std::fs::write(path, content).map_err(|e| format!("Failed to write markdown file: {e}"))
    }

    /// Gets a remove request to delete the article for a record. The return type
    /// is to be understood as follows:
    ///
    /// - [Some(Ok)][Some]: The article was successfully deleted.
    /// - [Some(Err)][Some]: The article was not deleted due to an error.
    ///
    /// Since a component can be associated with multiple files, and multiple components
    /// can be associated with the same file, this method is expected to remove all files
    /// that are solely associated with this component.
    ///
    /// The "Article" component will remove the file "<entity>.md"
    fn remove(&self, _: &PathBuf) -> Option<Result<(), String>> {
        Some(Err(
            "The article component can not be removed from a record.".to_string(),
        ))
    }
}
