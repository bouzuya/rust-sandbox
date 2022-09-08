use std::{fmt::Display, str::FromStr};

use uuid::{Uuid, Variant};

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("invalid format {0}")]
    InvalidFormat(String),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UserRequestId(Uuid);

impl UserRequestId {
    pub fn generate() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Display for UserRequestId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<UserRequestId> for u128 {
    fn from(user_id: UserRequestId) -> Self {
        user_id.0.as_u128()
    }
}

impl FromStr for UserRequestId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::parse_str(s)
            .map(Self)
            .map_err(|e| Error::InvalidFormat(e.to_string()))
    }
}

impl TryFrom<String> for UserRequestId {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Uuid::parse_str(&value)
            .map(Self)
            .map_err(|e| Error::InvalidFormat(e.to_string()))
    }
}

impl TryFrom<u128> for UserRequestId {
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
    fn string_conversion_test() -> anyhow::Result<()> {
        let s = "d271588f-6022-4a41-b636-04a160e4bb1a";
        let id1: UserRequestId = s.parse()?;
        assert_eq!(id1.to_string(), s);
        let id2 = UserRequestId::try_from(s.to_owned())?;
        assert_eq!(id1, id2);
        Ok(())
    }

    #[test]
    fn u128_conversion_test() -> anyhow::Result<()> {
        let uuid = Uuid::from_str("d271588f-6022-4a41-b636-04a160e4bb1a")?;
        let id1 = UserRequestId::try_from(uuid.as_u128())?;
        assert_eq!(u128::from(id1), uuid.as_u128());
        Ok(())
    }

    #[test]
    fn generate_test() {
        let id1 = UserRequestId::generate();
        let id2 = UserRequestId::generate();
        assert_ne!(id1, id2);
    }
}
