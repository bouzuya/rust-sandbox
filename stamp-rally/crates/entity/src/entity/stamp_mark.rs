use crate::{StampCardId, StampMarkId};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StampMark {
    id: StampMarkId,
    stamp_card_id: StampCardId,
}

impl StampMark {
    pub fn new(stamp_card_id: StampCardId) -> Self {
        Self {
            id: StampMarkId::generate(),
            stamp_card_id,
        }
    }

    pub fn id(&self) -> StampMarkId {
        self.id
    }

    pub fn stamp_card_id(&self) -> StampCardId {
        self.stamp_card_id
    }
}

#[cfg(test)]
mod tests {
    use crate::{StampCardId, StampMark, StampMarkId};

    #[test]
    fn new_test() {
        let stamp_card_id = StampCardId::generate();
        let stamp_mark = StampMark::new(stamp_card_id);
        assert_eq!(stamp_mark, stamp_mark);
    }

    #[test]
    fn id_test() {
        let stamp_card_id = StampCardId::generate();
        let stamp_mark = StampMark::new(stamp_card_id);
        assert_ne!(stamp_mark.id(), StampMarkId::generate());
    }

    #[test]
    fn stamp_card_id_test() {
        let stamp_card_id = StampCardId::generate();
        let stamp_mark = StampMark::new(stamp_card_id);
        assert_eq!(stamp_mark.stamp_card_id(), stamp_card_id);
    }
}
