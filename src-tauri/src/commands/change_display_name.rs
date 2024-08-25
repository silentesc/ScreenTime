use crate::utils::static_manager;

pub fn change_display_name(app_path: &str, new_display_name: &str) -> bool {
    let mut screen_time_apps = static_manager::get_screen_time_apps();
    let mut success = false;

    for screen_time_app in screen_time_apps.values_mut() {
        if screen_time_app.get_path() == app_path {
            screen_time_app.set_display_name(String::from(new_display_name));
            static_manager::remove_screen_time_app(app_path, true);
            static_manager::insert_screen_time_app(screen_time_app.clone());
            success = true;
            break;
        }
    }

    success
}
