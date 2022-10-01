use std::{fmt::Display, str::FromStr};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("deserialize {0}")]
    Deserialize(String),
    #[error("serialize {0}")]
    Serialize(String),
    #[error("too large")]
    TooLarge,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EventPayload(String);

impl EventPayload {
    pub fn from_structured<T: serde::Serialize>(value: &T) -> Result<Self, Error> {
        serde_json::to_string(value)
            .map(Self)
            .map_err(|e| Error::Serialize(e.to_string()))
    }

    pub fn to_structured<T: serde::de::DeserializeOwned>(&self) -> Result<T, Error> {
        serde_json::from_str(self.0.as_str()).map_err(|e| Error::Deserialize(e.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<EventPayload> for String {
    fn from(value: EventPayload) -> Self {
        value.0
    }
}

impl TryFrom<String> for EventPayload {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.as_bytes().len() > 1_000_000 {
            return Err(Error::TooLarge);
        }
        Ok(Self(value))
    }
}

impl Display for EventPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for EventPayload {
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
        assert_eq!(String::from(EventPayload::try_from("a".to_string())?), "a");
        assert!(EventPayload::try_from("a".repeat(1_000_000)).is_ok());
        assert!(EventPayload::try_from("a".repeat(1_000_001)).is_err());
        assert_eq!(EventPayload::from_str("a")?.to_string(), "a");
        assert_eq!(EventPayload::from_str("a")?.as_str(), "a");
        Ok(())
    }

    // TODO: test from_structured & to_structured
}
