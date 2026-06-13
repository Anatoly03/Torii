use std::error::Error;
use tauri::App;

/// Run the Tauri application. This is the entry point for the desktop application.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(enable_logging)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Enable logging for the application. This will log messages to
/// the console when in debug mode.
pub fn enable_logging(app: &mut App) -> Result<(), Box<dyn Error>> {
    if cfg!(debug_assertions) {
        app.handle().plugin(
            tauri_plugin_log::Builder::default()
                .level(log::LevelFilter::Info)
                .build(),
        )?;
    }
    Ok(())
}
