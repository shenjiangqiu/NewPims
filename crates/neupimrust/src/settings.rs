//! This module provides functionality to manage application settings.
//! It includes functions to initialize settings from a file and retrieve them.

use std::{ffi::c_char, sync::Mutex};
use tracing::info;

/// A global mutex-protected optional `Settings` instance.
pub static SETTINGS: Mutex<Option<Settings>> = Mutex::new(None);

/// A struct representing the application settings.
#[repr(C)]
#[derive(Debug, serde::Deserialize)]
pub struct Settings {
    pub fast_read: bool,
    pub fast_icnt: bool,
    pub no_conflict_act_to_gact: bool,
    pub no_conflict_gact_to_act: bool,
}

/// Initializes the settings from a file specified by a C-style string path.
///
/// # Safety
///
/// This function is unsafe because it dereferences a raw pointer.
#[no_mangle]
pub extern "C" fn init_settings_with_file(file_path: *const c_char) {
    let file_path = unsafe { std::ffi::CStr::from_ptr(file_path) };
    let file_path = file_path.to_str().unwrap();
    init_settings_with_file_(file_path);
}

/// Helper function to initialize settings from a file path.
fn init_settings_with_file_(file_path: &str) {
    let settings = std::fs::read_to_string(file_path).unwrap();
    let settings = toml::from_str(&settings).unwrap();
    set_settings(settings);
}

/// Initializes the settings using a default file path ("sjq.toml").
#[no_mangle]
pub extern "C" fn init_settings() {
    let file_path = "sjq.toml";
    init_settings_with_file_(file_path);
}

/// Sets the global settings to the provided `Settings` instance.
fn set_settings(table: Settings) {
    let mut settings = SETTINGS.lock().unwrap();
    info!("set_settings: {:?}", table);
    *settings = Some(table);
}

/// Retrieves the current settings as a pointer to a `Settings` instance.
///
/// Returns a null pointer if the settings have not been initialized.
#[no_mangle]
pub extern "C" fn get_settings() -> *const Settings {
    let settings = SETTINGS.lock().unwrap();
    match &*settings {
        Some(settings) => settings,
        None => std::ptr::null(),
    }
}
