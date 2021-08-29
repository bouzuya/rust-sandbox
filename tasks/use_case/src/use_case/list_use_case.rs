use crate::TaskRepository;
use entity::Task;
use std::rc::Rc;
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum ListUseCaseError {
    #[error("io error")]
    IOError,
}

pub struct ListUseCase {
    repository: Rc<dyn TaskRepository>,
}

impl ListUseCase {
    pub fn new(repository: Rc<dyn TaskRepository>) -> Self {
        Self { repository }
    }

    pub fn handle(&self, all: bool) -> Result<Vec<Task>, ListUseCaseError> {
        let tasks = self
            .repository
            .find_all()
            .map_err(|_| ListUseCaseError::IOError)?;
        let filtered = tasks
            .into_iter()
            .filter(|task| all || !task.done())
            .collect::<Vec<Task>>();
        Ok(filtered)
    }
}

#[cfg(test)]
mod tests {
    use entity::TaskText;

    use super::*;
    use crate::MockTaskRepository;

    #[test]
    fn test() -> anyhow::Result<()> {
        let repository = MockTaskRepository::new();
        repository.create(TaskText::from("task1".to_string()))?;
        let use_case = ListUseCase::new(Rc::new(repository));
        let tasks = use_case.handle(false)?;
        assert_eq!(
            tasks,
            vec![Task::new(1.into(), TaskText::from("task1".to_string()))]
        );
        Ok(())
    }
}
