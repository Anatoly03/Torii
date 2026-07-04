//! This module defines [tauri] commands associated with the [Record] instance.

use crate::Record;
use std::{io::ErrorKind::NotFound, path::PathBuf};

/// Lists all records in the given directory. This is used to populate the file tree
/// in the workspace UI.
///
/// If the directory does not exist, an empty list is returned.
#[tauri::command]
pub fn list_records(directory: PathBuf) -> Result<Vec<Record>, String> {
    match Record::list(directory) {
        Ok(records) => Ok(records),
        Err(e) if e.kind() == NotFound => Ok(vec![]),
        Err(e) => Err(format!("Failed to list records: {e}")),
    }
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
