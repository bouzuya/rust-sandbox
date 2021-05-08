use std::{cell::RefCell, rc::Rc};

use crate::{task::Task, TaskRepository};

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
        tasks.tasks.push(Task {
            done: false,
            id: next_id,
            text,
        });
        tasks.next_id += 1;
    }

    fn delete(&self, id: usize) {
        let mut tasks = self.rc.borrow_mut();
        let task_position = tasks.tasks.iter().position(|t| t.id == id).unwrap();
        tasks.tasks.remove(task_position);
    }

    fn find_all(&self) -> Vec<Task> {
        let tasks = self.rc.borrow();
        tasks.tasks.clone()
    }

    fn find_by_id(&self, id: usize) -> Option<Task> {
        let tasks = self.rc.borrow();
        tasks.tasks.iter().cloned().find(|t| t.id == id)
    }

    fn save(&self, task: Task) {
        let mut tasks = self.rc.borrow_mut();
        let task_position = tasks.tasks.iter().position(|t| t.id == task.id).unwrap();
        let task = tasks.tasks.get_mut(task_position).unwrap();
        task.done = true;
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
            vec![Task {
                done: false,
                id: 1,
                text: "task1".to_string()
            }]
        );
        assert_eq!(repository.find_by_id(2), None);
        assert_eq!(
            repository.find_by_id(1),
            Some(Task {
                done: false,
                id: 1,
                text: "task1".to_string()
            })
        );

        let updated = Task {
            done: true,
            id: 1,
            text: "task1".to_string(),
        };
        repository.save(updated);
        assert_eq!(
            repository.find_by_id(1),
            Some(Task {
                done: true,
                id: 1,
                text: "task1".to_string()
            })
        );

        repository.create("task2".to_string());
        assert_eq!(
            repository.find_all(),
            vec![
                Task {
                    done: true,
                    id: 1,
                    text: "task1".to_string()
                },
                Task {
                    done: false,
                    id: 2,
                    text: "task2".to_string()
                }
            ]
        );

        repository.delete(1);
        assert_eq!(
            repository.find_all(),
            vec![Task {
                done: false,
                id: 2,
                text: "task2".to_string()
            }]
        );
    }
}
