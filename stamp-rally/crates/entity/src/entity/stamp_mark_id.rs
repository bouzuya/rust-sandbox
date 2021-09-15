use thiserror::Error;
use ulid::Ulid;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct StampMarkId(Ulid);

#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("parse stamp mark id error")]
pub struct ParseStampMarkIdError;

impl StampMarkId {
    pub fn generate() -> Self {
        Self(Ulid::new())
    }
}

impl std::str::FromStr for StampMarkId {
    type Err = ParseStampMarkIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Ulid::from_str(s).map_err(|_| ParseStampMarkIdError)?))
    }
}

impl std::fmt::Display for StampMarkId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn string_convertion_test() -> anyhow::Result<()> {
        let s = "01D39ZY06FGSCTVN4T2V9PKHFZ";
        assert_eq!(StampMarkId::from_str(s)?.to_string(), s.to_string());
        Ok(())
    }

    #[test]
    fn generate_test() {
        let id1 = StampMarkId::generate();
        let id2 = StampMarkId::generate();
        assert_ne!(id1, id2);
    }
}
