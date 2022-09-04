use std::{fmt::Display, str::FromStr};

use uuid::{Uuid, Variant};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid format: {0}")]
    InvalidFormat(String),
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct EventId(Uuid);

impl EventId {
    pub fn generate() -> Self {
        Self(Uuid::new_v4())
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
        let uuid = Uuid::from_str(s).map_err(|e| Error::InvalidFormat(e.to_string()))?;
        Ok(Self(uuid))
    }
}

impl TryFrom<u128> for EventId {
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
    use uuid::Builder;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        assert_ne!(EventId::generate(), EventId::generate());
        let s = "70ec72e5-7fd8-4681-abfa-d60a9aa993c2";
        assert_eq!(EventId::from_str(s)?.to_string(), s);

        let uuid = Uuid::new_v4();
        assert!(EventId::try_from(uuid.as_u128()).is_ok());
        assert!(EventId::try_from(Uuid::nil().as_u128()).is_err());
        assert!(EventId::try_from(
            Builder::from_u128(uuid.as_u128())
                .with_variant(Variant::Microsoft)
                .as_uuid()
                .as_u128()
        )
        .is_err());
        assert!(EventId::try_from(
            Builder::from_u128(uuid.as_u128())
                .with_version(uuid::Version::Md5)
                .as_uuid()
                .as_u128()
        )
        .is_err());

        Ok(())
    }
}
