//! This module manages article components within a Torii project.
//!
//! It provides functionality to read article files and handle article-related operations.

use std::path::PathBuf;

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
}
