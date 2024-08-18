use dirs::config_dir;
use std::fs;

use crate::classes::screen_time_app;

pub fn get_data_file_path() -> String {
    // Get the path to the data file
    let mut data_file_path = String::new();
    if let Some(roaming_dir) = config_dir() {
        let mut path = roaming_dir;
        path.push("ScreenTime");

        if !path.exists() {
            std::fs::create_dir_all(&path).unwrap();
        }

        path.push("apps.json");
        data_file_path = path.to_str().unwrap().to_string();
    }
    data_file_path
}

pub fn set_data_from_file(file_name: &str) {
    if !std::path::Path::new(file_name).exists() {
        return;
    }
    let data = fs::read_to_string(file_name).expect("Unable to read file.");
    screen_time_app::ScreenTimeApp::set_apps_from_json_string(data);
}

pub fn save_screen_time_apps_to_file(file_name: &str) {
    let data = screen_time_app::ScreenTimeApp::get_apps_as_json_string();
    fs::write(file_name, data).expect("Unable to write file.");
}
