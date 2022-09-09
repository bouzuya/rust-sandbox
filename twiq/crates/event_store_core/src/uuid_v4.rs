use std::{fmt::Display, str::FromStr};

use uuid::{Uuid, Variant};

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub(crate) enum Error {
    #[error("invalid format {0}")]
    InvalidFormat(String),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UuidV4(Uuid);

impl UuidV4 {
    pub(crate) fn generate() -> Self {
        Self(Uuid::new_v4())
    }

    pub(crate) fn parse_str(s: &str) -> Result<Self, Error> {
        let uuid = Uuid::parse_str(s).map_err(|e| Error::InvalidFormat(e.to_string()))?;
        if Self::is_v4(uuid) {
            Ok(Self(uuid))
        } else {
            Err(Error::InvalidFormat("UUID is not v4".to_owned()))
        }
    }

    pub(crate) fn parse_u128(value: u128) -> Result<Self, Error> {
        let uuid = Uuid::from_u128(value);
        if Self::is_v4(uuid) {
            Ok(Self(uuid))
        } else {
            Err(Error::InvalidFormat("UUID is not v4".to_owned()))
        }
    }

    pub(crate) fn to_u128(self) -> u128 {
        self.0.as_u128()
    }

    fn is_v4(uuid: Uuid) -> bool {
        uuid.get_version_num() == 4 && uuid.get_variant() == Variant::RFC4122
    }
}

impl Display for UuidV4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<UuidV4> for u128 {
    fn from(user_id: UuidV4) -> Self {
        user_id.to_u128()
    }
}

impl FromStr for UuidV4 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_str(s)
    }
}

impl TryFrom<String> for UuidV4 {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse_str(&value)
    }
}

impl TryFrom<u128> for UuidV4 {
    type Error = Error;

    fn try_from(value: u128) -> Result<Self, Self::Error> {
        Self::parse_u128(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_conversion_test() -> anyhow::Result<()> {
        let s = "d271588f-6022-4a41-b636-04a160e4bb1a";
        let id1: UuidV4 = s.parse()?;
        assert_eq!(id1.to_string(), s);
        let id2 = UuidV4::try_from(s.to_owned())?;
        assert_eq!(id1, id2);
        Ok(())
    }

    #[test]
    fn u128_conversion_test() -> anyhow::Result<()> {
        let uuid = Uuid::from_str("d271588f-6022-4a41-b636-04a160e4bb1a")?;
        let id1 = UuidV4::try_from(uuid.as_u128())?;
        assert_eq!(u128::from(id1), uuid.as_u128());
        Ok(())
    }

    #[test]
    fn generate_test() {
        let id1 = UuidV4::generate();
        let id2 = UuidV4::generate();
        assert_ne!(id1, id2);
    }
}
