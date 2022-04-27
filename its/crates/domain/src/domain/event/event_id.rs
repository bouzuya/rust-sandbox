use std::{fmt::Display, str::FromStr};

use ulid::Ulid;

#[derive(Debug, thiserror::Error)]
#[error("EventIdError")]
pub struct Error;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct EventId(Ulid);

impl EventId {
    pub fn generate() -> Self {
        Self(Ulid::new())
    }
}

impl Display for EventId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for EventId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ulid = Ulid::from_str(s).map_err(|_| Error)?;
        Ok(Self(ulid))
    }
}
