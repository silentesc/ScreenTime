use crate::{classes::screen_time_app, utils::static_manager};

pub fn get_screen_time_apps() -> Vec<screen_time_app::ScreenTimeApp> {
    let screen_time_apps: Vec<screen_time_app::ScreenTimeApp> = static_manager::get_screen_time_apps().values().cloned().collect();
    let mut mut_apps = screen_time_apps.clone();
    mut_apps.sort_by(|a, b| a.get_display_name().to_lowercase().cmp(&b.get_display_name().to_lowercase()));
    mut_apps
}
