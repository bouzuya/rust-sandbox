use crate::TaskRepository;
use std::rc::Rc;

pub struct AddUseCase {
    repository: Rc<dyn TaskRepository>,
}

impl AddUseCase {
    pub fn new(repository: Rc<dyn TaskRepository>) -> Self {
        Self { repository }
    }

    pub fn handle(&self, text: String) {
        // TODO: unwrap
        self.repository.create(text).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MockTaskRepository;

    #[test]
    fn test() {
        let repository = MockTaskRepository::new();
        AddUseCase::new(Rc::new(repository));
        // TODO
    }
}
