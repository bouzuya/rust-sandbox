#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Task {
    pub done: bool,
    id: usize,
    pub text: String,
}

impl Task {
    pub fn raw(id: usize, text: String, done: bool) -> Self {
        Self { done, id, text }
    }

    pub fn new(id: usize, text: impl Into<String>) -> Self {
        Self {
            done: false,
            id,
            text: text.into(),
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn complete(&mut self) {
        self.done = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut task = Task::new(1, "task1");
        assert_eq!(task, Task::raw(1, "task1".to_string(), false));
        assert_eq!(task.id(), 1);
        task.complete();
        assert_eq!(task, Task::raw(1, "task1".to_string(), true));
    }
}
