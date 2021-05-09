use crate::use_case::TaskRepository;
use std::rc::Rc;

pub struct RemoveUseCase {
    repository: Rc<dyn TaskRepository>,
}

impl RemoveUseCase {
    pub fn new(repository: Rc<dyn TaskRepository>) -> Self {
        Self { repository }
    }

    pub fn handle(&self, id: usize) {
        self.repository.delete(id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::use_case::TaskMemoryRepository;

    #[test]
    fn test() {
        let repository = TaskMemoryRepository::new();
        RemoveUseCase::new(Rc::new(repository));
        // TODO
    }
}
