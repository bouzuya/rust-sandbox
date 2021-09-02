use entity::StampRallyId;
use thiserror::Error;

#[derive(Debug, Eq, Error, PartialEq)]
#[error("create stamp rally error")]
pub struct CreateStampRallyError;

pub trait CreateStampRallyUseCase {
    fn handle(&self) -> Result<StampRallyId, CreateStampRallyError>;
}
