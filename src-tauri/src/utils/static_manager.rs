use std::collections::HashMap;
use std::sync::{Arc, Mutex, TryLockError};

use crate::classes::screen_time_app::ScreenTimeApp;
use std::thread;
use std::time::Duration;

lazy_static::lazy_static!(
    static ref SCREEN_TIME_APPS: Arc<Mutex<HashMap<String, ScreenTimeApp>>> = Arc::new(Mutex::new(HashMap::new()));
);

pub fn get_screen_time_apps() -> HashMap<String, ScreenTimeApp> {
    match SCREEN_TIME_APPS.try_lock() {
        Ok(screen_time_apps) => screen_time_apps.clone(),
        Err(err) => match err {
            TryLockError::WouldBlock => {
                thread::sleep(Duration::from_millis(10));
                get_screen_time_apps()
            }
            TryLockError::Poisoned(_) => {
                return HashMap::new();
            }
        },
    }
}

pub fn get_screen_time_app(path: &str) -> Option<ScreenTimeApp> {
    get_screen_time_apps().get(path).cloned()
}

pub fn insert_screen_time_app(app: ScreenTimeApp) -> HashMap<String, ScreenTimeApp> {
    match SCREEN_TIME_APPS.try_lock() {
        Ok(mut screen_time_apps) => {
            screen_time_apps.insert(app.get_path().to_string(), app);
            drop(screen_time_apps);
            get_screen_time_apps()
        }
        Err(err) => match err {
            TryLockError::WouldBlock => {
                thread::sleep(Duration::from_millis(10));
                get_screen_time_apps()
            }
            TryLockError::Poisoned(_) => {
                return HashMap::new();
            }
        },
    }
}

pub fn remove_screen_time_app(path: &str, only_first: bool) -> HashMap<String, ScreenTimeApp> {
    match SCREEN_TIME_APPS.try_lock() {
        Ok(mut screen_time_apps) => {
            if only_first {
                screen_time_apps.remove_entry(path);
            } else {
                screen_time_apps.remove(path);
            }
            drop(screen_time_apps);
            get_screen_time_apps()
        }
        Err(err) => match err {
            TryLockError::WouldBlock => {
                thread::sleep(Duration::from_millis(10));
                get_screen_time_apps()
            }
            TryLockError::Poisoned(_) => {
                return HashMap::new();
            }
        },
    }
}
