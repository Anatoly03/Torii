//! This module exposes the interface to manage a Torii record.

use serde::{Deserialize, Serialize};
use std::{collections::HashSet, fs::read_dir, io::ErrorKind, path::PathBuf};
use tauri::ipc::Response;

/// A record in a Torii project. This is used to represent a single "thing"
/// in the project, such as an encyclopedia entry, a character sheet or a book
/// chapter.
#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    /// The path to the records' parent directory., without the file extension.
    pub directory: PathBuf,

    /// The name of the record, without the file extension. This is used to get
    /// the path of the component files as they following the following pattern:
    ///
    /// | Component / | Path |
    /// |:----------|:------|
    /// | Markdown  | `file:// <parent> / <name> .md` |
    /// | Image     | `file:// <parent> / <name> .png` |
    /// | Other     | `file:// <parent> / <name> . <component> . <data extension>` |
    ///
    /// A valid record name is a sequence of letters, numbers, underscores, dashes
    /// and spaces. **Not allowed are dots and magic characters.** See
    /// [file_prefix][std::path::Path::file_prefix].
    pub name: String,
}

impl Record {
    /// Lists the records in the given directory. This is used to populate the
    /// file tree in the workspace UI.
    pub fn list(parent: PathBuf) -> Result<Vec<Self>, String> {
        // (security) validate that the directory exists and is a directory
        if !parent.is_dir() {
            return Err(format!("Path is not a directory: {parent:?}"));
        }

        // Retrieve unique record names present in the directory. A record name
        // is the file prefix for all non-magic files.
        let record_names = read_dir(parent.clone())
            .map_err(|e| format!("Failed to read directory: {e}"))?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
            .filter_map(|entry| {
                let name = entry.path().file_prefix()?.to_string_lossy().to_string();

                // (security) skip config files. make sure the magic dot character is not
                // present in the record name
                match name.contains('.') {
                    true => None,
                    false => Some(name),
                }
            })
            .collect::<HashSet<_>>();

        // Create a record for each unique name.
        let records = record_names
            .into_iter()
            .map(|name| Record {
                directory: parent.clone(),
                name,
            })
            .collect();

        Ok(records)
    }

    /// Lists the components attached to a specific record.
    pub fn list_components(&self) -> Result<Vec<String>, String> {
        let mut components = vec!["article".to_string()];

        // If the record has an image, add the "image" component to the list.
        let image_path = self.directory.join(format!("{}.png", self.name));
        if image_path.exists() {
            components.push("image".to_string());
        }

        Ok(components)
    }

    /// Retrieve all files in the directory which define the record. When deleting a record,
    /// all these files should be deleted.
    ///
    /// Note that this does not delete all references to this record in other records.
    pub fn associated_files(&self) -> Result<Vec<PathBuf>, String> {
        let files = read_dir(self.directory.clone())
            .map_err(|e| format!("Failed to read directory: {e}"))?
            .filter_map(|e| e.ok())
            .map(|entry| entry.path())
            .filter(|p| p.is_file())
            .filter(|path| {
                path.file_prefix()
                    .is_some_and(|prefix| prefix.to_str().is_some_and(|n| n == self.name))
            })
            .collect();
        Ok(files)
    }

    /// Retrieve all files in the directory which define the records' components. When
    /// detaching a component from a record, all these files should be deleted.
    pub fn associated_component_files<C: AsRef<str>>(
        &self,
        component: C,
    ) -> Result<Vec<PathBuf>, String> {
        let predicate = match component.as_ref() {
            "article" => |p: &PathBuf| p.extension().is_some_and(|ext| ext == "md"),
            "image" => |p: &PathBuf| p.extension().is_some_and(|ext| ext == "png"),
            _ => |_: &PathBuf| false, // TODO: implement component-specific filtering
        };

        let component_files = self
            .associated_files()?
            .into_iter()
            .filter(predicate)
            .collect();
        Ok(component_files)
    }

    /// Returns the path to the markdown file of the record. This is used to read
    /// and write the "Article" component of the record.
    pub fn get_markdown_path(&self) -> PathBuf {
        self.directory.join(format!("{}.md", self.name))
    }
}

/// Gets the markdown file ("Article" component) of a record.
pub fn get_markdown_file(directory: PathBuf, name: String) -> Result<Vec<u8>, String> {
    let path = directory.join(format!("{}.md", name));

    match std::fs::read(path) {
        Ok(file) => Ok(file),
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(vec![]),
        Err(e) => return Err(format!("Failed to read markdown file: {e}")),
    }
}

/// Gets the image file ("Image" component) of a record.
pub fn get_image_file(directory: PathBuf, name: String) -> Result<Vec<u8>, String> {
    let path = directory.join(format!("{}.png", name));

    match std::fs::read(path) {
        Ok(file) => Ok(file),
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(vec![]),
        Err(e) => return Err(format!("Failed to read image file: {e}")),
    }
}

/// Saves (or creates) the markdown file ("Article" component) of a record.
pub fn save_markdown_file(directory: PathBuf, name: String, content: String) -> Result<(), String> {
    let path = directory.join(format!("{}.md", name));
    std::fs::write(path, content).map_err(|e| format!("Failed to write markdown file: {e}"))
}

/// Removes the markdown file ("Article" component) of a record.
pub fn remove_markdown_file(directory: PathBuf, name: String) -> Result<(), String> {
    let path = directory.join(format!("{}.md", name));
    std::fs::remove_file(path).map_err(|e| format!("Failed to remove markdown file: {e}"))
}

/// Lists all records in the given directory. This is used to populate the file tree
/// in the workspace UI.
#[tauri::command]
pub fn list_records(directory: PathBuf) -> Result<Vec<Record>, String> {
    Record::list(directory)
}

/// Renames a record in the given directory. This is used to rename a record in the workspace UI.
#[tauri::command]
pub fn rename_record(
    directory: PathBuf,
    old_name: String,
    new_name: String,
) -> Result<Vec<Record>, String> {
    Err(format!("Renaming records is not implemented yet"))
}

/// Removes a given record.
#[tauri::command]
pub fn remove_record(directory: PathBuf, name: String) -> Result<(), String> {
    let record = Record { directory, name };

    record
        .associated_files()?
        .iter()
        .map(|file| std::fs::remove_file(&file))
        .collect::<Result<Vec<()>, std::io::Error>>()
        .map_err(|e| format!("Failed to remove record files: {e}"))?;

    Ok(())
}

/// Returns the components attached to a specific record.
#[tauri::command]
pub fn list_record_components(directory: PathBuf, name: String) -> Result<Vec<String>, String> {
    let record = Record { directory, name };
    record.list_components()
}

/// Returns the content of a specific component for a given record.
#[tauri::command]
pub fn get_record_component(
    directory: PathBuf,
    name: String,
    component: String,
) -> Result<Response, String> {
    match component.as_str() {
        "article" => get_markdown_file(directory, name).map(Response::new),
        "image" => get_image_file(directory, name).map(Response::new),
        _ => Err(format!("Unknown component: {component}")),
    }
}

/// Saves (or creates) a specific component for a given record.
#[tauri::command]
pub fn save_record_component(
    directory: PathBuf,
    name: String,
    component: String,
    content: String,
) -> Result<(), String> {
    match component.as_str() {
        "article" => save_markdown_file(directory, name, content),
        "image" => Err("Saving image component is not implemented yet".to_string()),
        _ => Err(format!("Unknown component: {component}")),
    }
}

/// Removes a specific component for a given record. (It will
/// be detached from the record)
#[tauri::command]
pub fn remove_record_component(
    directory: PathBuf,
    name: String,
    component: String,
) -> Result<(), String> {
    let record = Record { directory, name };
    let component_files = record.associated_component_files(&component)?;

    component_files
        .iter()
        .map(|file| {
            std::fs::remove_file(&file)
                .map_err(|e| format!("Failed to remove component file {}: {e}", file.display()))
        })
        .collect::<Result<Vec<()>, String>>()?;

    Ok(())
}
