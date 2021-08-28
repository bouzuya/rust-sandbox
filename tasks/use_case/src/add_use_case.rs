use entity::TaskText;

use crate::TaskRepository;
use std::rc::Rc;
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum AddUseCaseError {
    #[error("io error")]
    IOError,
}

pub struct AddUseCase {
    repository: Rc<dyn TaskRepository>,
}

impl AddUseCase {
    pub fn new(repository: Rc<dyn TaskRepository>) -> Self {
        Self { repository }
    }

    pub fn handle(&self, text: TaskText) -> Result<(), AddUseCaseError> {
        self.repository
            .create(text)
            .map(|_| ())
            .map_err(|_| AddUseCaseError::IOError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MockTaskRepository;

    #[test]
    fn test() {
        let repository = MockTaskRepository::new();
        let use_case = AddUseCase::new(Rc::new(repository));
        let text = TaskText::from("text".to_string());
        let result = use_case.handle(text);
        assert!(result.is_ok());
    }
}
