use std::{fmt::Display, str::FromStr};

use event_store_core::event_stream_id::EventStreamId;

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("invalid format {0}")]
    InvalidFormat(String),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct UserRequestId(EventStreamId);

impl UserRequestId {
    pub fn generate() -> Self {
        Self(EventStreamId::generate())
    }
}

impl Display for UserRequestId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<UserRequestId> for u128 {
    fn from(value: UserRequestId) -> Self {
        u128::from(value.0)
    }
}

impl FromStr for UserRequestId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        EventStreamId::from_str(s)
            .map(Self)
            .map_err(|e| Error::InvalidFormat(e.to_string()))
    }
}

impl TryFrom<String> for UserRequestId {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        EventStreamId::from_str(&value)
            .map(Self)
            .map_err(|e| Error::InvalidFormat(e.to_string()))
    }
}

impl TryFrom<u128> for UserRequestId {
    type Error = Error;

    fn try_from(value: u128) -> Result<Self, Self::Error> {
        EventStreamId::try_from(value)
            .map(Self)
            .map_err(|e| Error::InvalidFormat(e.to_string()))
    }
}

impl From<UserRequestId> for EventStreamId {
    fn from(value: UserRequestId) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_stream_id_conversion_test() -> anyhow::Result<()> {
        let event_stream_id = EventStreamId::generate();
        let id1 = UserRequestId::try_from(u128::from(event_stream_id))?;
        assert_eq!(
            EventStreamId::from(id1),
            EventStreamId::try_from(u128::from(event_stream_id))?
        );
        Ok(())
    }

    #[test]
    fn string_conversion_test() -> anyhow::Result<()> {
        let s = EventStreamId::generate().to_string();
        let id1: UserRequestId = s.parse()?;
        assert_eq!(id1.to_string(), s);
        let id2 = UserRequestId::try_from(s)?;
        assert_eq!(id1, id2);
        Ok(())
    }

    #[test]
    fn u128_conversion_test() -> anyhow::Result<()> {
        let event_stream_id = EventStreamId::generate();
        let id1 = UserRequestId::try_from(u128::from(event_stream_id))?;
        assert_eq!(u128::from(id1), u128::from(event_stream_id));
        Ok(())
    }

    #[test]
    fn generate_test() {
        let id1 = UserRequestId::generate();
        let id2 = UserRequestId::generate();
        assert_ne!(id1, id2);
    }
}
