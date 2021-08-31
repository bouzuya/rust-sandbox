use crate::UserId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct User {
    id: UserId,
}

impl User {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            id: UserId::generate(),
        }
    }

    pub fn id(&self) -> UserId {
        self.id
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn new_test() {
        // TODO:
    }

    #[test]
    fn id_test() {
        // TODO:
    }
}
