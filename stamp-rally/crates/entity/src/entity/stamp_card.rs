use crate::{PlayerId, StampCardId, StampRallyId};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StampCard {
    id: StampCardId,
    player_id: PlayerId,
    stamp_rally_id: StampRallyId,
}

impl StampCard {
    pub fn new(stamp_rally_id: StampRallyId, player_id: PlayerId) -> Self {
        Self {
            id: StampCardId::generate(),
            player_id,
            stamp_rally_id,
        }
    }

    pub fn id(&self) -> StampCardId {
        self.id
    }

    pub fn player_id(&self) -> PlayerId {
        self.player_id
    }

    pub fn stamp_rally_id(&self) -> StampRallyId {
        self.stamp_rally_id
    }
}

#[cfg(test)]
mod tests {
    use crate::{PlayerId, StampCard, StampCardId, StampRallyId};

    #[test]
    fn new_test() {
        let stamp_rally_id = StampRallyId::generate();
        let player_id = PlayerId::generate();
        let stamp_card = StampCard::new(stamp_rally_id, player_id);
        assert_ne!(stamp_card.id(), StampCardId::generate());
        assert_eq!(stamp_card.player_id(), player_id);
        assert_eq!(stamp_card.stamp_rally_id(), stamp_rally_id);
    }

    #[test]
    fn id_test() {
        let stamp_card1 = new_stamp_card();
        let stamp_card2 = new_stamp_card();
        assert_ne!(stamp_card1.id(), stamp_card2.id());
    }

    #[test]
    fn player_id_test() {
        let stamp_rally_id = StampRallyId::generate();
        let player_id = PlayerId::generate();
        let stamp_card = StampCard::new(stamp_rally_id, player_id);
        assert_eq!(stamp_card.player_id(), player_id);
    }

    #[test]
    fn stamp_rally_id_test() {
        let stamp_rally_id = StampRallyId::generate();
        let player_id = PlayerId::generate();
        let stamp_card = StampCard::new(stamp_rally_id, player_id);
        assert_eq!(stamp_card.stamp_rally_id(), stamp_rally_id);
    }

    fn new_stamp_card() -> StampCard {
        let stamp_rally_id = StampRallyId::generate();
        let player_id = PlayerId::generate();
        StampCard::new(stamp_rally_id, player_id)
    }
}
