use std::collections::BTreeSet;
use thiserror::Error;

use crate::{Player, PlayerId, StampCard, StampCardId, StampRallyId, UserId};

#[derive(Debug, Eq, Error, PartialEq)]
#[error("issue stamp card error")]
pub struct IssueStampCardError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StampRally {
    id: StampRallyId,
    max_stamp_card_count: usize,
    stamp_card_ids: BTreeSet<StampCardId>, // the stamp card may not exist
}

impl StampRally {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            id: StampRallyId::generate(),
            max_stamp_card_count: 10, // TODO:
            stamp_card_ids: BTreeSet::new(),
        }
    }

    pub fn id(&self) -> StampRallyId {
        self.id
    }

    // factory
    pub fn join(&self, user_id: UserId) -> Player {
        Player::new(self.id, user_id)
    }

    pub fn issue(&mut self, player_id: PlayerId) -> Result<StampCard, IssueStampCardError> {
        if self.stamp_card_ids.len() >= self.max_stamp_card_count {
            return Err(IssueStampCardError);
        }

        let stamp_card = StampCard::new(self.id, player_id);
        if self.stamp_card_ids.insert(stamp_card.id()) {
            Ok(stamp_card)
        } else {
            Err(IssueStampCardError)
        }
    }

    pub fn is_issued(&mut self, stamp_card_id: StampCardId) -> bool {
        self.stamp_card_ids.contains(&stamp_card_id)
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
    fn join_test() {
        // TODO:
    }

    #[test]
    fn issue_test() {
        // TODO:
    }

    #[test]
    fn issued_test() {
        // TODO:
    }
}
