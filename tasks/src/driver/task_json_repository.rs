use anyhow::Context;
use std::{env, fs, path::PathBuf};
use tasks::{entity::Task, use_case::TaskRepository};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Tasks {
    next_id: usize,
    tasks: Vec<TaskData>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
struct TaskData {
    pub done: bool,
    pub id: usize,
    pub text: String,
}

impl From<Task> for TaskData {
    fn from(task: Task) -> Self {
        Self {
            done: task.done,
            id: task.id,
            text: task.text,
        }
    }
}

// -> TryFrom
impl From<TaskData> for Task {
    fn from(data: TaskData) -> Self {
        Self {
            done: data.done,
            id: data.id,
            text: data.text,
        }
    }
}

pub struct TaskJsonRepository {
    path: PathBuf,
}

impl TaskJsonRepository {
    pub fn new() -> anyhow::Result<Self> {
        let path = match env::var("TASKS_JSON") {
            Ok(path) => PathBuf::from(path),
            Err(_) => dirs::data_dir()
                .context("data_dir is none")?
                .join("net.bouzuya.rust-sandbox.tasks")
                .join("tasks.json"),
        };
        if let Some(dir) = path.parent() {
            if !dir.exists() {
                fs::create_dir(dir).unwrap();
            }
        }
        Ok(Self { path })
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
        tasks.tasks.push(TaskData {
            id: tasks.next_id,
            text,
            done: false,
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
        tasks
            .tasks
            .iter()
            .cloned()
            .map(Task::from)
            .collect::<Vec<Task>>()
    }

    fn find_by_id(&self, id: usize) -> Option<Task> {
        let tasks = self.read();
        tasks.tasks.into_iter().find(|t| t.id == id).map(Task::from)
    }

    fn save(&self, task: Task) {
        let mut tasks = self.read();
        let task_position = tasks.tasks.iter().position(|t| t.id == task.id).unwrap();
        let task = tasks.tasks.get_mut(task_position).unwrap();
        task.done = true;
        self.write(&tasks);
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let tasks_json = temp_dir.path().join("tasks.json");
        env::set_var("TASKS_JSON", tasks_json.as_path());
        let repository = TaskJsonRepository::new()?;
        assert_eq!(repository.find_all(), vec![]);
        assert_eq!(repository.find_by_id(1), None);
        assert_eq!(tasks_json.as_path().exists(), false);

        repository.create("task1".to_string());
        assert_eq!(repository.find_all(), vec![Task::new(1, "task1")]);
        assert_eq!(repository.find_by_id(1), Some(Task::new(1, "task1")));
        assert_eq!(
            fs::read_to_string(tasks_json.as_path())?,
            r#"{"next_id":2,"tasks":[{"done":false,"id":1,"text":"task1"}]}"#
        );

        let mut task = Task::new(1, "task1");
        task.done = true;
        repository.save(task.clone());
        assert_eq!(repository.find_all(), vec![task.clone()]);
        assert_eq!(repository.find_by_id(1), Some(task));
        assert_eq!(
            fs::read_to_string(tasks_json.as_path())?,
            r#"{"next_id":2,"tasks":[{"done":true,"id":1,"text":"task1"}]}"#
        );

        repository.delete(1);
        assert_eq!(repository.find_all(), vec![]);
        assert_eq!(repository.find_by_id(1), None);
        assert_eq!(
            fs::read_to_string(tasks_json.as_path())?,
            r#"{"next_id":2,"tasks":[]}"#
        );

        Ok(())
    }
}
