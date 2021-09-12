use entity::{StampCard, StampCardId};
use thiserror::Error;

#[derive(Clone, Debug, Error, Eq, PartialEq)]
#[error("stamp card repository error")]
pub struct StampCardRepositoryError;

pub trait StampCardRepository {
    fn find_by_id(
        &self,
        stamp_card_id: StampCardId,
    ) -> Result<Option<StampCard>, StampCardRepositoryError>;
    fn save(&self, stamp_card: StampCard) -> Result<(), StampCardRepositoryError>;
}

pub trait HasStampCardRepository {
    type StampCardRepository: StampCardRepository;

    fn stamp_card_repository(&self) -> &Self::StampCardRepository;
}
