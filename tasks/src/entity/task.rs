#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Task {
    pub done: bool,
    pub id: usize,
    pub text: String,
}

impl Task {
    pub fn new(id: usize, text: impl Into<String>) -> Self {
        Self {
            done: false,
            id,
            text: text.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let task = Task::new(1, "task1");
        assert_eq!(
            task,
            Task {
                done: false,
                id: 1,
                text: "task1".to_string()
            }
        );
    }
}
