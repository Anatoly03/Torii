use std::path::PathBuf;

mod article;
mod image;

pub use article::ArticleComponent;
pub use image::ImageComponent;

pub trait ToriiComponent {
    /// Retrieves the name of the component. Examples includes "article" and "image".
    fn component_name(&self) -> &str;

    /// Reads the file path and yields whether the file is associated with this component.
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
    /// The "Image" component oversees the file "Hello World.png". The components "Article" an
    /// "Brief" oversees the file "Hello World.md".
    fn is_associated(&self, path: &PathBuf) -> bool {
        false
    }

    /// Reads the record file path and yields whether the record implements this component.
    /// 
    /// For example if the record is "Users/Anatoly/Teada Recors/Diana Loewe", an "Article"
    /// component will check if the file "Users/Anatoly/Teada Recors/Diana Loewe.md" exists.
    /// The general rule is that if this file exists, so does the component. 
    fn is_attached(&self, path: &PathBuf) -> bool;

    /// Reads through multiple files and filters for files that are associated with this component.
    ///
    /// A file can be associated with multiple components, and multiple components can
    /// managed the same file. For example "Article" and "Brief" both read the same
    /// markdown file. For the components "Image" and "Banner" however, both read different
    /// files.
    ///
    /// When a component is detached from a record, all files associated with that component
    /// who have no other associated components should be deleted.
    fn filter_associated<'a>(&self, path: &'a [PathBuf]) -> Vec<&'a PathBuf> {
        path.iter().filter(|p| self.is_associated(p)).collect()
    }
}

/// Returns a boxed instance of a component based on its name. If the component name is not
/// recognized, it returns None.
pub fn get_component_by_name(name: &str) -> Option<Box<dyn ToriiComponent>> {
    match name {
        "article" => Some(Box::new(article::ArticleComponent)),
        "image" => Some(Box::new(image::ImageComponent)),
        _ => None,
    }
}
