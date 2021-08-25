use crate::{task_repository::TaskRepositoryError, TaskRepository};
use entity::{Task, TaskId, TaskText};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct Tasks {
    next_id: usize,
    tasks: Vec<Task>,
}

pub struct MockTaskRepository {
    rc: Rc<RefCell<Tasks>>,
}

impl MockTaskRepository {
    pub fn new() -> Self {
        Self {
            rc: Rc::new(RefCell::new(Tasks {
                next_id: 1,
                tasks: vec![],
            })),
        }
    }
}

impl TaskRepository for MockTaskRepository {
    fn create(&self, text: String) -> Result<TaskId, TaskRepositoryError> {
        // TODO: TaskText
        let text = TaskText::from(text);
        let mut tasks = self.rc.borrow_mut();
        let id = TaskId::from(tasks.next_id);
        tasks.tasks.push(Task::new(id, text));
        tasks.next_id += 1;
        Ok(id)
    }

    fn delete(&self, id: TaskId) -> Result<(), TaskRepositoryError> {
        let mut tasks = self.rc.borrow_mut();
        let task_position = tasks.tasks.iter().position(|t| t.id() == id).unwrap();
        tasks.tasks.remove(task_position);
        Ok(())
    }

    fn find_all(&self) -> Result<Vec<Task>, TaskRepositoryError> {
        let tasks = self.rc.borrow();
        Ok(tasks.tasks.clone())
    }

    fn find_by_id(&self, id: TaskId) -> Result<Option<Task>, TaskRepositoryError> {
        let tasks = self.rc.borrow();
        Ok(tasks.tasks.iter().cloned().find(|t| t.id() == id))
    }

    fn save(&self, task: Task) -> Result<(), TaskRepositoryError> {
        let mut tasks = self.rc.borrow_mut();
        let task_position = tasks
            .tasks
            .iter()
            .position(|t| t.id() == task.id())
            .unwrap();
        let task_mut = tasks.tasks.get_mut(task_position).unwrap();
        *task_mut = task;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use entity::TaskText;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let repository = MockTaskRepository::new();
        assert!(repository.find_all()?.is_empty());
        repository.create("task1".to_string())?;

        assert_eq!(
            repository.find_all()?,
            vec![Task::new(
                TaskId::from(1),
                TaskText::from("task1".to_string())
            )]
        );
        assert_eq!(repository.find_by_id(TaskId::from(2))?, None);
        assert_eq!(
            repository.find_by_id(TaskId::from(1))?,
            Some(Task::new(1.into(), TaskText::from("task1".to_string())))
        );

        let mut updated = Task::new(TaskId::from(1), TaskText::from("task1".to_string()));
        updated.complete();
        repository.save(updated.clone())?;
        assert_eq!(
            repository.find_by_id(TaskId::from(1))?,
            Some(updated.clone())
        );

        repository.create("task2".to_string())?;
        assert_eq!(
            repository.find_all()?,
            vec![
                updated,
                Task::new(TaskId::from(2), TaskText::from("task2".to_string())),
            ]
        );

        repository.delete(TaskId::from(1))?;
        assert_eq!(
            repository.find_all()?,
            vec![Task::new(2.into(), TaskText::from("task2".to_string()))]
        );
        Ok(())
    }
}
