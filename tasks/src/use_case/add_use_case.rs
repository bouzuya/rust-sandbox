use crate::use_case::TaskRepository;
use std::rc::Rc;

pub struct AddUseCase {
    repository: Rc<dyn TaskRepository>,
}

impl AddUseCase {
    pub fn new(repository: Rc<dyn TaskRepository>) -> Self {
        Self { repository }
    }

    pub fn handle(&self, text: String) {
        self.repository.create(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::use_case::MockTaskRepository;

    #[test]
    fn test() {
        let repository = MockTaskRepository::new();
        AddUseCase::new(Rc::new(repository));
        // TODO
    }
}
