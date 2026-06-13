//! Main entry point for the Torii desktop application, built using Tauri.

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

/// The main entry point for the desktop application. This function is called
/// when the application stars.
fn main() {
    app_lib::run();
}
