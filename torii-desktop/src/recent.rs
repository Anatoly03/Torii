//! The recent documents API for the Torii project.
//!
//! This module exposes the inteface to list, add and remove recent documents
//! from the desktop application.

use serde::{Deserialize, Serialize};
use std::{fs::File, path::PathBuf};
use tauri::{AppHandle, Manager};

/// The path to the file where recent projects are stored. This is a JSON file
/// that contains an array of `RecentProjectMetadata` objects. The parent is
/// `$APPLOCALDATA`
const RECENT_PROJECTS_FILE: &str = "recent_projects.json";

/// The metdata for a recently opened project. This includes the name and path of the project.
#[derive(Serialize, Deserialize, Debug)]
pub struct RecentProjectMetadata {
    /// The name of the project. This is used to display the project in the UI.
    pub name: String,

    /// The path to the project. This is used to open the project when the user clicks
    /// on it in the UI.
    pub path: PathBuf,

    /// The timestamp of when the project was last opened, in milliseconds since
    /// the Unix epoch.
    #[serde(skip_deserializing, default = "time_now")]
    pub last_opened: u64,
}

/// Gets the current time in milliseconds since the Unix epoch. This is used to
/// set the `last_opened` field of the `RecentProjectMetadata` struct when a project
/// is added to the list of recent projects.
pub fn time_now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| format!("Failed to get current time: {e}"))
        .map(|d| d.as_millis() as u64)
        .unwrap_or_default()
}

/// Gets the list of recently opened projects. This is used by the client to display the list
/// of recent projects in the UI.
pub async fn save_recent_projects(
    app: AppHandle,
    projects: Vec<RecentProjectMetadata>,
) -> Result<(), String> {
    let recent_projects_list = app
        .path()
        .app_local_data_dir()
        .map_err(|e| format!("Failed to get app local data directory: {e}"))?
        .join(RECENT_PROJECTS_FILE);
    let file = File::create(recent_projects_list)
        .map_err(|e| format!("Failed to create recent projects file: {e}"))?;
    serde_json::to_writer(file, &projects)
        .map_err(|e| format!("Failed to write recent projects file: {e}"))
}

/// Lists recently opened projects. This is used by the client to display the
/// list of recent projects in the UI.
#[tauri::command]
pub async fn list_recent_projects(app: AppHandle) -> Result<Vec<RecentProjectMetadata>, String> {
    let recent_projects_list = app
        .path()
        .app_local_data_dir()
        .map_err(|e| format!("Failed to get app local data directory: {e}"))?
        .join(RECENT_PROJECTS_FILE);
    // println!("Recent projects: {recent_projects_list:?}");
    let file = File::open(recent_projects_list)
        .map_err(|e| format!("Failed to open recent projects file: {e}"))?;
    serde_json::from_reader(file).map_err(|e| format!("Failed to parse recent projects file: {e}"))
}

/// Adds a recently opened project.
#[tauri::command]
pub async fn add_recent_project(
    app: AppHandle,
    metadata: RecentProjectMetadata,
) -> Result<(), String> {
    let mut recent_projects = list_recent_projects(app.clone()).await.unwrap_or_default();
    recent_projects.retain(|p| p.path != metadata.path);
    recent_projects.push(metadata);
    save_recent_projects(app, recent_projects).await
}

/// Removes a recently opened project.
#[tauri::command]
pub async fn remove_recent_project(app: AppHandle, path: PathBuf) -> Result<(), String> {
    let mut recent_projects = list_recent_projects(app.clone()).await.unwrap_or_default();
    recent_projects.retain(|p| p.path != path);
    save_recent_projects(app, recent_projects).await
}
