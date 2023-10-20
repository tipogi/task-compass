use std::{io::{self, Write}, str::FromStr};

use prettytable::{Table, Cell, Row, Attr, color::{self, Color}};

use crate::timeline::{Timeline, Calendar};
use crate::file::{File, week_file_path};
use crate::status::Status;
pub struct Resume {}

impl Resume {
    /// Generate different type of recap based on a timeline
    pub fn generate(timeline: Timeline) {
        match timeline {
            Timeline::Weekly => Self::print_weekly_table(),
            Timeline::Monthly => Self::print_monthly_table()
        }
    }

    /// Output to choose which kind of resume show
    pub fn get_type() -> Timeline {
        println!("To get the resume of your task first choose the timeline:");
        print!("- Weekly (w)\n- Monthly (m)\n💬 ");
        io::stdout().flush().unwrap();
        let mut timeline_buffer = String::new();
        io::stdin()
            .read_line(&mut timeline_buffer)
            .expect("Failed to get the timeline code");
        // Clear the buffer with trim
        let timeline_buffer: String = timeline_buffer.trim().parse().unwrap();
        Timeline::from_str(&timeline_buffer).unwrap()
    }

    /// Print weekly resume
    pub fn print_weekly_table() {
        let calendar = Calendar::new();
        let path = week_file_path(&calendar);
        let week_task = File::read_week_task_status_file(&path);
        let week_task_ordered = Status::order_status_by_weekday(week_task.clone());
        let mut table = Table::new();
        Self::create_table_header(&mut table);
        let tasks_length = File::task_file_length();
        // Loop weekly task status
        for (day, tasks) in week_task_ordered {
            let mut weekday_row: Vec<Cell> = vec!(Cell::new(&day));
            // Create a colored cells for each task to after wrapped in a row
            for task_number in 1..=tasks_length {
                let exist_task_status = tasks.iter().find(|&status| status.id == task_number);
                let color:Color = match exist_task_status {
                    Some(status) => status.render(),
                    // When the task activity does not exist
                    None => color::BLUE
                };
                weekday_row.push(
                    Cell::new("")
                        .with_style(Attr::BackgroundColor(color))
                        .with_style(Attr::Italic(true))
                );
            }
            table.add_row(Row::new(weekday_row));
        }
        table.printstd();
    }

    pub fn print_monthly_table() {
        //TODO
    }
    /// Generate the header with the title of the defined tasks
    fn create_table_header(table: &mut Table) {
        let tasks = File::read_task_file();
        let mut headers:Vec<String> = Vec::new();
        for task in tasks.into_iter() {
            let header = format!("{} {}", task.icon, task.title);
            headers.push(header)
        }
        let mut headers_cell:Vec<Cell> = headers.iter().map(|header| {
            Cell::new(header)
        }).collect();
        headers_cell.insert(0, Cell::new(""));
        table.add_row(Row::new(headers_cell));
    }
}
