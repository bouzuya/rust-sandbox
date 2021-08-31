use crate::{PlayerId, StampRallyId, UserId};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Player {
    id: PlayerId,
    stamp_rally_id: StampRallyId,
    user_id: UserId,
}

impl Player {
    pub fn new(stamp_rally_id: StampRallyId, user_id: UserId) -> Self {
        Self {
            id: PlayerId::generate(),
            stamp_rally_id,
            user_id,
        }
    }

    pub fn id(&self) -> PlayerId {
        self.id
    }

    pub fn stamp_rally_id(&self) -> StampRallyId {
        self.stamp_rally_id
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
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

    #[test]
    fn stamp_rally_id_test() {
        // TODO:
    }

    #[test]
    fn user_id_test() {
        // TODO:
    }
}
