use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;

use crate::utils::{date_utils, static_manager};

#[derive(Serialize, Deserialize, Clone)]
pub struct ScreenTimeApp {
    display_name: String,
    hidden: bool,
    name: String,
    path: String,
    millis_in_foreground: HashMap<String, u64>,
    millis_in_background: HashMap<String, u64>,
    times_opened: HashMap<String, u64>,
    times_focused: HashMap<String, u64>,
}

impl ScreenTimeApp {
    /**
     * Static methods
     */

    pub fn new(name: String, path: String, hidden: bool) -> ScreenTimeApp {
        ScreenTimeApp {
            display_name: name.clone(),
            hidden,
            name,
            path,
            millis_in_foreground: HashMap::new(),
            millis_in_background: HashMap::new(),
            times_opened: HashMap::new(),
            times_focused: HashMap::new(),
        }
    }

    pub fn get_apps_as_json_string() -> String {
        let screen_time_apps = static_manager::get_screen_time_apps();
        let apps: Vec<&ScreenTimeApp> = screen_time_apps.values().collect();
        serde_json::to_string(&apps).unwrap_or_else(|_| "[]".to_string())
    }

    /**
     * Set the apps from a JSON string and merge similar paths, NOT names
     * Names are not merged here but in the focus_change_event.rs on_focus_change function
     * This is because there can always be an update of an app while ScreenTime is running and we want to merge then
     *
     * @param json_string: String
     * @return void
     */
    pub fn set_apps_from_json_string(json_string: String) {
        let apps: Vec<ScreenTimeApp> = serde_json::from_str(&json_string).unwrap_or_else(|_| vec![]);

        let mut screen_time_apps = static_manager::get_screen_time_apps();
        for app in apps {
            if screen_time_apps.contains_key(app.path.as_str()) {
                screen_time_apps.get_mut(app.path.as_str()).unwrap().merge(&app);
                continue;
            }
            static_manager::insert_screen_time_app(app.clone());
        }
    }

    /**
     * Instance methods
     */

    pub fn merge(&mut self, other: &ScreenTimeApp) {
        for (date, millis) in other.millis_in_foreground.iter() {
            self.millis_in_foreground
                .insert(date.clone(), self.millis_in_foreground.get(date).unwrap_or(&0) + millis);
        }
        for (date, millis) in other.millis_in_background.iter() {
            self.millis_in_background
                .insert(date.clone(), self.millis_in_background.get(date).unwrap_or(&0) + millis);
        }
        for (date, times) in other.times_opened.iter() {
            self.times_opened
                .insert(date.clone(), self.times_opened.get(date).unwrap_or(&0) + times);
        }
        for (date, times) in other.times_focused.iter() {
            self.times_focused
                .insert(date.clone(), self.times_focused.get(date).unwrap_or(&0) + times);
        }
    }

    pub fn add_millis_in_foreground(&mut self, millis: u64) {
        let today_date = date_utils::get_today_date();
        self.millis_in_foreground.insert(
            today_date.clone(),
            self.millis_in_foreground.get(today_date.as_str()).unwrap_or(&0) + millis,
        );
    }

    pub fn add_millis_in_background(&mut self, millis: u64) {
        let today_date = date_utils::get_today_date();
        self.millis_in_background.insert(
            today_date.clone(),
            self.millis_in_background.get(today_date.as_str()).unwrap_or(&0) + millis,
        );
    }

    pub fn increment_times_opened(&mut self) {
        let today_date = date_utils::get_today_date();
        self.times_opened.insert(
            today_date.clone(),
            self.times_opened.get(today_date.as_str()).unwrap_or(&0) + 1,
        );
    }

    pub fn increment_times_focused(&mut self) {
        let today_date = date_utils::get_today_date();
        self.times_focused.insert(
            today_date.clone(),
            self.times_focused.get(today_date.as_str()).unwrap_or(&0) + 1,
        );
    }

    /**
     * Getters
     */

    pub fn get_display_name(&self) -> &str {
        &self.display_name
    }

    pub fn is_hidden(&self) -> bool {
        self.hidden
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn get_millis_in_foreground(&self, date: &str) -> u64 {
        *self.millis_in_foreground.get(date).unwrap_or(&0)
    }

    pub fn get_millis_in_background(&self, date: &str) -> u64 {
        *self.millis_in_background.get(date).unwrap_or(&0)
    }

    pub fn get_times_opened(&self, date: &str) -> u64 {
        *self.times_opened.get(date).unwrap_or(&0)
    }

    pub fn get_times_focused(&self, date: &str) -> u64 {
        *self.times_focused.get(date).unwrap_or(&0)
    }

    /**
     * Setters
     */
    pub fn set_display_name(&mut self, display_name: String) {
        self.display_name = display_name;
    }

    pub fn set_hidden(&mut self, hidden: bool) {
        self.hidden = hidden;
    }

    pub fn set_path(&mut self, path: String) {
        self.path = path;
    }
}
