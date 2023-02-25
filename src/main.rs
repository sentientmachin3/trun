use clap::{Arg, ArgAction, Command};
use env_logger;
use log;
mod tasks;
use tasks::tasks::{list, parse, run, Tasks};

fn main() {
    env_logger::init();
    let mut command = build_cli_command();
    let matches = command.get_matches_mut();

    let cli_config = matches.get_one("config").unwrap();
    let parsed_tasks = parse(&cli_config);
    log::debug!("Parsed tasks: {:?}", &parsed_tasks);

    let tasks: Tasks = match parsed_tasks {
        Ok(t) => t,
        Err(e) => panic!("{}", e),
    };

    let run_arg = matches.get_one::<String>("run");
    if run_arg.is_some() {
        let task_name = run_arg.unwrap().to_string();
        let task = &tasks.tasks[&task_name];
        let result = run(&task);
        log::debug!(
            "Ran command {} - {}",
            &task_name,
            &result.unwrap().to_string()
        )
    }

    let list_arg = matches.get_one::<bool>("list");
    if list_arg.is_some() {
        return list(&tasks);
    }

    match command.print_help() {
        Ok(_) => (),
        Err(e) => log::error!("{}", e)
    };
}

fn build_cli_command() -> Command {
    return Command::new("trun")
        .arg(
            Arg::new("config")
                .short('c')
                .help("Set the config file path")
                .default_value("./trun.yaml"),
        )
        .arg(
            Arg::new("run")
                .short('r')
                .help("Run task in the trun.yaml file"),
        )
        .arg(
            Arg::new("list")
                .short('l')
                .help("List available commands as provided in trun.yaml file")
                .action(ArgAction::SetTrue),
        )
        .arg_required_else_help(true);
}
