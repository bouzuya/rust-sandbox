use std::{fmt::Display, str::FromStr};

use event_store_core::event_stream_id::EventStreamId;

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("invalid format {0}")]
    InvalidFormat(String),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct UserId(EventStreamId);

impl UserId {
    pub fn generate() -> Self {
        Self(EventStreamId::generate())
    }
}

impl Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<UserId> for u128 {
    fn from(value: UserId) -> Self {
        u128::from(value.0)
    }
}

impl FromStr for UserId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        EventStreamId::from_str(s)
            .map(Self)
            .map_err(|e| Error::InvalidFormat(e.to_string()))
    }
}

impl TryFrom<String> for UserId {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        EventStreamId::from_str(&value)
            .map(Self)
            .map_err(|e| Error::InvalidFormat(e.to_string()))
    }
}

impl TryFrom<u128> for UserId {
    type Error = Error;

    fn try_from(value: u128) -> Result<Self, Self::Error> {
        EventStreamId::try_from(value)
            .map(Self)
            .map_err(|e| Error::InvalidFormat(e.to_string()))
    }
}

impl From<UserId> for EventStreamId {
    fn from(value: UserId) -> Self {
        value.0
    }
}

impl From<EventStreamId> for UserId {
    fn from(value: EventStreamId) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_stream_id_conversion_test() -> anyhow::Result<()> {
        let event_stream_id = EventStreamId::generate();
        let id1 = UserId::try_from(u128::from(event_stream_id))?;
        assert_eq!(UserId::from(EventStreamId::from(id1)), id1);
        Ok(())
    }

    #[test]
    fn string_conversion_test() -> anyhow::Result<()> {
        let s = EventStreamId::generate().to_string();
        let id1: UserId = s.parse()?;
        assert_eq!(id1.to_string(), s);
        let id2 = UserId::try_from(s)?;
        assert_eq!(id1, id2);
        Ok(())
    }

    #[test]
    fn u128_conversion_test() -> anyhow::Result<()> {
        let event_stream_id = EventStreamId::generate();
        let id1 = UserId::try_from(u128::from(event_stream_id))?;
        assert_eq!(u128::from(id1), u128::from(event_stream_id));
        Ok(())
    }

    #[test]
    fn generate_test() {
        let id1 = UserId::generate();
        let id2 = UserId::generate();
        assert_ne!(id1, id2);
    }
}
