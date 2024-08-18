use crate::utils::static_manager;

/**
 * Called when a new process is started
 * Arguments:
 *   process_name: String - The name of the process that was started
 *   path: String - The .exe path of the process that was started
 * Returns:
 *   None
 */
pub fn on_process_start(_process_name: String, path: String) {
    let mut screen_time_apps = static_manager::get_screen_time_apps();

    // Increase the times opened
    if let Some(screen_time_app) = screen_time_apps.get_mut(path.as_str()) {
        screen_time_app.increment_times_opened();
        static_manager::insert_screen_time_app(screen_time_app.clone());
    }
}
