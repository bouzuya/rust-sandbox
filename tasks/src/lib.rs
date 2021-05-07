use std::{fs, path::PathBuf};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Task {
    pub done: bool,
    pub id: usize,
    pub text: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Tasks {
    next_id: usize,
    tasks: Vec<Task>,
}

pub struct TaskRepository {
    path: PathBuf,
}

impl TaskRepository {
    pub fn new() -> Self {
        let path = tasks_json_path();
        TaskRepository { path }
    }

    pub fn create(&self, text: String) {
        let mut tasks = self.read();
        tasks.tasks.push(Task {
            done: false,
            id: tasks.next_id,
            text,
        });
        tasks.next_id += 1;
        self.write(&tasks);
    }

    pub fn delete(&self, id: usize) {
        let mut tasks = self.read();
        let task_position = tasks.tasks.iter().position(|t| t.id == id).unwrap();
        tasks.tasks.remove(task_position);
        self.write(&tasks);
    }

    pub fn find_all(&self) -> Vec<Task> {
        let tasks = self.read();
        tasks.tasks
    }

    pub fn find_by_id(&self, id: usize) -> Option<Task> {
        let tasks = self.read();
        tasks.tasks.into_iter().find(|t| t.id == id)
    }

    pub fn save(&self, task: Task) {
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
