use chrono::Utc;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Task {
    completed_at: Option<i64>,
    id: usize,
    text: String,
}

impl Task {
    pub fn raw(id: usize, text: String, completed_at: Option<i64>) -> Self {
        Self {
            completed_at,
            id,
            text,
        }
    }

    pub fn new(id: usize, text: impl Into<String>) -> Self {
        Self {
            completed_at: None,
            id,
            text: text.into(),
        }
    }

    pub fn done(&self) -> bool {
        self.completed_at.is_some()
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn complete(&mut self) {
        self.completed_at = Some(Utc::now().timestamp());
    }

    pub fn completed_at(&self) -> Option<i64> {
        self.completed_at
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut task = Task::new(1, "task1");
        assert_eq!(task, Task::raw(1, "task1".to_string(), None));
        assert_eq!(task.id(), 1);
        task.complete();
        assert_eq!(
            task,
            Task::raw(1, "task1".to_string(), Some(task.completed_at().unwrap()))
        );
    }
}
