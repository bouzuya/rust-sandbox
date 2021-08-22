#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TaskId(usize);

impl From<TaskId> for usize {
    fn from(id: TaskId) -> Self {
        id.0
    }
}

impl From<usize> for TaskId {
    fn from(v: usize) -> Self {
        Self(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(usize::from(TaskId::from(1)), 1);
    }
}
