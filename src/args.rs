use clap::{arg, Parser, command};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]

/// Clap arguments to set different commands to the application
pub struct Args {
    /// run the daily task notifications
    #[arg(
        long
    )]
    pub init: bool,

    /// operations related with task
    #[arg(
        long
    )]
    pub task: bool,

    /// add new task in the task list. Requires --task parameter
    #[arg(
        long,
        requires = "task"
    )]
    pub add: bool,

    /// show all the tasks order by week. Requires --task parameter
    #[arg(
        long,
        requires = "task"
    )]
    pub resume: bool
}