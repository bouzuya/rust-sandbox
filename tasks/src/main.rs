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
    text: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct TasksJson {
    tasks: Vec<Task>,
}

fn tasks_json_path() -> PathBuf {
    let data_dir = dirs::data_dir().unwrap();
    let data_dir = data_dir.join("net.bouzuya.rust-sandbox.tasks");
    if !data_dir.exists() {
        fs::create_dir(data_dir.as_path()).unwrap();
    }
    data_dir.join("tasks.json")
}

fn read_tasks_json(path: &Path) -> TasksJson {
    let json_string = if path.exists() {
        fs::read_to_string(path).unwrap()
    } else {
        r#"{"tasks":[]}"#.to_owned()
    };
    serde_json::from_str(json_string.as_str()).unwrap()
}

fn write_tasks_json(path: &Path, json: &TasksJson) {
    let json_string = serde_json::to_string(&json).unwrap();
    fs::write(path, json_string).unwrap();
}

fn main() {
    let opt = Opt::from_args();
    let path = tasks_json_path();
    match opt.sub_command {
        SubCommand::Add { text } => {
            let mut json = read_tasks_json(path.as_path());
            json.tasks.push(Task { done: false, text });
            write_tasks_json(path.as_path(), &json);
        }
        SubCommand::Done { id } => {
            let mut json = read_tasks_json(path.as_path());
            let task = json.tasks.get_mut(id - 1).unwrap();
            task.done = true;
            write_tasks_json(path.as_path(), &json);
        }
        SubCommand::List => {
            let json = read_tasks_json(path.as_path());
            println!(
                "{}",
                json.tasks
                    .iter()
                    .enumerate()
                    .map(|(i, task)| format!(
                        "{} {} {}",
                        i + 1,
                        if task.done { "☑" } else { "☐" },
                        task.text
                    ))
                    .collect::<Vec<String>>()
                    .join("\n")
            );
        }
        SubCommand::Remove { id } => {
            let mut json = read_tasks_json(path.as_path());
            json.tasks.remove(id - 1);
            write_tasks_json(path.as_path(), &json);
        }
    }
}
