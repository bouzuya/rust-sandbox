use std::{fmt::Display, str::FromStr};

use uuid::{Uuid, Variant};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid format: {0}")]
    InvalidFormat(String),
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct EventStreamId(Uuid);

impl EventStreamId {
    pub fn generate() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Display for EventStreamId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<EventStreamId> for u128 {
    fn from(event_stream_id: EventStreamId) -> Self {
        event_stream_id.0.as_u128()
    }
}

impl FromStr for EventStreamId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uuid = Uuid::from_str(s).map_err(|e| Error::InvalidFormat(e.to_string()))?;
        Ok(Self(uuid))
    }
}

impl TryFrom<u128> for EventStreamId {
    type Error = Error;

    fn try_from(value: u128) -> Result<Self, Self::Error> {
        let uuid = Uuid::from_u128(value);
        if !(uuid.get_version_num() == 4 && uuid.get_variant() == Variant::RFC4122) {
            return Err(Error::InvalidFormat("u128 value is not UUID v4".to_owned()));
        }
        Ok(Self(uuid))
    }
}

#[cfg(test)]
mod tests {
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
