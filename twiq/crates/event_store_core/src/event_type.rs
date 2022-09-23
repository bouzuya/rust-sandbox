use std::{fmt::Display, str::FromStr};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid chars")]
    InvalidChars,
    #[error("too long")]
    TooLong,
    #[error("too short")]
    TooShort,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EventType(String);

impl EventType {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<EventType> for String {
    fn from(value: EventType) -> Self {
        value.0
    }
}

impl TryFrom<String> for EventType {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let bytes = value.as_bytes();
        if bytes.len() >= 64 {
            return Err(Error::TooLong);
        }
        if bytes.is_empty() {
            return Err(Error::TooShort);
        }
        if !bytes.is_ascii()
            || !bytes
                .iter()
                .all(|b| (b'a'..=b'z').contains(b) || (b'0'..=b'9').contains(b) || b == &b'_')
        {
            return Err(Error::InvalidChars);
        }
        Ok(Self(value))
    }
}

impl Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for EventType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        assert_eq!(String::from(EventType::try_from("a".to_string())?), "a");
        assert!(EventType::try_from("a".repeat(63)).is_ok());
        assert!(EventType::try_from("a".repeat(64)).is_err());
        assert!(EventType::try_from("".to_owned()).is_err());
        assert!(EventType::try_from("abc_123".to_owned()).is_ok());
        assert!(EventType::try_from("Abc_123".to_owned()).is_err());
        assert!(EventType::try_from("abc 123".to_owned()).is_err());
        assert_eq!(EventType::from_str("a")?.to_string(), "a");
        assert_eq!(EventType::from_str("a")?.as_str(), "a");
        Ok(())
    }
}
