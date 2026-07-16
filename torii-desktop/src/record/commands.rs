//! This module defines [tauri] commands associated with the [Record] instance.

use crate::{Component, Record, components::ComponentAction};
use base64::{Engine as _, engine::general_purpose};
use serde_json::{Value, json};
use std::{io::ErrorKind::NotFound, path::PathBuf};
use tauri::ipc::Response;

/// Lists all records in the given directory. This is used to populate the file tree
/// in the workspace UI.
///
/// If the directory does not exist, an empty list is returned.
#[tauri::command]
pub fn list_records(
    workspace: PathBuf,
    directory: PathBuf,
    recursive: Option<bool>,
) -> Result<Vec<Record>, String> {
    let recursive = recursive.unwrap_or(false);

    // Chose the appropriate listing method based on the `recursive` flag.
    let result = if recursive {
        Record::list_recursive(&workspace, &directory)
    } else {
        Record::list(&workspace, &directory)
    };

    // Handle the result of the listing operation. If the directory does not
    // exist, return an empty list.
    match result {
        Ok(records) => Ok(records),
        Err(e) if e.kind() == NotFound => Ok(vec![]),
        Err(e) => Err(format!("Failed to list records: {e}")),
    }
}

/// Renames a given record. This is used to rename a record on the disc. It will
/// move the file to the new name. The `new_name` argument is the new file path
/// relative to the workspace root.
#[tauri::command]
pub fn rename_record(record: Record, new_name: PathBuf) -> Result<Record, String> {
    record
        .rename(new_name)
        .map_err(|e| format!("Failed to rename record: {e}"))
}

/// Removes a given record. This is used to remove a record from the disc. It will
/// remove all files associated with the record.
///
/// If the file is a folder, it will remove all records in the folder. If you want
/// to remove the article functionality from a file and preserve the folder content,
/// you should instead remove the component itself.
#[tauri::command]
pub fn remove_record(record: Record) -> Result<(), String> {
    record
        .associated_paths()
        .map_err(|e| format!("Failed to get record files: {e}"))?
        .iter()
        .map(|file| match file.is_dir() {
            true => std::fs::remove_dir_all(&file),
            false => std::fs::remove_file(&file),
        })
        .collect::<Result<Vec<()>, std::io::Error>>()
        .map_err(|e| format!("Failed to remove record files: {e}"))?;
    Ok(())
}

/// Lists all components attached to a given record. This returns
///
/// This is used in the project view to conditionally render only existing
/// components, and is used in the file tree UI to enable special behaviour
/// for folders.
#[tauri::command]
pub fn list_record_components(record: Record) -> Value {
    record
        .list_components()
        .iter()
        .map(|c| {
            let can_write = c.write(&record, Vec::new()).is_implemented();
            let can_write_from_file = c.write_from_file(&record, &PathBuf::new()).is_implemented();
            let can_read = c.read(&record).is_implemented();
            let can_remove = c.remove(&record).is_implemented();

            json!({
                "name": c.component_name(),
                "permissions": {
                    "write": can_write,
                    "write_from_file": can_write_from_file,
                    "read": can_read,
                    "remove": can_remove,
                }
            })
        })
        .collect()
}

/// Returns the content of a specific component for a given record.
///
/// Under the hood it invokes [read][Component::read] for the provided component,
/// which returns a [Response] containing the component data. The response can
/// be a string, a byte array, or any other data type that the component sends.
#[tauri::command]
pub fn get_record_component(
    record: Record,
    component: Box<dyn Component>,
) -> Result<Response, String> {
    component.read(&record).invoke().map_err(|e| e.to_string())
}

/// Saves (or creates) a specific component for a given record to the disc
/// from a string content.
///
/// This command takes a few additional parameters to specify the content mime type,
/// which is used to determine how to decode the content. The mime types starting with
/// "text" are treated as UTF-8 text, while the mime types starting with "image" are
/// treated as base64-encoded image data.
#[tauri::command]
pub fn save_record_component(
    record: Record,
    component: Box<dyn Component>,
    content: String,
    content_type: String,
) -> Result<(), String> {
    let content_type = content_type.split('/').next().unwrap_or("").to_lowercase();

    let bytes = match content_type.as_str() {
        "text" => content.into_bytes(),
        "image" => general_purpose::STANDARD
            .decode(content)
            .map_err(|e| format!("Failed to decode base64 content: {e}"))?,
        _ => return Err(format!("Unsupported content type: {content_type}")),
    };

    component
        .write(&record, bytes)
        .invoke()
        .map_err(|e| e.to_string())
}

/// Saves (or creates) a specific component for a given record to the disc,
/// from a local file. The file is
#[tauri::command]
pub fn save_record_component_from_local_file(
    record: Record,
    component: Box<dyn Component>,
    source: PathBuf,
) -> Result<(), String> {
    component
        .write_from_file(&record, &source)
        .invoke()
        .map_err(|e| e.to_string())
}

/// Removes a specific component for a given record from the disc.
///
/// This will also cleanup all files that are managed solely by this component.
#[tauri::command]
pub fn remove_record_component(
    record: Record,
    component: Box<dyn Component>,
) -> Result<(), String> {
    component
        .remove(&record)
        .invoke()
        .map_err(|e| e.to_string())

    /*
    // If the component implements a custom remove method, use it. Otherwise, fall back to the default
    // implementation.
    match component.remove(&record) {
        ComponentAction::Unimplemented { .. } => (),
        ComponentAction::Action { action } => {
            return action().map_err(|e| e.to_string());
        }
    };

    // Here begins the default component destructor.

    // TODO: for each other components, keep files which still have a component attached

    let record_files = record
        .associated_paths()
        .map_err(|e| format!("Failed to read record-associated paths: {e}"))?;
    component
        .filter_associated(&record_files)
        .iter()
        .map(|file| {
            std::fs::remove_file(&file)
                .map_err(|e| format!("Failed to remove component file {}: {e}", file.display()))
        })
        .collect::<Result<Vec<()>, String>>()?;

    Ok(())
    */
}
