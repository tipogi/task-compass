use std::{str::FromStr, process};

use chrono::{Weekday, Datelike, Local, DateTime};

#[derive(Debug)]
pub enum Timeline {
    Weekly,
    Monthly
}

impl FromStr for Timeline {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, ()> {
        println!("{:#?}", input);
        match input {
            "w" => Ok(Timeline::Weekly),
            "m" => Ok(Timeline::Monthly),
            _ => {
                println!("🛑 The selected timeline does not exist. Choose from the selected list the keyword");
                process::exit(1)
            }
        }
    }
}

pub struct Calendar {
    now: DateTime<Local>
}

impl Calendar {
    // Create a new calendar
    pub fn new() -> Self {
        Self {
            now: chrono::offset::Local::now()
        }
    }

    /// Actual date
    pub fn now(&self) -> DateTime<Local> {
        self.now
    }

    /// Actual calendar year
    pub fn year(&self) -> i32 {
        self.now.year()
    }

    /// Actual calendar week number
    pub fn week_number(&self) -> u32 {
        self.now.iso_week().week()
    }

    /// Actual calendar weekday
    pub fn weekday(&self) -> Weekday {
        self.now.weekday()
    }
}