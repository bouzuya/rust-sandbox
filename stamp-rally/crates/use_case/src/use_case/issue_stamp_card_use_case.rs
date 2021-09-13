use entity::{PlayerId, StampCardId, StampRallyId};
use thiserror::Error;

use crate::{
    port::{HasStampRallyRepository, StampCardRepository, StampRallyRepository},
    HasPlayerRepository, HasStampCardRepository, PlayerRepository,
};

#[derive(Debug, Eq, Error, PartialEq)]
pub enum IssueStampCardError {
    #[error("stamp rally not found error")]
    StampRallyNotFound,
    #[error("player not found error")]
    PlayerNotFound,
    #[error("unknown error")]
    Unknown,
}

pub trait IssueStampCardUseCase:
    HasPlayerRepository + HasStampCardRepository + HasStampRallyRepository
{
    fn handle(
        &self,
        stamp_rally_id: StampRallyId,
        player_id: PlayerId,
    ) -> Result<StampCardId, IssueStampCardError> {
        let player_repository = self.player_repository();
        let stamp_card_repository = self.stamp_card_repository();
        let stamp_rally_repository = self.stamp_rally_repository();

        let mut stamp_rally = stamp_rally_repository
            .find_by_id(stamp_rally_id)
            .map_err(|_| IssueStampCardError::Unknown)?
            .ok_or(IssueStampCardError::StampRallyNotFound)?;
        let player = player_repository
            .find_by_id(player_id)
            .map_err(|_| IssueStampCardError::Unknown)?
            .ok_or(IssueStampCardError::PlayerNotFound)?;

        let stamp_card = stamp_rally.issue(player.id());

        // TODO: save stamp_rally

        let stamp_card_id = stamp_card.id();
        stamp_card_repository
            .save(stamp_card)
            .map(|_| stamp_card_id)
            .map_err(|_| IssueStampCardError::Unknown)
    }
}

impl<T: HasPlayerRepository + HasStampCardRepository + HasStampRallyRepository>
    IssueStampCardUseCase for T
{
}

pub trait HasIssueStampCardUseCase {
    type IssueStampCardUseCase: IssueStampCardUseCase;

    fn issue_stamp_card_use_case(&self) -> &Self::IssueStampCardUseCase;
}

#[cfg(test)]
mod tests {
    use entity::{StampRally, User};

    use crate::{
        InMemoryPlayerRepository, InMemoryStampCardRepository, InMemoryStampRallyRepository,
    };

    use super::*;

    struct U {
        player_repository: InMemoryPlayerRepository,
        stamp_rally_repository: InMemoryStampRallyRepository,
        stamp_card_repository: InMemoryStampCardRepository,
    }

    impl U {
        fn new_join_stamp_rally_use_case() -> impl IssueStampCardUseCase {
            Self {
                player_repository: InMemoryPlayerRepository::new(),
                stamp_rally_repository: InMemoryStampRallyRepository::new(),
                stamp_card_repository: InMemoryStampCardRepository::new(),
            }
        }
    }

    impl HasPlayerRepository for U {
        type PlayerRepository = InMemoryPlayerRepository;

        fn player_repository(&self) -> &Self::PlayerRepository {
            &self.player_repository
        }
    }

    impl HasStampRallyRepository for U {
        type StampRallyRepository = InMemoryStampRallyRepository;

        fn stamp_rally_repository(&self) -> &Self::StampRallyRepository {
            &self.stamp_rally_repository
        }
    }

    impl HasStampCardRepository for U {
        type StampCardRepository = InMemoryStampCardRepository;

        fn stamp_card_repository(&self) -> &Self::StampCardRepository {
            &self.stamp_card_repository
        }
    }

    #[test]
    fn test() -> anyhow::Result<()> {
        let use_case = U::new_join_stamp_rally_use_case();
        let (stamp_rally_id, player_id) = {
            let stamp_rally = StampRally::new();
            let user = User::new();
            let stamp_rally_id = stamp_rally.id();
            let player = stamp_rally.join(user.id());
            let player_id = player.id();
            use_case.stamp_rally_repository().save(stamp_rally)?;
            use_case.player_repository().save(player)?;
            (stamp_rally_id, player_id)
        };

        let stamp_card_id = IssueStampCardUseCase::handle(&use_case, stamp_rally_id, player_id)?;

        assert!(use_case
            .stamp_card_repository()
            .find_by_id(stamp_card_id)?
            .is_some());
        Ok(())
    }
}
