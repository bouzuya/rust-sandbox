use crate::TaskRepository;
use std::rc::Rc;

pub struct CompleteUseCase {
    repository: Rc<dyn TaskRepository>,
}

impl CompleteUseCase {
    pub fn new(repository: Rc<dyn TaskRepository>) -> Self {
        Self { repository }
    }

    pub fn handle(&self, id: usize) {
        let mut task = self.repository.find_by_id(id).unwrap();
        task.done = true;
        self.repository.save(task);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TaskMemoryRepository;

    #[test]
    fn test() {
        let repository = TaskMemoryRepository::new();
        CompleteUseCase::new(Rc::new(repository));
        // TODO
    }
}
