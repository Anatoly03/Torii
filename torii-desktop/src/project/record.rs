//! This module exposes the interface to manage a Torii record.

use base64::{Engine as _, engine::general_purpose};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, fs::read_dir, path::PathBuf};
use tauri::ipc::Response;

use crate::components::{get_all_components, get_component_by_name};

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
        let record = self.directory.join(&self.name);

        // Retrieve all components and find which are attached to the record.
        let listed = get_all_components()
            .iter()
            .filter_map(|comp| match comp.is_attached(&record) {
                true => Some(comp.component_name().to_string()),
                false => None,
            })
            .collect::<Vec<String>>();

        Ok(listed)
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
    let path = directory.join(&name);
    let component =
        get_component_by_name(&component).ok_or(format!("Unknown component: {component}"))?;
    component.read(&path)
}

/// Saves (or creates) a specific component for a given record.
#[tauri::command]
pub fn save_record_component(
    directory: PathBuf,
    name: String,
    component: String,
    content: String,
    content_type: String,
) -> Result<(), String> {
    let path = directory.join(&name);
    let component =
        get_component_by_name(&component).ok_or(format!("Unknown component: {component}"))?;
    let content_type = content_type.split('/').next().unwrap_or("").to_lowercase();

    let bytes = match content_type.as_str() {
        "text" => content.into_bytes(),
        "image" => general_purpose::STANDARD
            .decode(content)
            .map_err(|e| format!("Failed to decode base64 content: {e}"))?,
        _ => return Err(format!("Unsupported content type: {content_type}")),
    };

    component.write(&path, &bytes)
}

/// Saves (or creates) a specific component for a given record.
#[tauri::command]
pub fn save_record_component_from_local_file(
    directory: PathBuf,
    name: String,
    component: String,
    source: PathBuf,
) -> Result<(), String> {
    let path = directory.join(&name);
    let component =
        get_component_by_name(&component).ok_or(format!("Unknown component: {component}"))?;
    match component.write_from_file(&path, &source) {
        Some(result) => result,
        None => Err(format!(
            "Component {} does not support writing from a file.",
            component.component_name()
        )),
    }
}

/// Removes a specific component for a given record. (It will be detached from the record).
///
/// This will also cleanup all files that are managed solely by this component.
#[tauri::command]
pub fn remove_record_component(
    directory: PathBuf,
    name: String,
    component: String,
) -> Result<(), String> {
    let record = Record { directory, name };
    let component =
        get_component_by_name(&component).ok_or(format!("Unknown component: {component}"))?;
    let record_files = record.associated_files()?;

    // TODO: for each other components, keep files which still have a component attached

    component
        .filter_associated(&record_files)
        .iter()
        .map(|file| {
            std::fs::remove_file(&file)
                .map_err(|e| format!("Failed to remove component file {}: {e}", file.display()))
        })
        .collect::<Result<Vec<()>, String>>()?;

    Ok(())
}
