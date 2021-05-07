use std::{
    fs,
    path::{Path, PathBuf},
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    sub_command: SubCommand,
}

#[derive(Debug, StructOpt)]
enum SubCommand {
    #[structopt(about = "Adds a new task")]
    Add { text: String },
    #[structopt(about = "Completes the task")]
    Done { id: usize },
    #[structopt(about = "Lists tasks")]
    List,
    #[structopt(about = "Removes the task")]
    Remove { id: usize },
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Task {
    done: bool,
    id: usize,
    text: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Tasks {
    next_id: usize,
    tasks: Vec<Task>,
}

struct TaskRepository {
    path: PathBuf,
}

impl TaskRepository {
    fn read(&self) -> Tasks {
        let json_string = if self.path.exists() {
            fs::read_to_string(self.path.as_path()).unwrap()
        } else {
            r#"{"next_id":1,"tasks":[]}"#.to_owned()
        };
        serde_json::from_str(json_string.as_str()).unwrap()
    }

    fn write(&self, tasks: &Tasks) {
        let json_string = serde_json::to_string(tasks).unwrap();
        fs::write(self.path.as_path(), json_string).unwrap();
    }
}

fn tasks_json_path() -> PathBuf {
    let data_dir = dirs::data_dir().unwrap();
    let data_dir = data_dir.join("net.bouzuya.rust-sandbox.tasks");
    if !data_dir.exists() {
        fs::create_dir(data_dir.as_path()).unwrap();
    }
    data_dir.join("tasks.json")
}

fn main() {
    let opt = Opt::from_args();
    let path = tasks_json_path();
    let repository = TaskRepository { path };
    match opt.sub_command {
        SubCommand::Add { text } => {
            let mut json = repository.read();
            json.tasks.push(Task {
                done: false,
                id: json.next_id,
                text,
            });
            json.next_id += 1;
            repository.write(&json);
        }
        SubCommand::Done { id } => {
            let mut json = repository.read();
            let task_position = json.tasks.iter().position(|t| t.id == id).unwrap();
            let task = json.tasks.get_mut(task_position).unwrap();
            task.done = true;
            repository.write(&json);
        }
        SubCommand::List => {
            let json = repository.read();
            println!(
                "{}",
                json.tasks
                    .iter()
                    .map(|task| format!(
                        "{} {} {}",
                        task.id,
                        if task.done { "☑" } else { "☐" },
                        task.text
                    ))
                    .collect::<Vec<String>>()
                    .join("\n")
            );
        }
        SubCommand::Remove { id } => {
            let mut json = repository.read();
            let task_position = json.tasks.iter().position(|t| t.id == id).unwrap();
            json.tasks.remove(task_position);
            repository.write(&json);
        }
    }
}
