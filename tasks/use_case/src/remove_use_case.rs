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

    pub fn handle(&self, id: TaskId) {
        // TODO: unwrap
        self.repository.delete(id).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use entity::TaskText;

    use super::*;
    use crate::MockTaskRepository;

    #[test]
    fn test() -> anyhow::Result<()> {
        let repository = MockTaskRepository::new();
        repository.create(TaskText::from("text".to_string()))?;
        assert!(!repository.find_all()?.is_empty());
        let repository = Rc::new(repository);
        let use_case = RemoveUseCase::new(repository.clone());
        use_case.handle(TaskId::from(1));
        assert!(repository.find_all()?.is_empty());
        Ok(())
    }
}