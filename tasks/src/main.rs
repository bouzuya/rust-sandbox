use std::{fs, path::PathBuf};
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
    fn create(&self, text: String) {
        let mut tasks = self.read();
        tasks.tasks.push(Task {
            done: false,
            id: tasks.next_id,
            text,
        });
        tasks.next_id += 1;
        self.write(&tasks);
    }

    fn delete(&self, id: usize) {
        let mut tasks = self.read();
        let task_position = tasks.tasks.iter().position(|t| t.id == id).unwrap();
        tasks.tasks.remove(task_position);
        self.write(&tasks);
    }

    fn find_all(&self) -> Vec<Task> {
        let tasks = self.read();
        tasks.tasks
    }

    fn find_by_id(&self, id: usize) -> Option<Task> {
        let tasks = self.read();
        tasks.tasks.into_iter().find(|t| t.id == id)
    }

    fn save(&self, task: Task) {
        let mut tasks = self.read();
        let task_position = tasks.tasks.iter().position(|t| t.id == task.id).unwrap();
        let task = tasks.tasks.get_mut(task_position).unwrap();
        task.done = true;
        self.write(&tasks);
    }

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
            repository.create(text);
        }
        SubCommand::Done { id } => {
            let mut task = repository.find_by_id(id).unwrap();
            task.done = true;
            repository.save(task);
        }
        SubCommand::List => {
            let tasks = repository.find_all();
            println!(
                "{}",
                tasks
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
            repository.delete(id);
        }
    }
}
