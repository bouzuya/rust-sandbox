use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use entity::{StampCard, StampCardId};

use crate::port::{StampCardRepository, StampCardRepositoryError};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct InMemoryStampCardRepository {
    rc: Rc<RefCell<BTreeMap<StampCardId, StampCard>>>,
}

impl InMemoryStampCardRepository {
    pub fn new() -> Self {
        Self::default()
    }
}

impl StampCardRepository for InMemoryStampCardRepository {
    fn find_by_id(
        &self,
        stamp_card_id: StampCardId,
    ) -> Result<Option<StampCard>, StampCardRepositoryError> {
        let storage = self.rc.borrow();
        Ok(storage.get(&stamp_card_id).cloned())
    }

    fn save(&self, stamp_card: StampCard) -> Result<(), StampCardRepositoryError> {
        let mut storage = self.rc.borrow_mut();
        storage.insert(stamp_card.id(), stamp_card);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use entity::{PlayerId, StampRallyId};

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let repository = InMemoryStampCardRepository::default();
        let stamp_rally_id = StampRallyId::generate();
        let player_id = PlayerId::generate();
        let stamp_card = StampCard::new(stamp_rally_id, player_id);
        repository.save(stamp_card.clone())?;
        assert_eq!(repository.find_by_id(stamp_card.id())?, Some(stamp_card));
        Ok(())
    }
}
