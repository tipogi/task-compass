use indexmap::IndexMap;
use prettytable::color::{self, Color};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    pub completed: bool,
    pub id: usize,
}

impl Status {
    pub fn new(completed: bool, id: usize) -> Self {
        Self {
            completed,
            id
        }
    }

    /// Color the task status. The color is the background color
    pub fn render(&self) -> Color {
        match self.completed {
            true => color::GREEN,//"✅",
            false => color::RED //"🔴"
        }
    }

    /// Order the status list by week days
    pub fn order_status_by_weekday(status_list: Map<String, Value>) -> IndexMap<String, Vec<Status>> {
        let weekday_order = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
        // HashMap and BTReeMap does not respect the insertion order. IndexMap yes!
        // If we use the other maps, it orders alphabetically and we do no get weekday real order
        let mut ordered_list: IndexMap<String, Vec<Status>> = IndexMap::new();
        for weekday_key in weekday_order.into_iter() {
            let weekday_status = status_list.get(weekday_key);
            match weekday_status {
                Some(status_list) => {
                    let day_tasks:Vec<Status> = serde_json::from_str(&status_list.to_string()).unwrap();
                    ordered_list.insert(weekday_key.to_string(), day_tasks);
                }
                None => {
                    ordered_list.insert(weekday_key.to_string(), Vec::new());
                }
            };
        }
        ordered_list
    }
}

