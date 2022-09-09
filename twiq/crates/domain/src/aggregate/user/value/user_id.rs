use std::{fmt::Display, str::FromStr};

use super::uuid_v4::UuidV4;

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("invalid format {0}")]
    InvalidFormat(String),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UserId(UuidV4);

impl UserId {
    pub fn generate() -> Self {
        Self(UuidV4::generate())
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
        UuidV4::from_str(s)
            .map(Self)
            .map_err(|e| Error::InvalidFormat(e.to_string()))
    }
}

impl TryFrom<String> for UserId {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        UuidV4::try_from(value)
            .map(Self)
            .map_err(|e| Error::InvalidFormat(e.to_string()))
    }
}

impl TryFrom<u128> for UserId {
    type Error = Error;

    fn try_from(value: u128) -> Result<Self, Self::Error> {
        UuidV4::try_from(value)
            .map(Self)
            .map_err(|e| Error::InvalidFormat(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_conversion_test() -> anyhow::Result<()> {
        let s = UuidV4::generate().to_string();
        let id1: UserId = s.parse()?;
        assert_eq!(id1.to_string(), s);
        let id2 = UserId::try_from(s)?;
        assert_eq!(id1, id2);
        Ok(())
    }

    #[test]
    fn u128_conversion_test() -> anyhow::Result<()> {
        let uuid = UuidV4::generate();
        let id1 = UserId::try_from(uuid.to_u128())?;
        assert_eq!(u128::from(id1), uuid.to_u128());
        Ok(())
    }

    #[test]
    fn generate_test() {
        let id1 = UserId::generate();
        let id2 = UserId::generate();
        assert_ne!(id1, id2);
    }
}
