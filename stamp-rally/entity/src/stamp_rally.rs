use crate::StampRallyId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StampRally {
    id: StampRallyId,
}

impl StampRally {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            id: StampRallyId::generate(),
        }
    }

    pub fn id(&self) -> StampRallyId {
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
