use entity::{Task, TaskId};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("task repository error")]
pub struct TaskRepositoryError;

pub trait TaskRepository {
    fn create(&self, text: String) -> Result<TaskId, TaskRepositoryError>;
    fn delete(&self, id: TaskId) -> Result<(), TaskRepositoryError>;
    fn find_all(&self) -> Result<Vec<Task>, TaskRepositoryError>;
    fn find_by_id(&self, id: TaskId) -> Result<Option<Task>, TaskRepositoryError>;
    fn save(&self, task: Task) -> Result<(), TaskRepositoryError>;
}
