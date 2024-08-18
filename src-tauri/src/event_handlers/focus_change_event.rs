use crate::{classes::screen_time_app, event_handlers::focus_keep_event::on_focus_keep, utils::static_manager};

/**
 * Function to be called when the focus changes
 * Arguments:
 *   pid - The process ID of the foreground window
 *   process_name - The name of the process that has focus
 *   path - The path of the process that has focus (executable)
 *   last_pid - The process ID of the last foreground window
 *   last_process_name - The name of the process that had focus
 *   last_path - The path of the process that had focus (executable)
 *   poll_interval_millis - The time in milliseconds between each poll
 *   system - A reference to the sysinfo::System object to get process details
 * Returns:
 *   None
 */
pub fn on_focus_change(
    _pid: u32,
    process_name: String,
    path: String,
    last_pid: u32,
    last_process_name: String,
    last_path: String,
    poll_interval_millis: u64,
    system: &sysinfo::System,
) {
    // Call the on_focus_keep function because the last process had the focus until now
    on_focus_keep(last_pid, last_process_name, last_path, poll_interval_millis, system);

    let mut screen_time_apps = static_manager::get_screen_time_apps();

    // Code below only runs if a screen_time_app with the path doesn't exist
    // It will either merge existing apps with the same name or create a new one
    if !screen_time_apps.contains_key(path.as_str()) {
        // Get all apps with similar name and different path
        let mut apps_to_merge = Vec::new();
        for (old_path, screen_time_app) in screen_time_apps.iter() {
            if screen_time_app.get_name() == process_name && old_path.to_string() != path {
                apps_to_merge.push((old_path.clone(), path.clone(), screen_time_app.clone()));
            }
        }

        /*
        If there are any apps with similar name but different path to merge, we merge them
        Otherwise, we create a new ScreenTimeApp because the app is new
        NOTE: Same paths are merged when app starts, not here because it's impossible to have the same path with different names during runtime
         */
        if apps_to_merge.len() > 0 {
            for (old_path, new_path, mut old_screen_time_app) in apps_to_merge {
                // Remove old app
                screen_time_apps = static_manager::remove_screen_time_app(old_path.as_str(), true);

                // Merge apps and attributes if the new path has been inserted already
                if screen_time_apps.contains_key(new_path.as_str()) {
                    let new_screen_time_app = screen_time_apps.get_mut(new_path.as_str()).unwrap();
                    new_screen_time_app.merge(&old_screen_time_app);
                }
                // Update the path of the app and insert it again
                else {
                    old_screen_time_app.set_path(new_path.clone());
                    static_manager::insert_screen_time_app(old_screen_time_app);
                }
            }
        } else {
            let new_app = screen_time_app::ScreenTimeApp::new(process_name.clone(), path.clone());
            static_manager::insert_screen_time_app(new_app);
        }
    }

    // Increase the focus time of the new process in focus
    if let Some(screen_time_app) = screen_time_apps.get_mut(path.as_str()) {
        screen_time_app.increment_times_focused();
        static_manager::insert_screen_time_app(screen_time_app.clone());
    }
}
