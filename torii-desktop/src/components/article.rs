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
    fn is_associated(&self, path: &PathBuf) -> bool {
        path.extension().is_some_and(|ext| ext == "md")
    }
}
