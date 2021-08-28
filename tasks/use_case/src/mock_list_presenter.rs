use crate::ListPresenter;
use entity::Task;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Default)]
pub struct MockListPresenter {
    pub rc: Rc<RefCell<Option<Vec<Task>>>>,
}

impl MockListPresenter {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ListPresenter for MockListPresenter {
    fn complete(&self, tasks: &[Task]) {
        let mut cell = self.rc.borrow_mut();
        *cell = Some(tasks.to_vec());
    }
}

#[cfg(test)]
mod tests {
    use entity::TaskText;

    use super::*;

    #[test]
    fn test() {
        let presenter = MockListPresenter::new();
        let tasks = vec![Task::new(1.into(), TaskText::from("task 1".to_string()))];
        presenter.complete(&tasks);

        let cell = presenter.rc.borrow_mut();
        assert_eq!(*cell, Some(tasks));
    }
}
