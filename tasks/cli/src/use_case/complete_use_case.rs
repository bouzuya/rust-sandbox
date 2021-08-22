use crate::{entity::TaskId, use_case::TaskRepository};
use std::rc::Rc;

pub struct CompleteUseCase {
    repository: Rc<dyn TaskRepository>,
}

impl CompleteUseCase {
    pub fn new(repository: Rc<dyn TaskRepository>) -> Self {
        Self { repository }
    }

    // TODO: id -> task_id
    pub fn handle(&self, id: usize) {
        let id = TaskId::from(id);
        let mut task = self.repository.find_by_id(id).unwrap();
        task.complete();
        self.repository.save(task);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::use_case::MockTaskRepository;

    #[test]
    fn test() {
        let repository = MockTaskRepository::new();
        CompleteUseCase::new(Rc::new(repository));
        // TODO
    }
}
