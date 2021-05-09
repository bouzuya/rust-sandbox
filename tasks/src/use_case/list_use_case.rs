use crate::TaskRepository;
use std::rc::Rc;

pub struct ListUseCase {
    repository: Rc<dyn TaskRepository>,
}

impl ListUseCase {
    pub fn new(repository: Rc<dyn TaskRepository>) -> Self {
        Self { repository }
    }

    pub fn handle(&self, all: bool) {
        let tasks = self.repository.find_all();
        println!(
            "{}",
            tasks
                .iter()
                .filter(|task| all || !task.done)
                .map(|task| format!(
                    "{} {} {}",
                    task.id,
                    if task.done { "☑" } else { "☐" },
                    task.text
                ))
                .collect::<Vec<String>>()
                .join("\n")
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TaskMemoryRepository;

    #[test]
    fn test() {
        let repository = TaskMemoryRepository::new();
        ListUseCase::new(Rc::new(repository));
        // TODO
    }
}
