use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use entity::{Player, PlayerId};

use crate::port::{PlayerRepository, PlayerRepositoryError};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct InMemoryPlayerRepository {
    rc: Rc<RefCell<BTreeMap<PlayerId, Player>>>,
}

impl InMemoryPlayerRepository {
    pub fn new() -> Self {
        Self::default()
    }
}

impl PlayerRepository for InMemoryPlayerRepository {
    fn find_by_id(&self, player_id: PlayerId) -> Result<Option<Player>, PlayerRepositoryError> {
        let storage = self.rc.borrow();
        Ok(storage.get(&player_id).cloned())
    }

    fn save(&self, player: Player) -> Result<(), PlayerRepositoryError> {
        let mut storage = self.rc.borrow_mut();
        storage.insert(player.id(), player);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use entity::{StampRallyId, UserId};

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let stamp_rally_id = StampRallyId::generate();
        let user_id = UserId::generate();
        let repository = InMemoryPlayerRepository::default();
        let player = Player::new(stamp_rally_id, user_id);
        repository.save(player.clone())?;
        assert_eq!(repository.find_by_id(player.id())?, Some(player));
        Ok(())
    }
}
