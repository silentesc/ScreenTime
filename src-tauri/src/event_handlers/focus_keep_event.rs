use crate::utils::static_manager;

/**
 * Function to be called when the focus is kept on the same window
 * Arguments:
 *   pid - The process ID of the foreground window
 *   process_name - The name of the process that has focus
 *   path - The path of the process that has focus (executable)
 *   keep_millis - The time in milliseconds the focus was kept
 *   system - A reference to the sysinfo::System object to get process details
 * Returns:
 *   None
 */
pub fn on_focus_keep(_pid: u32, _process_name: String, path: String, keep_millis: u64, system: &sysinfo::System) {
    let mut screen_time_apps = static_manager::get_screen_time_apps();

    // Increase the foreground time of the app in focus
    if let Some(screen_time_app) = screen_time_apps.get_mut(path.as_str()) {
        screen_time_app.add_millis_in_foreground(keep_millis);
        static_manager::insert_screen_time_app(screen_time_app.clone());
    }

    // Loop through running processes and increase the background time of the apps that are not in focus
    let mut increased_apps: Vec<String> = Vec::new();

    for (_, process) in system.processes() {
        // Get the path of the process
        if let Some(process_path) = process.exe() {
            let process_path_str = process_path.to_str().unwrap().to_string();
            // Skip the process that has focus
            if process_path_str == path {
                continue;
            }
            // Skip the process if it's already been increased, one app can have multiple processes
            if increased_apps.contains(&process_path_str) {
                continue;
            }
            // Increase the background time of the app
            if let Some(screen_time_app) = screen_time_apps.get_mut(process_path_str.as_str()) {
                screen_time_app.add_millis_in_background(keep_millis);
                static_manager::insert_screen_time_app(screen_time_app.clone());
                increased_apps.push(process_path_str);
            }
        }
    }
}
