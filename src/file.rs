use std::env;
use std::{fs, cell::RefCell, path::Path};
use std::io::{self, ErrorKind};

use serde_json::{Map, Value, json};

use crate::timeline::Calendar;
use crate::{task::Task, status::Status};

const TASK_PATH: &str = "files/import";
const TASK_FILE: &str = "files/import/task.json";

pub struct File {}

impl File {

    // This it was related for cronjob execution. We do not need
    pub fn _get_app_binary_path() {
        match env::current_exe() {
            Ok(exe_path) => println!("Path of this executable is: {}",
                                     exe_path.display()),
            Err(e) => println!("failed to get current exe path: {e}"),
        };
    }

    /// Get the created tasks
    pub fn read_task_file() -> Vec<Task> {
        let result = fs::read_to_string(TASK_FILE);
        let task_file = match result {
            Ok(file) => file,
            Err(e) => error_control(e)
        };
        // Parse the task list from JSON to Vec type
        serde_json::from_str(&task_file).unwrap()
    }

    /// Get all the defined task number
    pub fn task_file_length() -> usize {
        File::read_task_file().len()
    }

    /// Push another task to already existing list
    pub fn add_new_task(new_task: Task) {
        let mut task_list = File::read_task_file();
        task_list.push(new_task);
        let tasks_json = serde_json::to_string(&task_list).unwrap();
        fs::write(TASK_FILE, tasks_json).unwrap();
    }

    /// Having the path of the week task JSON, convert to Map Struct
    pub fn read_week_task_status_file(path: &str) -> Map<String, Value> {
        let task_file = fs::read_to_string(path).unwrap();
        let parsed: Value = serde_json::from_str(&task_file).unwrap();
        parsed.as_object().unwrap().clone()
    }

    /// Add new record in the daily tasks list. That list is the weekly file
    pub fn save_daily_tasks(daily_tasks: &RefCell<Vec<Status>>) {
        let mut map = Map::new();
        let calendar = Calendar::new();
        let week_day = calendar.weekday();
        // Take the ownership of the RefCel value 
        // and add the week day key to the task entries
        map.insert(week_day.to_string(), json!(daily_tasks.take()));
        // Create the path to save the file
        let path = week_file_path(&calendar);
        File::save_file(&path, &mut map);
    }

    /// Edit or create a new weekly file 
    pub fn save_file(path: &str, map: &mut Map<String, Value>) {
        let mut file_exists = false;
        // Check if the file and folders exists before add the new entries
        match Path::new(path).exists() {
            false   => File::create_file_and_parents(path),
            _       => file_exists = true
        }
        let new_status_map = match file_exists {
            false   => map.clone(),
            true    => add_previous_days_tasks(path, map)
        };
        // Convert our Map object into String to save as a JSON file
        let contents = Value::from(new_status_map.clone()).to_string();
        fs::write(path, contents).unwrap();
    }

    /// If there is some folder that does not exist in the path, create new folders
    pub fn create_file_and_parents(path: &str) {
        let native_path = std::path::Path::new(path);
        let prefix = native_path.parent().unwrap();
        std::fs::create_dir_all(prefix).unwrap();
    }
}

/// Get the file of the week to display the resume
pub fn week_file_path(calendar: &Calendar) -> String {
    format!(
        "files/export/{}/week_{}.json", 
        calendar.year(), 
        calendar.week_number()
    )
}

/// Join task status files. Already existing one with the new
fn add_previous_days_tasks(path: &str, map: &mut Map<String, Value>) -> Map<String, Value> {
    let mut week_status_file = File::read_week_task_status_file(path);
    week_status_file.append(map);
    week_status_file
}

/// File error control
fn error_control(e: io::Error) -> String {
    match e.kind() {
        // Create a file/folder if it does not exist
        ErrorKind::NotFound => {
            let task_path = Path::new(TASK_PATH);
            let task_file = Path::new(TASK_FILE);
            println!("🚩 File or path does not exist! Creating...");
            fs::create_dir_all(task_path).unwrap();
            fs::write(task_file, "[]").unwrap();
            "[]".to_string()
        }
        _ => "[]".to_string()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn get_week_number() {
        let daily_tasks:RefCell<Vec<Status>> = RefCell::new(Vec::new());
        File::save_daily_tasks(&daily_tasks);
    }
}