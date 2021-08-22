use entity::{Task, TaskId};

pub trait TaskRepository {
    fn create(&self, text: String);
    fn delete(&self, id: TaskId);
    fn find_all(&self) -> Vec<Task>;
    fn find_by_id(&self, id: TaskId) -> Option<Task>;
    fn save(&self, task: Task);
}
