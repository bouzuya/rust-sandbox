use crate::{use_case::ListPresenter, Task};
use std::{cell::RefCell, rc::Rc};

pub struct ListMemoryPresenter {
    pub rc: Rc<RefCell<Option<Vec<Task>>>>,
}

impl ListMemoryPresenter {
    pub fn new() -> Self {
        Self {
            rc: Rc::new(RefCell::new(None)),
        }
    }
}

impl ListPresenter for ListMemoryPresenter {
    fn complete(&self, tasks: &Vec<Task>) {
        let mut cell = self.rc.borrow_mut();
        *cell = Some(tasks.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let presenter = ListMemoryPresenter::new();
        let tasks = vec![Task {
            done: false,
            id: 1,
            text: "task 1".to_string(),
        }];
        presenter.complete(&tasks);

        let cell = presenter.rc.borrow_mut();
        assert_eq!(*cell, Some(tasks));
    }
}
