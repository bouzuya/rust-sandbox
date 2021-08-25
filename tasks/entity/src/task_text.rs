#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TaskText(String);

impl From<String> for TaskText {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl std::fmt::Display for TaskText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<TaskText> for String {
    fn from(t: TaskText) -> Self {
        t.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(TaskText::default(), TaskText::from(String::default()));
        assert_eq!(
            String::from(TaskText::from("text".to_string())),
            "text".to_string()
        );
    }
}
