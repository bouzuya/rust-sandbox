use thiserror::Error;
use ulid::Ulid;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct StampCardId(Ulid);

#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("parse stamp card id error")]
pub struct ParseStampCardIdError;

impl StampCardId {
    pub fn generate() -> Self {
        Self(Ulid::new())
    }
}

impl std::str::FromStr for StampCardId {
    type Err = ParseStampCardIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Ulid::from_str(s).map_err(|_| ParseStampCardIdError)?))
    }
}

impl std::fmt::Display for StampCardId {
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
        assert_eq!(StampCardId::from_str(s)?.to_string(), s.to_string());
        Ok(())
    }

    #[test]
    fn generate_test() {
        let id1 = StampCardId::generate();
        let id2 = StampCardId::generate();
        assert_ne!(id1, id2);
    }
}
