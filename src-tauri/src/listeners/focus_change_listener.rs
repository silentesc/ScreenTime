use windows_sys::Win32::Foundation::HWND;
use windows_sys::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId};

use crate::event_handlers::{focus_change_event, focus_keep_event};

/**
 * Starts a thread that listens for changes in the foreground window
 * Arguments:
 *  None
 * Returns:
 *   None
 */
pub fn start_focus_change_listener(poll_interval_millis: u64) {
    std::thread::spawn(move || {
        listen_for_focus_change(poll_interval_millis);
    });
}

/**
 * Listens for changes in the foreground window and calls the on_focus_change function when the focus changes
 * Arguments:
 *  None
 * Returns:
 *   None
 */
fn listen_for_focus_change(poll_interval_millis: u64) {
    let mut system = sysinfo::System::new_all();
    let mut last_pid: u32 = 0;
    let mut last_process_name: String = String::new();
    let mut last_path: String = String::new();

    loop {
        // Get the PID of the foreground process
        let mut foreground_pid: u32 = 0;
        unsafe {
            let foreground_window_hwnd: HWND = GetForegroundWindow();
            GetWindowThreadProcessId(foreground_window_hwnd, &mut foreground_pid);
        }

        // Check if the PID is 0 (no foreground process found) and return
        if foreground_pid == 0 {
            println!("No foreground process found");
            std::thread::sleep(std::time::Duration::from_millis(poll_interval_millis));
            continue;
        }
        // Check if the PID is the same as the last PID (process didn't change) and return
        if foreground_pid == last_pid {
            focus_keep_event::on_focus_keep(
                foreground_pid,
                last_process_name.clone(),
                last_path.clone(),
                poll_interval_millis,
                &system,
            );
            std::thread::sleep(std::time::Duration::from_millis(poll_interval_millis));
            continue;
        }

        /* If we are here, the foreground process has changed */

        // Get the process details from the PID
        system.refresh_all();
        let mut found = false;
        for (pid, process) in system.processes() {
            if process.exe().is_some() && pid.as_u32() == foreground_pid {
                found = true;
                focus_change_event::on_focus_change(
                    pid.as_u32(),
                    String::from(process.name()),
                    process.exe().unwrap().to_string_lossy().to_string(),
                    last_pid,
                    last_process_name.clone(),
                    last_path.clone(),
                    poll_interval_millis,
                    &system,
                );
                last_pid = pid.as_u32();
                last_process_name = String::from(process.name());
                last_path = process.exe().unwrap().to_string_lossy().to_string();
                break;
            }
        }
        if !found {
            println!("Process details not found for PID: {}", foreground_pid);
            std::thread::sleep(std::time::Duration::from_millis(poll_interval_millis));
            continue;
        }

        // Wait before the next iteration
        std::thread::sleep(std::time::Duration::from_millis(poll_interval_millis));
    }
}
