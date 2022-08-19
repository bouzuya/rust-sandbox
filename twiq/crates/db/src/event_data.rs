use std::{fmt::Display, str::FromStr};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("too large")]
    TooLarge,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EventData(String);

impl From<EventData> for String {
    fn from(value: EventData) -> Self {
        value.0
    }
}

impl TryFrom<String> for EventData {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.as_bytes().len() > 1_000_000 {
            return Err(Error::TooLarge);
        }
        Ok(Self(value.to_string()))
    }
}

impl Display for EventData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for EventData {
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
        assert_eq!(String::from(EventData::try_from("a".to_string())?), "a");
        assert!(EventData::try_from("a".repeat(1_000_000)).is_ok());
        assert!(EventData::try_from("a".repeat(1_000_001)).is_err());
        assert_eq!(EventData::from_str("a")?.to_string(), "a");
        Ok(())
    }
}
