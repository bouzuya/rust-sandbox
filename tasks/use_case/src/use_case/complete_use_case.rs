use crate::TaskRepository;
use entity::TaskId;
use std::rc::Rc;
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum CompleteUseCaseError {
    #[error("task not found error")]
    TaskNotFoundError,
    #[error("io error")]
    IOError,
}

pub struct CompleteUseCase {
    repository: Rc<dyn TaskRepository>,
}

impl CompleteUseCase {
    pub fn new(repository: Rc<dyn TaskRepository>) -> Self {
        Self { repository }
    }

    pub fn handle(&self, id: TaskId) -> Result<(), CompleteUseCaseError> {
        match self
            .repository
            .find_by_id(id)
            .map_err(|_| CompleteUseCaseError::IOError)?
        {
            None => Err(CompleteUseCaseError::TaskNotFoundError),
            Some(mut task) => {
                task.complete();
                self.repository
                    .save(task)
                    .map_err(|_| CompleteUseCaseError::IOError)
            }
        }
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
        repository.create(TaskText::from("text".to_string()))?;
        let created = repository.find_all()?.first().unwrap().clone();
        assert!(created.completed_at().is_none());
        let repository = Rc::new(repository);
        let use_case = CompleteUseCase::new(repository.clone());
        use_case.handle(created.id())?;
        let completed = repository.find_all()?.first().unwrap().clone();
        assert!(completed.completed_at().is_some());
        Ok(())
    }
}
