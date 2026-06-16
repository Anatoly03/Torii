//! Main entry point for the Torii desktop application, built using Tauri.

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

/// The main entry point for the desktop application. This function is called
/// when the application stars.
fn main() {
    #[cfg(dev)]
    set_desktop_env(":0.0");

    app_lib::run();
}

/// Sets the desktop environment variables for development. This function exists
/// because code maintainer works with a headless ssh environment and needs to
/// export the `$DISPLAY` variable to run the application.
#[cfg(dev)]
fn set_desktop_env(fallback_value: &str) {
    unsafe {
        use std::env::{VarError::NotPresent, set_var, var};

        match var("DISPLAY") {
            Err(NotPresent) => set_var("DISPLAY", fallback_value),
            _ => (),
        }
    }
}
