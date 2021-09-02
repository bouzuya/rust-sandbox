use entity::{StampRally, StampRallyId};
use port::{CreateStampRallyError, CreateStampRallyUseCase};

pub struct CreateStampRally;

impl CreateStampRally {
    pub fn new() -> Self {
        Self
    }
}

impl CreateStampRallyUseCase for CreateStampRally {
    fn handle(&self) -> Result<StampRallyId, CreateStampRallyError> {
        let stamp_rally = StampRally::new();
        Ok(stamp_rally.id())
    }
}
