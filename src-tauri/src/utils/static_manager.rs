use std::collections::HashMap;

use crate::classes::screen_time_app::ScreenTimeApp;

static mut SCREEN_TIME_APPS: Option<HashMap<String, ScreenTimeApp>> = None;

pub fn init() {
    unsafe {
        if SCREEN_TIME_APPS.is_none() {
            SCREEN_TIME_APPS = Some(HashMap::new());
        }
    }
}

pub fn get_screen_time_apps() -> HashMap<String, ScreenTimeApp> {
    unsafe {
        SCREEN_TIME_APPS.as_ref().unwrap().clone()
    }
}

pub fn get_screen_time_app(path: &str) -> Option<ScreenTimeApp> {
    get_screen_time_apps().get(path).cloned()
}

pub fn insert_screen_time_app(app: ScreenTimeApp) -> HashMap<String, ScreenTimeApp> {
    unsafe {
        SCREEN_TIME_APPS.as_mut().unwrap().insert(app.get_path().to_string(), app);
    }
    get_screen_time_apps()
}

pub fn remove_screen_time_app(path: &str, only_first: bool) -> HashMap<String, ScreenTimeApp> {
    unsafe {
        let screen_time_apps = SCREEN_TIME_APPS.as_mut().unwrap();
        if only_first {
            screen_time_apps.remove_entry(path);
        } else {
            screen_time_apps.remove(path);
        }
    }
    get_screen_time_apps()
}
