//! Desktop library for the Torii desktop application, built using Tauri.

pub mod components;
pub mod project;
pub mod recent;
pub mod record;
pub mod workspace;

pub use components::Component;
pub use record::Record;
pub use workspace::Workspace;

use std::error::Error;
use tauri::App;

/// Run the Tauri application. This is the entry point for the desktop application.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_cors_fetch::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(enable_logging)
        .invoke_handler(tauri::generate_handler![
            recent::list_recent_projects,
            recent::add_recent_project,
            recent::remove_recent_project,
            record::commands::list_records,
            record::commands::rename_record,    
            record::commands::remove_record,
            record::commands::list_record_components,
            record::commands::get_record_component,
            record::commands::save_record_component,
            record::commands::save_record_component_from_local_file,
            record::commands::remove_record_component,
            project::read_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Enable logging for the application. This will log messages to
/// the console when in debug mode.
pub fn enable_logging(app: &mut App) -> Result<(), Box<dyn Error>> {
    #[cfg(debug_assertions)]
    {
        app.handle().plugin(
            tauri_plugin_log::Builder::default()
                .level(log::LevelFilter::Info)
                .build(),
        )?;
    }
    Ok(())
}
