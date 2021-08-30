use entity::{StampRally, StampRallyId};
use thiserror::Error;

#[derive(Debug, Eq, Error, PartialEq)]
#[error("create stamp rally error")]
pub struct CreateStampRallyError;

pub fn create_stamp_rally() -> Result<StampRallyId, CreateStampRallyError> {
    let stamp_rally = StampRally::new();
    Ok(stamp_rally.id())
}
