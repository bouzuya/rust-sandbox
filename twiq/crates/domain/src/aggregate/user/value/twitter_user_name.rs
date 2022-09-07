use std::{fmt::Display, str::FromStr};

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("invalid format")]
    InvalidFormat,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TwitterUserName(String);

impl Display for TwitterUserName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for TwitterUserName {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s.to_owned())
    }
}

impl TryFrom<String> for TwitterUserName {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        // 十分な長さを上限に設定している
        if value.len() > 1024 {
            return Err(Error::InvalidFormat);
        }
        Ok(Self(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_conversion_test() -> anyhow::Result<()> {
        let s = "bouzuya";
        let id1: TwitterUserName = s.parse()?;
        assert_eq!(s.to_string(), s);
        let id2 = TwitterUserName::try_from(s.to_owned())?;
        assert_eq!(id1, id2);
        Ok(())
    }
}
