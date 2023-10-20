use std::io::{self, Write};
use std::{fmt::Display, cell::RefCell};
use notify_rust::{Hint, Urgency, Notification};
use serde::{Serialize, Deserialize};
use zbus::export::futures_util::FutureExt;
use zbus;

use crate::status::Status;
use crate::file::File;

pub enum Work {
    Done,
    Fail,
    UnKnown
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: usize,
    pub title: String,
    pub body: String,
    pub icon: String
}

impl Task {
    /// Create new task struct
    pub fn new(title: &String, body: &String, icon: &String) -> Self {
        // The task id we should get calculating the length of elements
        let id = File::task_file_length() + 1;
        Self {
            id,
            title: title.to_string(),
            body: body.to_string(),
            icon: icon.to_string()
        }
    }
    /// User create a new task in its task list
    pub fn generate() {
        let task_messages = [
            "Type your task name",
            "Add the description of the task",
            "Task related icon"
        ];

        let mut task_values = Vec::<String>::new();

        for &message in task_messages.iter() {
            print!("{:#}: ", message);
            io::stdout().flush().unwrap();
            let mut name = String::new();
            io::stdin()
                .read_line(&mut name)
                .expect("Failed to read task name");
            // Delete the start/end spaces and parse back to String
            let name: String = name.trim().parse().unwrap();
            task_values.push(name);
        }
        
        let slice: &[String] = task_values.as_slice();
        // Important to do that one. If not in runtime might break the program
        if let [title, body, icon] = slice {
            let new_task = Task::new(title, body, icon);
            File::add_new_task(new_task);
        }
    }

    /// Run the daily notification to check the completion of each task
    pub fn run_task_notification() {
        let daily_tasks:RefCell<Vec<Status>> = RefCell::new(Vec::new());
        let task_collection = File::read_task_file();
        if task_collection.len() == 0 {
            println!("🧭 Task compass has been initialise without any defined task. First you need to create the tasks")
        } else {
            println!("⏳ Waiting the user to reply the notifications...");
            for task in task_collection.iter() {
                zbus::block_on(async {
                    task.chech_task_done(&daily_tasks).await;
                });
            }
            File::save_daily_tasks(&daily_tasks);
            println!("✅ Notification interaction finished!")
        }
    }

    ///  Create a notification and wait the user interaction. An asynchronous operation
    pub async fn chech_task_done(&self, daily_task: &RefCell<Vec<Status>>) {
        let title = format!("{} {}", self.icon, self.title);
        // Build a notification for the user
        Notification::new()
            .summary(&title)
            .hint(Hint::Urgency(Urgency::Critical))
            .body(&self.body)
            .action("done", "✔")
            .action("fail", "✗")
            .show_async()
            .then(|handle| async move {
                match handle {
                    Ok(handle) => handle.wait_for_action(| action_identifier | {
                        let completed = match action_identifier {
                            "done"  => true,
                            _       => false
                        };
                        let task_status = Status::new(
                            completed,
                            self.id
                        );
                        daily_task.borrow_mut().push(task_status);
                    }),
                    Err(error) => println!("failed to send notification {error}"),
                };
            }).await;
    }

}

// Implement the trait for a better output of the object
impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}: {}", self.icon,self.title, self.body)
    }
}
