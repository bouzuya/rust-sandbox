use crate::{ListPresenter, TaskRepository};
use entity::Task;
use std::rc::Rc;
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum ListUseCaseError {
    #[error("io error")]
    IOError,
}

pub struct ListUseCase {
    presenter: Rc<dyn ListPresenter>,
    repository: Rc<dyn TaskRepository>,
}

impl ListUseCase {
    pub fn new(presenter: Rc<dyn ListPresenter>, repository: Rc<dyn TaskRepository>) -> Self {
        Self {
            presenter,
            repository,
        }
    }

    pub fn handle(&self, all: bool) -> Result<(), ListUseCaseError> {
        let tasks = self
            .repository
            .find_all()
            .map_err(|_| ListUseCaseError::IOError)?;
        let filtered = tasks
            .into_iter()
            .filter(|task| all || !task.done())
            .collect::<Vec<Task>>();
        self.presenter.complete(&filtered);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use entity::TaskText;

    use super::*;
    use crate::{MockListPresenter, MockTaskRepository};

    #[test]
    fn test() -> anyhow::Result<()> {
        let presenter = Rc::new(MockListPresenter::new());
        let repository = MockTaskRepository::new();
        repository.create(TaskText::from("task1".to_string()))?;
        let use_case = ListUseCase::new(presenter.clone(), Rc::new(repository));
        use_case.handle(false)?;
        let cell = presenter.rc.borrow_mut();
        assert_eq!(
            *cell,
            Some(vec![Task::new(
                1.into(),
                TaskText::from("task1".to_string())
            )])
        );
        Ok(())
    }
}
