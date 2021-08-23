use crate::TaskRepository;
use entity::TaskId;
use std::rc::Rc;

pub struct CompleteUseCase {
    repository: Rc<dyn TaskRepository>,
}

impl CompleteUseCase {
    pub fn new(repository: Rc<dyn TaskRepository>) -> Self {
        Self { repository }
    }

    pub fn handle(&self, id: TaskId) {
        let mut task = self.repository.find_by_id(id).unwrap();
        task.complete();
        self.repository.save(task);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MockTaskRepository;

    #[test]
    fn test() {
        let repository = MockTaskRepository::new();
        repository.create("text".to_string());
        let created = repository.find_all().first().unwrap().clone();
        assert!(created.completed_at().is_none());
        let repository = Rc::new(repository);
        let use_case = CompleteUseCase::new(repository.clone());
        use_case.handle(created.id());
        let completed = repository.find_all().first().unwrap().clone();
        assert!(completed.completed_at().is_some());
    }
}
