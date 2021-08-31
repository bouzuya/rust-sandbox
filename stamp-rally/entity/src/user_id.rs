use thiserror::Error;
use ulid::Ulid;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UserId(Ulid);

#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("parse user id error")]
pub struct ParseUserIdError;

impl UserId {
    pub fn generate() -> Self {
        Self(Ulid::new())
    }
}

impl std::str::FromStr for UserId {
    type Err = ParseUserIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Ulid::from_str(s).map_err(|_| ParseUserIdError)?))
    }
}

impl std::fmt::Display for UserId {
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
        assert_eq!(UserId::from_str(s)?.to_string(), s.to_string());
        Ok(())
    }

    #[test]
    fn generate_test() {
        let id1 = UserId::generate();
        let id2 = UserId::generate();
        assert_ne!(id1, id2);
    }
}
