use crate::TaskRepository;
use entity::TaskId;
use std::rc::Rc;

pub struct RemoveUseCase {
    repository: Rc<dyn TaskRepository>,
}

impl RemoveUseCase {
    pub fn new(repository: Rc<dyn TaskRepository>) -> Self {
        Self { repository }
    }

    // TODO: id -> task_id
    pub fn handle(&self, id: usize) {
        let id = TaskId::from(id);
        self.repository.delete(id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MockTaskRepository;

    #[test]
    fn test() {
        let repository = MockTaskRepository::new();
        RemoveUseCase::new(Rc::new(repository));
        // TODO
    }
}
