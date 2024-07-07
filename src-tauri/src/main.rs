// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use utils::storage_utils;

pub mod listeners {
    pub mod focus_change_listener;
    pub mod process_listener;
}

pub mod event_handlers {
    pub mod focus_change_event;
    pub mod focus_keep_event;
    pub mod process_start_event;
    pub mod process_stop_event;
}

pub mod classes {
    pub mod screen_time_app;
}

pub mod utils {
    pub mod date_utils;
    pub mod static_manager;
    pub mod storage_utils;
}

lazy_static::lazy_static! {
    static ref INITIALIZED: Mutex<bool> = Mutex::new(false);
}

#[tauri::command]
fn init_backend() {
    // Check if the backend is already initialized
    let mut initialized = INITIALIZED.lock().unwrap();
    if *initialized {
        println!("Backend already initialized, skipping initialization.");
        return;
    }
    *initialized = true;

    println!("Initializing backend...");

    let data_file_path = storage_utils::get_data_file_path();

    // Read the data from the file
    storage_utils::set_data_from_file(data_file_path.as_str());

    // Save the data to the file
    storage_utils::save_screen_time_apps_to_file(data_file_path.as_str());

    // Start the listeners
    listeners::focus_change_listener::start_focus_change_listener(500);
    listeners::process_listener::start_process_listener(1000);

    println!("Backend initialized");
}

#[tauri::command]
fn save_data() {
    // Save the data to the file
    let data_file_path = storage_utils::get_data_file_path();
    println!("Saving data to file: {}", data_file_path.as_str());
    storage_utils::save_screen_time_apps_to_file(data_file_path.as_str());
    println!("Data saved to file: {}", data_file_path.as_str());
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![init_backend, save_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
