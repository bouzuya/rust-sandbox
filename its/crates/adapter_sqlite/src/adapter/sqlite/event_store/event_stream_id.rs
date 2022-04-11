use std::{fmt::Display, str::FromStr};

use ulid::Ulid;

use super::event_store_error::EventStoreError;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct EventStreamId(Ulid);

impl EventStreamId {
    pub fn generate() -> Self {
        Self(Ulid::new())
    }
}

impl Display for EventStreamId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for EventStreamId {
    type Err = EventStoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ulid = Ulid::from_str(s).map_err(|_| EventStoreError::InvalidEventStreamId)?;
        Ok(Self(ulid))
    }
}
