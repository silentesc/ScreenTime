use sysinfo;

use crate::event_handlers::process_start_event;
use crate::event_handlers::process_stop_event;

/**
 * Starts a thread that listens for new processes and stopped processes
 * Arguments:
 *   None
 * Returns:
 *   None
 */
pub fn start_process_listener(poll_interval_millis: u64) {
    std::thread::spawn(move || {
        listen_for_processes(poll_interval_millis);
    });
}

/**
 * Listens for new processes and stopped processes and calls on_process_start and on_process_stop
 * Arguments:
 *   None
 * Returns:
 *   None
 */
fn listen_for_processes(poll_interval_millis: u64) {
    let mut running_processes: Vec<String> = Vec::new();
    let mut system = sysinfo::System::new_all();

    loop {
        let mut temp_processes: Vec<String> = Vec::new();

        system.refresh_all();

        // Check if process is new
        for (_, process) in system.processes() {
            if !process.exe().is_some() {
                continue;
            }
            let process_name = process.name().replace(".exe", "").to_string();
            let process_path = process.exe().unwrap().to_string_lossy().to_string();

            temp_processes.push(process_path.clone());
            if running_processes.contains(&process_path) {
                continue;
            }

            process_start_event::on_process_start(process_name.clone(), process_path.clone());
            running_processes.push(process_path);
        }

        // Check if process has stopped
        for process_path in running_processes.clone().iter() {
            if !temp_processes.contains(process_path) {
                process_stop_event::on_process_stop(process_path.to_string());
                running_processes.retain(|x| x != process_path);
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(poll_interval_millis));
    }
}
