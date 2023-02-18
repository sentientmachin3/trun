pub mod tasks {
    use serde::Deserialize;
    use serde_yaml;
    use std::fs::File;
    use std::process::{Command, ExitStatus};

    #[derive(Debug, Deserialize)]
    pub struct Task {
        name: String,
        description: String,
        command: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct Tasks {
        pub tasks: std::collections::HashMap<String, Task>,
    }

    pub fn parse() -> Result<Tasks, serde_yaml::Error> {
        log::debug!("Parsing tasks from file");
        let file = File::open("./tasks.yaml").expect("Unable to open file");
        return serde_yaml::from_reader(file);
    }

    pub fn run(task: &Task) -> Result<ExitStatus, std::io::Error> {
        log::info!("Running command {:?}", &task.name);
        return Command::new("sh").arg("-c").arg(&task.command).status();
    }

    pub fn list() {}
}
