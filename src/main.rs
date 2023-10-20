use clap::Parser;
use task_compass::task::Task;
use task_compass::{args::Args, resume::Resume};



fn main() {
    let params = Args::parse();
    match &params {
        args if args.init => Task::run_task_notification(),
        args if args.task => {
            match params {
                args if args.resume => {
                    let timeline = Resume::get_type();
                    Resume::generate(timeline)
                }
                args if args.add => Task::generate(),
                _ => println!("Cannot run task operation, missing arguments. Check --help")
            }
        }
        _ => println!("That params does not exist")
    }
    
}


