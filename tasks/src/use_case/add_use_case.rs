use std::rc::Rc;

use crate::TaskRepository;

pub struct AddUseCase {
    repository: Rc<dyn TaskRepository>,
}

impl AddUseCase {
    pub fn new(repository: Rc<dyn TaskRepository>) -> Self {
        Self { repository }
    }

    pub fn add(&self, text: String) {
        self.repository.create(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TaskMemoryRepository;

    #[test]
    fn test() {
        let repository = TaskMemoryRepository::new();
        AddUseCase::new(Rc::new(repository));
        // TODO
    }
}
