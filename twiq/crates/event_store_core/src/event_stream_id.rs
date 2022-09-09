use std::{fmt::Display, str::FromStr};

use crate::uuid_v4::UuidV4;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid format: {0}")]
    InvalidFormat(String),
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct EventStreamId(UuidV4);

impl EventStreamId {
    pub fn generate() -> Self {
        Self(UuidV4::generate())
    }
}

impl Display for EventStreamId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<EventStreamId> for u128 {
    fn from(event_stream_id: EventStreamId) -> Self {
        event_stream_id.0.to_u128()
    }
}

impl FromStr for EventStreamId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        UuidV4::from_str(s)
            .map(Self)
            .map_err(|e| Error::InvalidFormat(e.to_string()))
    }
}

impl TryFrom<u128> for EventStreamId {
    type Error = Error;

    fn try_from(value: u128) -> Result<Self, Self::Error> {
        UuidV4::try_from(value)
            .map(Self)
            .map_err(|e| Error::InvalidFormat(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;

    #[test]
    fn u128_conversion_test() -> anyhow::Result<()> {
        let uuid = Uuid::from_str("d271588f-6022-4a41-b636-04a160e4bb1a")?;
        let id1 = EventStreamId::try_from(uuid.as_u128())?;
        assert_eq!(u128::from(id1), uuid.as_u128());
        Ok(())
    }

    #[test]
    fn test() -> anyhow::Result<()> {
        assert_ne!(EventStreamId::generate(), EventStreamId::generate());
        let s = "70ec72e5-7fd8-4681-abfa-d60a9aa993c2";
        assert_eq!(EventStreamId::from_str(s)?.to_string(), s);
        Ok(())
    }
}
