pub mod tasks {
    use serde::Deserialize;
    use serde_yaml;
    use std::fs::File;
    use std::process::{Command, ExitStatus};

    #[derive(Debug, Deserialize)]
    pub struct Task {
        description: String,
        command: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct Tasks {
        pub tasks: std::collections::HashMap<String, Task>,
    }

    pub fn parse(file_path: &String) -> Result<Tasks, serde_yaml::Error> {
        log::debug!("Parsing tasks from file");
        let file = File::open(file_path).expect("Unable to open file");
        return serde_yaml::from_reader(file);
    }

    pub fn run(task: &Task) -> Result<ExitStatus, std::io::Error> {
        return Command::new("sh").arg("-c").arg(&task.command).status();
    }

    pub fn list(tasks: &Tasks) {
        for t in tasks.tasks.iter() {
            println!("{1: <20}{}", &t.0, &t.1.description);
        }
    }
}
