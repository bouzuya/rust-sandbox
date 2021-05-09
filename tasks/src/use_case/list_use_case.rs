use crate::entity::Task;
use crate::use_case::{ListPresenter, TaskRepository};
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
        let tasks = self
            .repository
            .find_all()
            .into_iter()
            .filter(|task| all || !task.done)
            .collect::<Vec<Task>>();
        self.presenter.complete(&tasks);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::use_case::{ListMemoryPresenter, TaskMemoryRepository};

    #[test]
    fn test() {
        let presenter = Rc::new(ListMemoryPresenter::new());
        let repository = TaskMemoryRepository::new();
        repository.create("task1".to_string());
        ListUseCase::new(presenter.clone(), Rc::new(repository)).handle(false);
        let cell = presenter.rc.borrow_mut();
        assert_eq!(*cell, Some(vec![Task::new(1, "task1")]));
    }
}
