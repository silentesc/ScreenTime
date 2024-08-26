use crate::classes::screen_time_app;
use crate::utils::static_manager;

pub fn get_screen_time_apps_sorted(date: &str, sort_mode: &str, reversed: bool) -> Vec<screen_time_app::ScreenTimeApp> {
    let mut screen_time_apps: Vec<screen_time_app::ScreenTimeApp> =
        static_manager::get_screen_time_apps().into_values().collect();

    match sort_mode {
        "millis_in_foreground" => {
            screen_time_apps.sort_by_key(|screen_time_app| screen_time_app.get_millis_in_foreground(date));
            screen_time_apps.retain(|screen_time_app| screen_time_app.get_millis_in_foreground(date) > 0);
        }
        "millis_in_background" => {
            screen_time_apps.sort_by_key(|screen_time_app| screen_time_app.get_millis_in_background(date));
            screen_time_apps.retain(|screen_time_app| screen_time_app.get_millis_in_background(date) > 0);
        }
        "times_opened" => {
            screen_time_apps.sort_by_key(|screen_time_app| screen_time_app.get_times_opened(date));
            screen_time_apps.retain(|screen_time_app| screen_time_app.get_times_opened(date) > 0);
        }
        "times_focused" => {
            screen_time_apps.sort_by_key(|screen_time_app| screen_time_app.get_times_focused(date));
            screen_time_apps.retain(|screen_time_app| screen_time_app.get_times_focused(date) > 0);
        }
        _ => {
            return Vec::new();
        }
    }

    if reversed {
        screen_time_apps.reverse();
    }

    screen_time_apps.retain(|app| !app.is_hidden());

    screen_time_apps
}
