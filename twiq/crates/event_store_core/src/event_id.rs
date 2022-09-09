use std::{fmt::Display, str::FromStr};

use crate::uuid_v4::UuidV4;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid format: {0}")]
    InvalidFormat(String),
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct EventId(UuidV4);

impl EventId {
    pub fn generate() -> Self {
        Self(UuidV4::generate())
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
        UuidV4::from_str(s)
            .map(Self)
            .map_err(|e| Error::InvalidFormat(e.to_string()))
    }
}

impl TryFrom<u128> for EventId {
    type Error = Error;

    fn try_from(value: u128) -> Result<Self, Self::Error> {
        UuidV4::try_from(value)
            .map(Self)
            .map_err(|e| Error::InvalidFormat(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use uuid::{Builder, Variant};

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        assert_ne!(EventId::generate(), EventId::generate());
        let s = "70ec72e5-7fd8-4681-abfa-d60a9aa993c2";
        assert_eq!(EventId::from_str(s)?.to_string(), s);

        let uuid = UuidV4::generate();
        assert!(EventId::try_from(uuid.to_u128()).is_ok());
        assert!(EventId::try_from(
            Builder::from_u128(uuid.to_u128())
                .with_variant(Variant::Microsoft)
                .as_uuid()
                .as_u128()
        )
        .is_err());
        assert!(EventId::try_from(
            Builder::from_u128(uuid.to_u128())
                .with_version(uuid::Version::Md5)
                .as_uuid()
                .as_u128()
        )
        .is_err());

        Ok(())
    }
}
