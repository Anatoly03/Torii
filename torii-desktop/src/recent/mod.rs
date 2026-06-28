//! The recent documents API for the Torii project.
//!
//! This module exposes the inteface to list, add and remove recent documents
//! from the desktop application.

use serde::{Deserialize, Serialize};
use std::{fs::File, io::ErrorKind, path::PathBuf};
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

    /// Wether the project is managed by the system. Such projects can not be removed from the
    /// recent projects list.
    #[serde(skip_deserializing, default = "default_false")]
    pub is_system: bool,

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

/// Returns false. This is used to set default false fields in [RecentProjectMetadata].
pub fn default_false() -> bool {
    false
}

impl RecentProjectMetadata {
    /// Retrieves the recent projects from the global file.
    pub async fn get(app: AppHandle) -> Result<Vec<Self>, String> {
        // Compute the path to the recent projects file. This is stored in the app
        // local data directory.
        let recent_projects_list_path = app
            .path()
            .app_local_data_dir()
            .map_err(|e| format!("Failed to get app local data directory: {e}"))?
            .join(RECENT_PROJECTS_FILE);
        // Parse the recent projects. If the file does not exist yet, assume we have
        // no recent projects. If any other error occurs report.
        let mut projects: Vec<RecentProjectMetadata> = match File::open(recent_projects_list_path) {
            Ok(file) => serde_json::from_reader(file)
                .map_err(|e| format!("Failed to parse recent projects file: {e}"))?,
            Err(e) if e.kind() == ErrorKind::NotFound => vec![],
            Err(e) => return Err(format!("Failed to open recent projects file: {e}")),
        };

        if cfg!(dev) {
            // If we are in development mode, link the `torii-example` demo workspace. This
            // Torii project is version tracked in this repository and shared with collaborators.
            let workspace_dir = std::env::var("CARGO_MANIFEST_DIR")
                .map(|dir| PathBuf::from(dir).join("../torii-example"));

            // Link demo project for development purposes.
            if let Ok(dir) = workspace_dir
                && dir.is_dir()
            {
                projects.push(RecentProjectMetadata {
                    name: "Torii Guide".to_string(),
                    path: dir,
                    is_system: true,
                    last_opened: 0,
                });
            }
        }

        Ok(projects)
    }

    /// Saves the recent project metadata to the recent projects file. This is used
    /// when a project is added to the list of recent projects.
    pub async fn save(app: AppHandle, list: Vec<Self>) -> Result<(), String> {
        let recent_projects_list = app
            .path()
            .app_local_data_dir()
            .map_err(|e| format!("Failed to get app local data directory: {e}"))?
            .join(RECENT_PROJECTS_FILE);
        let file = File::create(recent_projects_list)
            .map_err(|e| format!("Failed to create recent projects file: {e}"))?;
        serde_json::to_writer(file, &list)
            .map_err(|e| format!("Failed to write recent projects file: {e}"))
    }
}

/// Lists recently opened projects. This is used by the client to display the
/// list of recent projects in the UI.
#[tauri::command]
pub async fn list_recent_projects(app: AppHandle) -> Result<Vec<RecentProjectMetadata>, String> {
    RecentProjectMetadata::get(app).await
}

/// Adds a recently opened project.
#[tauri::command]
pub async fn add_recent_project(
    app: AppHandle,
    metadata: RecentProjectMetadata,
) -> Result<(), String> {
    let mut recent_projects = RecentProjectMetadata::get(app.clone())
        .await
        .unwrap_or_default();
    recent_projects.retain(|p| p.path != metadata.path);
    recent_projects.push(metadata);
    RecentProjectMetadata::save(app, recent_projects).await
}

/// Removes a recently opened project.
#[tauri::command]
pub async fn remove_recent_project(app: AppHandle, path: PathBuf) -> Result<(), String> {
    let mut recent_projects = RecentProjectMetadata::get(app.clone())
        .await
        .unwrap_or_default();
    recent_projects.retain(|p| p.path != path);
    RecentProjectMetadata::save(app, recent_projects).await
}
