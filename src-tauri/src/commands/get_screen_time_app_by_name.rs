use crate::{classes::screen_time_app, utils::static_manager};

pub fn get_screen_time_app_by_name(app_name: &str, ignore_case: bool) -> Option<screen_time_app::ScreenTimeApp> {
    let screen_time_apps = static_manager::get_screen_time_apps();

    for screen_time_app in screen_time_apps.values() {
        if ignore_case {
            if screen_time_app.get_name().to_lowercase() == app_name.to_lowercase() {
                return Some(screen_time_app.clone());
            }
        } else {
            if screen_time_app.get_name() == app_name {
                return Some(screen_time_app.clone());
            }
        }
    }

    None
}
