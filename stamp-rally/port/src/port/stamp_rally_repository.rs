use entity::{StampRally, StampRallyId};
use thiserror::Error;

#[derive(Clone, Debug, Error, Eq, PartialEq)]
#[error("stamp rally repository error")]
pub struct StampRallyRepositoryError;

pub trait StampRallyRepository {
    fn find_by_id(
        &self,
        stamp_rally_id: StampRallyId,
    ) -> Result<Option<StampRally>, StampRallyRepositoryError>;
    fn save(&self, stamp_rally: StampRally) -> Result<(), StampRallyRepositoryError>;
}
