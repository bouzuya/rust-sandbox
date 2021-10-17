use std::convert::TryFrom;

use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Instant(u64);

#[derive(Debug, Eq, Error, PartialEq)]
pub enum ParseInstantError {
    #[error("invalid format")]
    InvalidFormat,
    #[error("out of range")]
    OutOfRange,
}

#[derive(Debug, Eq, Error, PartialEq)]
pub enum TryFromInstantError {
    #[error("out of range")]
    OutOfRange,
}

impl std::fmt::Display for Instant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for Instant {
    type Err = ParseInstantError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let timestamp = s
            .parse::<u64>()
            .map_err(|_| ParseInstantError::InvalidFormat)?;
        Instant::try_from(timestamp).map_err(|_| ParseInstantError::OutOfRange)
    }
}

impl std::convert::TryFrom<u64> for Instant {
    type Error = TryFromInstantError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value > 253_402_300_799_u64 {
            return Err(TryFromInstantError::OutOfRange);
        }
        Ok(Self(value))
    }
}

impl From<Instant> for u64 {
    fn from(instant: Instant) -> Self {
        instant.0
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn u64_conversion_test() -> anyhow::Result<()> {
        assert_eq!(u64::from(Instant::try_from(0_u64)?), 0_u64);
        assert_eq!(
            u64::from(Instant::try_from(253_402_300_799_u64)?),
            253_402_300_799_u64
        );
        assert!(Instant::try_from(253_402_300_799_u64 + 1).is_err());
        Ok(())
    }

    #[test]
    fn str_conversion_test() {
        type E = ParseInstantError;
        let f = |s: &str| Instant::from_str(s);

        assert!(matches!(f("0"), Ok(_)));
        assert!(matches!(f("253402300799"), Ok(_)));
        assert!(matches!(f("a"), Err(E::InvalidFormat)));
        assert!(matches!(f("18446744073709551616"), Err(E::InvalidFormat)));
        assert!(matches!(f("253402300800"), Err(E::OutOfRange)));

        assert_eq!(f("0").map(|d| d.to_string()), Ok("0".to_string()));
    }
}