use anyhow::Context;
use entity::{Task, TaskId, TaskText};
use std::{env, fs, path::PathBuf};
use use_case::{TaskRepository, TaskRepositoryError};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Tasks {
    next_id: usize,
    tasks: Vec<TaskData>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
struct TaskData {
    pub completed_at: Option<i64>,
    pub id: usize,
    pub text: String,
}

impl From<Task> for TaskData {
    fn from(task: Task) -> Self {
        Self {
            completed_at: task.completed_at(),
            id: usize::from(task.id()),
            text: task.text().to_string(),
        }
    }
}

// -> TryFrom
impl From<TaskData> for Task {
    fn from(data: TaskData) -> Self {
        Self::raw(
            TaskId::from(data.id),
            TaskText::from(data.text),
            data.completed_at,
        )
    }
}

pub struct JsonTaskDataSource {
    path: PathBuf,
}

impl JsonTaskDataSource {
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

    fn read(&self) -> anyhow::Result<Tasks> {
        let json_string = if self.path.exists() {
            fs::read_to_string(self.path.as_path())?
        } else {
            r#"{"next_id":1,"tasks":[]}"#.to_owned()
        };
        Ok(serde_json::from_str(json_string.as_str())?)
    }

    fn write(&self, tasks: &Tasks) -> anyhow::Result<()> {
        let json_string = serde_json::to_string(tasks)?;
        Ok(fs::write(self.path.as_path(), json_string)?)
    }
}

impl TaskRepository for JsonTaskDataSource {
    fn create(&self, text: TaskText) -> Result<TaskId, TaskRepositoryError> {
        let mut tasks = self.read().map_err(|_| TaskRepositoryError)?;
        let id = tasks.next_id;
        tasks.tasks.push(TaskData {
            id,
            text: String::from(text),
            completed_at: None,
        });
        tasks.next_id += 1;
        self.write(&tasks).map_err(|_| TaskRepositoryError)?;
        Ok(TaskId::from(id))
    }

    fn delete(&self, id: TaskId) -> Result<(), TaskRepositoryError> {
        let id = usize::from(id);
        let mut tasks = self.read().map_err(|_| TaskRepositoryError)?;
        let task_position = tasks.tasks.iter().position(|t| t.id == id).unwrap();
        tasks.tasks.remove(task_position);
        self.write(&tasks).map_err(|_| TaskRepositoryError)?;
        Ok(())
    }

    fn find_all(&self) -> Result<Vec<Task>, TaskRepositoryError> {
        let tasks = self.read().map_err(|_| TaskRepositoryError)?;
        Ok(tasks
            .tasks
            .iter()
            .cloned()
            .map(Task::from)
            .collect::<Vec<Task>>())
    }

    fn find_by_id(&self, id: TaskId) -> Result<Option<Task>, TaskRepositoryError> {
        let id = usize::from(id);
        let tasks = self.read().map_err(|_| TaskRepositoryError)?;
        Ok(tasks.tasks.into_iter().find(|t| t.id == id).map(Task::from))
    }

    fn save(&self, task: Task) -> Result<(), TaskRepositoryError> {
        let mut tasks = self.read().map_err(|_| TaskRepositoryError)?;
        let id_as_usize = usize::from(task.id());
        let task_position = tasks
            .tasks
            .iter()
            .position(|t| t.id == id_as_usize)
            .ok_or(TaskRepositoryError)?;
        let task_data_mut = tasks
            .tasks
            .get_mut(task_position)
            .ok_or(TaskRepositoryError)?;
        *task_data_mut = TaskData::from(task);
        self.write(&tasks).map_err(|_| TaskRepositoryError)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use entity::TaskText;
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let tasks_json = temp_dir.path().join("tasks.json");
        env::set_var("TASKS_JSON", tasks_json.as_path());
        let repository = JsonTaskDataSource::new()?;
        let id = TaskId::from(1);
        assert_eq!(repository.find_all()?, vec![]);
        assert_eq!(repository.find_by_id(id)?, None);
        assert!(!tasks_json.as_path().exists());

        repository.create(TaskText::from("task1".to_string()))?;
        assert_eq!(
            repository.find_all()?,
            vec![Task::new(id, TaskText::from("task1".to_string()))]
        );
        assert_eq!(
            repository.find_by_id(id)?,
            Some(Task::new(id, TaskText::from("task1".to_string())))
        );
        assert_eq!(
            fs::read_to_string(tasks_json.as_path())?,
            r#"{"next_id":2,"tasks":[{"completed_at":null,"id":1,"text":"task1"}]}"#
        );

        let mut task = Task::new(id, TaskText::from("task1".to_string()));
        task.complete();
        repository.save(task.clone())?;
        assert_eq!(repository.find_all()?, vec![task.clone()]);
        assert_eq!(repository.find_by_id(id)?, Some(task.clone()));
        assert_eq!(
            fs::read_to_string(tasks_json.as_path())?,
            format!(
                r#"{{"next_id":2,"tasks":[{{"completed_at":{},"id":1,"text":"task1"}}]}}"#,
                task.completed_at().unwrap()
            )
        );

        repository.delete(id)?;
        assert_eq!(repository.find_all()?, vec![]);
        assert_eq!(repository.find_by_id(id)?, None);
        assert_eq!(
            fs::read_to_string(tasks_json.as_path())?,
            r#"{"next_id":2,"tasks":[]}"#
        );

        Ok(())
    }
}
