use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use entity::{StampRally, StampRallyId};

use crate::port::{StampRallyRepository, StampRallyRepositoryError};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct InMemoryStampRallyRepository {
    rc: Rc<RefCell<BTreeMap<StampRallyId, StampRally>>>,
}

impl InMemoryStampRallyRepository {
    pub fn new() -> Self {
        Self::default()
    }
}

impl StampRallyRepository for InMemoryStampRallyRepository {
    fn find_by_id(
        &self,
        stamp_rally_id: StampRallyId,
    ) -> Result<Option<StampRally>, StampRallyRepositoryError> {
        let storage = self.rc.borrow();
        Ok(storage.get(&stamp_rally_id).cloned())
    }

    fn save(&self, stamp_rally: StampRally) -> Result<(), StampRallyRepositoryError> {
        let mut storage = self.rc.borrow_mut();
        storage.insert(stamp_rally.id(), stamp_rally);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let repository = InMemoryStampRallyRepository::default();
        let stamp_rally = StampRally::new();
        repository.save(stamp_rally.clone())?;
        assert_eq!(repository.find_by_id(stamp_rally.id())?, Some(stamp_rally));
        Ok(())
    }
}
