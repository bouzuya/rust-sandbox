use thiserror::Error;
use ulid::Ulid;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct StampRallyId(Ulid);

#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("parse stamp rally id error")]
pub struct ParseStampRallyIdError;

impl StampRallyId {
    pub fn generate() -> Self {
        Self(Ulid::new())
    }
}

impl std::str::FromStr for StampRallyId {
    type Err = ParseStampRallyIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Ulid::from_str(s).map_err(|_| ParseStampRallyIdError)?))
    }
}

impl std::fmt::Display for StampRallyId {
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
        assert_eq!(StampRallyId::from_str(s)?.to_string(), s.to_string());
        Ok(())
    }

    #[test]
    fn generate_test() {
        let id1 = StampRallyId::generate();
        let id2 = StampRallyId::generate();
        assert_ne!(id1, id2);
    }
}
