use crate::task::Task;

pub trait TaskRepository {
    fn create(&self, text: String);
    fn delete(&self, id: usize);
    fn find_all(&self) -> Vec<Task>;
    fn find_by_id(&self, id: usize) -> Option<Task>;
    fn save(&self, task: Task);
}
