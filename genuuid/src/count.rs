use std::convert::TryFrom;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("parse count error")]
pub struct ParseCountError;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Count(usize);

impl Default for Count {
    fn default() -> Self {
        Self(1)
    }
}

impl std::convert::TryFrom<usize> for Count {
    type Error = ParseCountError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if (1..=100).contains(&value) {
            Ok(Self(value))
        } else {
            Err(ParseCountError)
        }
    }
}

impl std::fmt::Display for Count {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for Count {
    type Err = ParseCountError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n = usize::from_str(s).map_err(|_| ParseCountError)?;
        Self::try_from(n)
    }
}

impl std::convert::From<Count> for usize {
    fn from(count: Count) -> Self {
        count.0
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn default_test() -> anyhow::Result<()> {
        assert_eq!(usize::from(Count::default()), 1_usize);
        Ok(())
    }

    #[test]
    fn str_conversion_test() -> anyhow::Result<()> {
        assert!(Count::from_str("0").is_err());
        assert_eq!(Count::from_str("1")?.to_string(), "1");
        assert_eq!(Count::from_str("100")?.to_string(), "100");
        assert!(Count::from_str("101").is_err());
        Ok(())
    }

    #[test]
    fn usize_conversion_test() -> anyhow::Result<()> {
        assert!(Count::try_from(0_usize).is_err());
        assert_eq!(usize::from(Count::try_from(1_usize)?), 1_usize);
        assert_eq!(usize::from(Count::try_from(100_usize)?), 100_usize);
        assert!(Count::try_from(101_usize).is_err());
        Ok(())
    }
}
