use entity::{StampRally, StampRallyId};
use thiserror::Error;

use crate::port::{HasStampRallyRepository, StampRallyRepository};

#[derive(Debug, Eq, Error, PartialEq)]
#[error("create stamp rally error")]
pub struct CreateStampRallyError;

pub trait CreateStampRallyUseCase: HasStampRallyRepository {
    fn handle(&self) -> Result<StampRallyId, CreateStampRallyError> {
        let stamp_rally_repository = self.stamp_rally_repository();
        let stamp_rally = StampRally::new();
        stamp_rally_repository
            .save(stamp_rally.clone())
            .map(|_| stamp_rally.id())
            .map_err(|_| CreateStampRallyError)
    }
}

impl<T: HasStampRallyRepository> CreateStampRallyUseCase for T {}

pub trait HasCreateStampRallyUseCase {
    type CreateStampRallyUseCase: CreateStampRallyUseCase;

    fn create_stamp_rally_use_case(&self) -> &Self::CreateStampRallyUseCase;
}
