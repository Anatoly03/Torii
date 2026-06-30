//! This module contains the definition of the [ToriiComponent] trait and its core implementations.

mod article;
mod image;

pub use article::ArticleComponent;
pub use image::ImageComponent;
use std::path::PathBuf;
use tauri::ipc::Response;

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
    fn is_associated(&self, path: &PathBuf) -> bool;

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

    /// Gets a read request to view the component data for a record. This returns a
    /// [Response][ipc::Response].
    ///
    /// For example, the "Article" component will return a markdown string, while the "Image"
    /// component will return a raw byte array of the image data.
    fn read(&self, record: &PathBuf) -> Result<Response, String>;

    /// Gets a write request to save the component data for a record. This takes a
    /// base64 encoded string representing the binary data to be saved.
    ///
    /// For example, the "Article" component will interpret the resulting binary as a
    /// markdown string, while the "Image" component will interpret content as raw byte
    /// data.
    fn write(&self, record: &PathBuf, content: &[u8]) -> Result<(), String>;

    /// Gets a write request to save the component data for a record, taking a local
    /// file path as the copy source. This method is optional and the return type is to
    /// be unerstood as follows:
    ///
    /// - [None]: The component does not support writing from a file.
    /// - [Some(Ok)][Some]: The component successfully wrote the file.
    /// - [Some(Err)][Some]: The component understanding the request but failed to write
    ///   the file.
    ///
    /// For example, the "Article" component will reject the request to write from a file,
    /// while the "Image" component will accept a file path and copy the file to the record's
    /// directory.
    fn write_from_file(&self, _record: &PathBuf, _source: &PathBuf) -> Option<Result<(), String>> {
        None
    }

    /// Gets a remove request to delete the component data for a record. This method is
    /// optional and the return type is to be understood as follows:
    ///
    /// - [None]: The component does not implement a custom remove method. The default
    ///   behaviour (pointer calculation and file deletion) will be used.
    /// - [Some(Ok)][Some]: The component was successfully deleted.
    /// - [Some(Err)][Some]: The component failed to delete the component data for the
    ///   record.
    ///
    /// Since a component can be associated with multiple files, and multiple components
    /// can be associated with the same file, this method is expected to remove all files
    /// that are solely associated with this component. 
    /// 
    /// For example, the "Article" component will remove the file "Diana Loewe.md", overriding
    /// the default behaviour of removing all (!) markdown files, as well will the "Image"
    /// component only check for the exact file "Diana Loewe.png" and remove it, but keep
    /// the "Banner" component's file "Diana Loewe.banner.png" intact.
    fn remove(&self, record: &PathBuf) -> Option<Result<(), String>>;
}

/// Returns a boxed instance of a component based on its name. If the component name is not
/// recognized, it returns None.
pub fn get_component_by_name(name: &str) -> Option<Box<dyn ToriiComponent>> {
    match name {
        "article" => Some(Box::new(ArticleComponent)),
        "image" => Some(Box::new(ImageComponent::new("image", "png"))),
        "banner" => Some(Box::new(ImageComponent::new("banner", "banner.png"))),
        _ => None,
    }
}

/// Returns a boxed instance of a component based on its name. If the component name is not
/// recognized, it returns None.
pub fn get_all_components() -> Vec<Box<dyn ToriiComponent>> {
    vec![
        Box::new(ArticleComponent),
        Box::new(ImageComponent::new("image", "png")),
        Box::new(ImageComponent::new("banner", "banner.png")),
    ]
}
