// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use classes::screen_time_app;
use single_instance::SingleInstance;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
};
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};

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

pub mod commands {
    pub mod get_screen_time_app_by_name;
    pub mod get_screen_time_apps_sorted;
}

lazy_static::lazy_static! {
    static ref INITIALIZED: Mutex<bool> = Mutex::new(false);
}

/**
 *
 * Tauri commands
 *
 */

#[tauri::command]
fn get_today_date() -> String {
    utils::date_utils::get_today_date()
}

#[tauri::command]
fn get_specific_date(day: u32, month: u32, year: i32) -> String {
    utils::date_utils::get_specific_date(day, month, year)
}

#[tauri::command]
fn get_screen_time_apps_sorted(date: &str, sort_mode: &str, reversed: bool) -> Vec<screen_time_app::ScreenTimeApp> {
    commands::get_screen_time_apps_sorted::get_screen_time_apps_sorted(date, sort_mode, reversed)
}

#[tauri::command]
fn get_screen_time_app_by_name(app_name: &str, ignore_case: bool) -> Option<screen_time_app::ScreenTimeApp> {
    commands::get_screen_time_app_by_name::get_screen_time_app_by_name(app_name, ignore_case)
}

/**
 *
 * Backend stuff
 *
 */

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

    // Read the data from the file
    let data_file_path = storage_utils::get_data_file_path();
    storage_utils::set_data_from_file(data_file_path.as_str());

    // Start the listeners
    listeners::focus_change_listener::start_focus_change_listener(500);
    listeners::process_listener::start_process_listener(1000);

    // Save the data every 5 minutes
    std::thread::spawn(move || loop {
        save_data();
        std::thread::sleep(std::time::Duration::from_secs(60 * 5));
    });

    println!("Backend initialized");
}

fn hide_window(window: &tauri::Window) {
    window.minimize().unwrap();
    std::thread::sleep(std::time::Duration::from_millis(150));
    window.hide().unwrap();
}

fn show_window(window: &tauri::Window) {
    window.show().unwrap();
    std::thread::sleep(std::time::Duration::from_millis(150));
    window.unminimize().unwrap();
    window.set_focus().unwrap();
}

/**
 *
 * Main function
 *
 */

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
                        hide_window(&window_clone);
                    }
                });

                // Listen for other instances
                let window_mutex = Arc::new(Mutex::new(window));
                let tcp_listener = TcpListener::bind("127.0.0.1:50505").expect("Failed to bind listener");
                std::thread::spawn(move || {
                    for stream in tcp_listener.incoming() {
                        match stream {
                            Ok(mut stream) => {
                                let mut buffer = [0; 128];
                                match stream.read(&mut buffer) {
                                    Ok(0) => {
                                        println!("Received empty message, ignoring.");
                                        continue;
                                    }
                                    Ok(bytes_read) => {
                                        let message = String::from_utf8_lossy(&buffer[..bytes_read]).trim().to_string();
                                        if message == "show".to_string() {
                                            let window = window_mutex.lock().unwrap();
                                            show_window(&window);
                                        }

                                        stream.shutdown(std::net::Shutdown::Both).unwrap();
                                    }
                                    Err(e) => {
                                        if e.kind() == std::io::ErrorKind::Interrupted {
                                            println!("Operation interrupted, shutting down.");
                                            break;
                                        } else {
                                            println!("Failed to read message, ignoring. Error: {}", e);
                                            continue;
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                if e.kind() == std::io::ErrorKind::Interrupted {
                                    println!("Accept operation interrupted, shutting down.");
                                    break;
                                } else {
                                    println!("Failed to accept connection, ignoring. Error: {}", e);
                                    continue;
                                }
                            }
                        }
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
                        show_window(&window);
                    }
                    "quit" => {
                        save_data();
                        let window = app.get_window("main").unwrap();
                        window.close().unwrap();
                        app.exit(0);
                    }
                    _ => {}
                },
                _ => {}
            })
            .invoke_handler(tauri::generate_handler![
                get_screen_time_apps_sorted,
                get_today_date,
                get_specific_date,
                get_screen_time_app_by_name
            ])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    } else {
        // Send a message to the other instance to show the window
        let mut stream = TcpStream::connect("127.0.0.1:50505").expect("Failed to connect to listener");
        stream.write("show".as_bytes()).unwrap();
        stream.flush().unwrap();
        stream.shutdown(std::net::Shutdown::Both).unwrap();

        println!("Another instance is already running, sending message to show window.");

        // Exit the current instance
        std::process::exit(0);
    }
}
