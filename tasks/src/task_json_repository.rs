use std::{fs, path::PathBuf};
use tasks::{use_case::TaskRepository, Task};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Tasks {
    next_id: usize,
    tasks: Vec<Task>,
}

pub struct TaskJsonRepository {
    path: PathBuf,
}

impl TaskJsonRepository {
    pub fn new() -> Self {
        let data_dir = dirs::data_dir().unwrap();
        let data_dir = data_dir.join("net.bouzuya.rust-sandbox.tasks");
        if !data_dir.exists() {
            fs::create_dir(data_dir.as_path()).unwrap();
        }
        let path = data_dir.join("tasks.json");
        Self { path }
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

impl TaskRepository for TaskJsonRepository {
    fn create(&self, text: String) {
        let mut tasks = self.read();
        tasks.tasks.push(Task {
            done: false,
            id: tasks.next_id,
            text: text.into(),
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
}
