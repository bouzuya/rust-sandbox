use crate::entity::{Task, TaskId};
use crate::use_case::TaskRepository;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct Tasks {
    next_id: usize,
    tasks: Vec<Task>,
}

pub struct TaskMemoryRepository {
    rc: Rc<RefCell<Tasks>>,
}

impl TaskMemoryRepository {
    pub fn new() -> Self {
        Self {
            rc: Rc::new(RefCell::new(Tasks {
                next_id: 1,
                tasks: vec![],
            })),
        }
    }
}

impl TaskRepository for TaskMemoryRepository {
    fn create(&self, text: String) {
        let mut tasks = self.rc.borrow_mut();
        let next_id = tasks.next_id;
        tasks.tasks.push(Task::new(TaskId::from(next_id), text));
        tasks.next_id += 1;
    }

    fn delete(&self, id: TaskId) {
        let mut tasks = self.rc.borrow_mut();
        let task_position = tasks.tasks.iter().position(|t| t.id() == id).unwrap();
        tasks.tasks.remove(task_position);
    }

    fn find_all(&self) -> Vec<Task> {
        let tasks = self.rc.borrow();
        tasks.tasks.clone()
    }

    fn find_by_id(&self, id: TaskId) -> Option<Task> {
        let tasks = self.rc.borrow();
        tasks.tasks.iter().cloned().find(|t| t.id() == id)
    }

    fn save(&self, task: Task) {
        let mut tasks = self.rc.borrow_mut();
        let task_position = tasks
            .tasks
            .iter()
            .position(|t| t.id() == task.id())
            .unwrap();
        let task_mut = tasks.tasks.get_mut(task_position).unwrap();
        *task_mut = task;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let repository = TaskMemoryRepository::new();
        assert!(repository.find_all().is_empty());
        repository.create("task1".to_string());

        assert_eq!(
            repository.find_all(),
            vec![Task::new(TaskId::from(1), "task1")]
        );
        assert_eq!(repository.find_by_id(TaskId::from(2)), None);
        assert_eq!(
            repository.find_by_id(TaskId::from(1)),
            Some(Task::new(1.into(), "task1"))
        );

        let mut updated = Task::new(TaskId::from(1), "task1");
        updated.complete();
        repository.save(updated.clone());
        assert_eq!(
            repository.find_by_id(TaskId::from(1)),
            Some(updated.clone())
        );

        repository.create("task2".to_string());
        assert_eq!(
            repository.find_all(),
            vec![updated, Task::new(TaskId::from(2), "task2"),]
        );

        repository.delete(TaskId::from(1));
        assert_eq!(repository.find_all(), vec![Task::new(2.into(), "task2")]);
    }
}
