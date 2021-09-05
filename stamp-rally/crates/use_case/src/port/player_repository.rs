use entity::{Player, PlayerId};
use thiserror::Error;

#[derive(Clone, Debug, Error, Eq, PartialEq)]
#[error("player repository error")]
pub struct PlayerRepositoryError;

pub trait PlayerRepository {
    fn find_by_id(&self, player_id: PlayerId) -> Result<Option<Player>, PlayerRepositoryError>;
    fn save(&self, player: Player) -> Result<(), PlayerRepositoryError>;
}

pub trait HasPlayerRepository {
    type PlayerRepository: PlayerRepository;

    fn player_repository(&self) -> &Self::PlayerRepository;
}
