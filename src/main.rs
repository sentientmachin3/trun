use clap::Parser;
use env_logger;
use log;
mod tasks;
use tasks::tasks::{list, parse, run, Tasks};

#[derive(Parser, Debug)]
#[command(author="sentientmachin3", version, about="Simple tool to run tasks given a config yaml file", long_about = None)]
struct Args {
    /// Name of the task to run
    #[arg(short, long)]
    run: Option<String>,

    /// List the available tasks
    #[arg(short, long)]
    list: Option<String>,
}

fn main() {
    env_logger::init();
    let args = Args::parse();
    log::debug!("Parsed args {:?}", &args);

    let parsed_tasks: Result<tasks::tasks::Tasks, serde_yaml::Error> = parse();
    log::debug!("Parsed tasks: {:?}", &parsed_tasks);
    let tasks: Tasks = match parsed_tasks {
        Ok(t) => t,
        Err(e) => panic!("{}", e),
    };

    if args.run.is_some() {
        let task_name: String = args.run.unwrap();
        let task = &tasks.tasks[&task_name];
        let result = run(&task);
        log::debug!("Ran command {} - exit status {}", &task_name, &result.unwrap())
    }

    if args.list.is_some() {
        return list();
    }
}
