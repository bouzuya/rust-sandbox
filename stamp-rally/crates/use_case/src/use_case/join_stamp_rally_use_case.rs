use entity::{PlayerId, StampRallyId, UserId};
use thiserror::Error;

use crate::{
    port::{HasStampRallyRepository, StampRallyRepository, UserRepository},
    HasPlayerRepository, HasUserRepository, PlayerRepository,
};

#[derive(Debug, Eq, Error, PartialEq)]
pub enum JoinStampRallyError {
    #[error("stamp rally not found error")]
    StampRallyNotFound,
    #[error("user not found error")]
    UserNotFound,
    #[error("unknown error")]
    Unknown,
}

pub trait JoinStampRallyUseCase:
    HasPlayerRepository + HasStampRallyRepository + HasUserRepository
{
    fn handle(
        &self,
        stamp_rally_id: StampRallyId,
        user_id: UserId,
    ) -> Result<PlayerId, JoinStampRallyError> {
        let player_repository = self.player_repository();
        let stamp_rally_repository = self.stamp_rally_repository();
        let user_repository = self.user_repository();

        let stamp_rally = stamp_rally_repository
            .find_by_id(stamp_rally_id)
            .map_err(|_| JoinStampRallyError::Unknown)?
            .ok_or(JoinStampRallyError::StampRallyNotFound)?;
        let user = user_repository
            .find_by_id(user_id)
            .map_err(|_| JoinStampRallyError::Unknown)?
            .ok_or(JoinStampRallyError::UserNotFound)?;

        let player = stamp_rally.join(user.id());
        let player_id = player.id();
        player_repository
            .save(player)
            .map(|_| player_id)
            .map_err(|_| JoinStampRallyError::Unknown)
    }
}

impl<T: HasPlayerRepository + HasStampRallyRepository + HasUserRepository> JoinStampRallyUseCase
    for T
{
}

pub trait HasJoinStampRallyUseCase {
    type JoinStampRallyUseCase: JoinStampRallyUseCase;

    fn join_stamp_rally_use_case(&self) -> &Self::JoinStampRallyUseCase;
}

#[cfg(test)]
mod tests {
    use entity::{StampRally, User};

    use crate::{InMemoryPlayerRepository, InMemoryStampRallyRepository, InMemoryUserRepository};

    use super::*;

    struct U {
        player_repository: InMemoryPlayerRepository,
        stamp_rally_repository: InMemoryStampRallyRepository,
        user_repository: InMemoryUserRepository,
    }

    impl U {
        fn new_join_stamp_rally_use_case() -> impl JoinStampRallyUseCase {
            Self {
                player_repository: InMemoryPlayerRepository::new(),
                stamp_rally_repository: InMemoryStampRallyRepository::new(),
                user_repository: InMemoryUserRepository::new(),
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

    impl HasUserRepository for U {
        type UserRepository = InMemoryUserRepository;

        fn user_repository(&self) -> &Self::UserRepository {
            &self.user_repository
        }
    }

    #[test]
    fn test() -> anyhow::Result<()> {
        let use_case = U::new_join_stamp_rally_use_case();
        let (stamp_rally_id, user_id) = {
            let stamp_rally = StampRally::new();
            let user = User::new();
            let stamp_rally_id = stamp_rally.id();
            let user_id = user.id();
            use_case.stamp_rally_repository().save(stamp_rally)?;
            use_case.user_repository().save(user)?;
            (stamp_rally_id, user_id)
        };

        let player_id = JoinStampRallyUseCase::handle(&use_case, stamp_rally_id, user_id)?;

        assert!(use_case
            .player_repository()
            .find_by_id(player_id)?
            .is_some());
        Ok(())
    }
}
