// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use single_instance::SingleInstance;
use std::sync::Mutex;
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};

use utils::{static_manager, storage_utils};

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

fn save_data() {
    // Save the data to the file
    let data_file_path = storage_utils::get_data_file_path();
    println!("Saving data to file: {}", data_file_path.as_str());
    storage_utils::save_screen_time_apps_to_file(data_file_path.as_str());
    println!("Data saved to file: {}", data_file_path.as_str());
}

fn init_backend() {
    // Check if the backend is already initialized
    let mut initialized = INITIALIZED.lock().unwrap();
    if *initialized {
        println!("Backend already initialized, skipping initialization.");
        return;
    }
    *initialized = true;

    println!("Initializing backend...");

    static_manager::init();

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

fn main() {
    // Single instance setup
    let instance = SingleInstance::new("screen-time").unwrap();

    if instance.is_single() {
        let tray_menu = SystemTrayMenu::new()
            .add_item(CustomMenuItem::new("show".to_string(), "Show"))
            .add_item(CustomMenuItem::new("quit".to_string(), "Quit"));
        let system_tray = SystemTray::new().with_menu(tray_menu);

        tauri::Builder::default()
            .setup(move |app| {
                let window = app.get_window("main").unwrap();

                // Hide the window instead of closing
                let window_clone = window.clone();
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        api.prevent_close();
                        window_clone.hide().unwrap();
                    }
                });

                // Initialize the backend
                init_backend();

                Ok(())
            })
            .system_tray(system_tray)
            .on_system_tray_event(|app, event| match event {
                SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                    "show" => {
                        let window = app.get_window("main").unwrap();
                        window.show().unwrap();
                        window.set_focus().unwrap();
                    }
                    "quit" => {
                        save_data();
                        let window = app.get_window("main").unwrap();
                        window.close().unwrap();
                    }
                    _ => {}
                },
                _ => {}
            })
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    } else {
        // Exit the current instance
        std::process::exit(0);
    }
}
