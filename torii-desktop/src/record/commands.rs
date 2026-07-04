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
