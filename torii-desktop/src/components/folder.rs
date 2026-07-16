//! This module manages folders within a Torii project.
//!
//! It provides functionality to read directory contents and handle folder-related operations.

use super::{Component, ComponentAction};
use crate::Record;
use serde_json::json;
use std::{io::ErrorKind, path::PathBuf};
use tauri::ipc::Response;

/// Represents a folder component in a Torii project.
#[derive(Clone, Debug)]
pub struct FolderComponent;

impl Component for FolderComponent {
    /// Retrieves the name of the component, which is "folder".
    ///
    /// # Example
    ///
    /// ```no_test
    /// let folder_component = FolderComponent;
    /// assert_eq!(folder_component.component_name(), "folder");
    /// ```
    fn component_name(&self) -> &str {
        "folder"
    }

    /// Creates a boxed clone of the component.
    /// 
    /// # Example
    /// 
    /// ```
    /// use app_lib::components::{Component, FolderComponent};
    /// 
    /// let folder_component = FolderComponent;
    /// let cloned_component = folder_component.clone_component();
    /// ```
    fn clone_component(&self) -> Box<dyn Component> {
        Box::new(Self)
    }

    /// Reads the file path and yields wether the file is associated with the folder component.
    ///
    /// The folder component reads directories.
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
    /// The "Folder" component oversees the directory "Hello World" and will reject the file "Hello
    /// World.png".
    fn is_associated(&self, path: &PathBuf) -> bool {
        path.is_dir()
    }

    /// Reads the record file path and yields whether the record implements the folder component.
    ///
    /// For example if the record is "Diana Loewe", we scan for "Diana Loewe.md" in the record's
    /// directory. If this file exists, then the record implements the folder component.
    fn is_attached(&self, record: &Record) -> bool {
        record.path().is_dir()
    }

    /// Gets a read request to view the "Folder" component data for a record. This returns a
    /// [Response][ipc::Response] containing the list of files and directories within the folder.
    fn read(&self, record: &Record) -> ComponentAction<Response> {
        let path: PathBuf = record.path();

        ComponentAction::Action {
            action: Box::new(|| {
                let files = match std::fs::read_dir(path) {
                    Ok(entries) => entries
                        .filter_map(|entry| entry.ok())
                        .map(|entry| entry.path().to_string_lossy().to_string())
                        .collect::<Vec<String>>(),
                    Err(e) if e.kind() == ErrorKind::NotFound => vec![],
                    Err(e) => return Err(format!("Failed to read folder: {e}").into()),
                };

                let value = json!({ "files": files });
                Ok(Response::new(value.to_string()))
            }),
        }
    }

    /// Gets a write request to save the component data for a record. This takes a
    /// base64 encoded string representing the binary data to be saved.
    ///
    /// The "Article" component will interpret the resulting binary as a markdown
    /// string.
    fn write(&self, record: &Record, _content: Vec<u8>) -> ComponentAction<()> {
        let path = record.path();

        ComponentAction::Action {
            action: Box::new(move || std::fs::create_dir_all(path).map_err(|e| e.into())),
        }
    }

    /// Gets a write request to save the component for a record, taking a local file path as
    /// the copy source. This method returns the following.
    fn write_from_file(&self, _record: &Record, _source: &PathBuf) -> ComponentAction<()> {
        ComponentAction::unimplemented("The `Folder` component cannot be copied from a file.")
    }

    /// Gets a remove request to delete the folder for a record. The return type
    /// is to be understood as follows:
    ///
    /// - [Some(Ok)][Some]: The folder was successfully deleted.
    /// - [Some(Err)][Some]: The folder was not deleted due to an error.
    ///
    /// Since a component can be associated with multiple files, and multiple components
    /// can be associated with the same file, this method is expected to remove all files
    /// that are solely associated with this component.
    ///
    /// The "Folder" component will remove the file "<entity>.md"
    fn remove(&self, _record: &Record) -> ComponentAction<()> {
        ComponentAction::unimplemented("The `Folder` component cannot be removed from a record.")
    }
}
