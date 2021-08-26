use crate::{ListPresenter, TaskRepository};
use entity::Task;
use std::rc::Rc;

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

    pub fn handle(&self, all: bool) {
        // TODO: unwrap
        let tasks = self
            .repository
            .find_all()
            .unwrap()
            .into_iter()
            .filter(|task| all || !task.done())
            .collect::<Vec<Task>>();
        self.presenter.complete(&tasks);
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
        ListUseCase::new(presenter.clone(), Rc::new(repository)).handle(false);
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
