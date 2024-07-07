use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::RwLock;

use crate::classes::screen_time_app::ScreenTimeApp;

lazy_static! {
    static ref SCREEN_TIME_APPS: RwLock<HashMap<String, ScreenTimeApp>> =
        RwLock::new(HashMap::new());
}

/**
 * Get a copy of the screen time apps.
 * Args:
 *    None
 * Returns:
 *    A copy of the screen time apps.
 */
pub fn get_screen_time_apps() -> HashMap<String, ScreenTimeApp> {
    SCREEN_TIME_APPS.read().unwrap().clone()
}

/**
 * Get a cloned screen time app by its path.
 * Args:
 *    path: The path of the screen time app.
 * Returns:
 *    The screen time app if it exists, otherwise None.
 */
pub fn get_screen_time_app(path: &str) -> Option<ScreenTimeApp> {
    get_screen_time_apps().get(path).cloned()
}

/**
 * Insert a screen time app.
 * Args:
 *    app: The screen time app to insert.
 * Returns:
 *    The updated screen time apps HashMap.
 */
pub fn insert_screen_time_app(app: ScreenTimeApp) -> HashMap<String, ScreenTimeApp> {
    SCREEN_TIME_APPS.write().unwrap().insert(app.get_path().to_string(), app);
    get_screen_time_apps()
}

/**
 * Remove a screen time app by its path.
 * Args:
 *    path: The path of the screen time app.
 *    only_first: Whether to remove only the first entry or all matching the key.
 * Returns:
 *    The updated screen time apps HashMap.
 */
pub fn remove_screen_time_app(path: &str, only_first: bool) -> HashMap<String, ScreenTimeApp> {
    let mut screen_time_apps = SCREEN_TIME_APPS.write().unwrap();
    if only_first {
        screen_time_apps.remove_entry(path);
    } else {
        screen_time_apps.remove(path);
    }
    get_screen_time_apps()
}
