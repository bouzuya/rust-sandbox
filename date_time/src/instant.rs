use std::{convert::TryFrom, ops::Add};

use thiserror::Error;

use crate::{Days, Seconds};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Instant(u64);

impl Instant {
    pub fn max() -> Self {
        Self(253_402_300_799_u64)
    }

    pub fn min() -> Self {
        Self(0_u64)
    }

    pub fn now() -> Self {
        let timestamp = chrono::Utc::now().timestamp();
        if (0..=i64::from(Self::max())).contains(&timestamp) {
            Self(timestamp as u64)
        } else {
            panic!()
        }
    }
}

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

impl std::convert::TryFrom<i64> for Instant {
    type Error = TryFromInstantError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if (0..=i64::from(Instant::max())).contains(&value) {
            Ok(Self(value as u64))
        } else {
            Err(TryFromInstantError::OutOfRange)
        }
    }
}

impl std::convert::TryFrom<u64> for Instant {
    type Error = TryFromInstantError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if (0..=u64::from(Instant::max())).contains(&value) {
            Ok(Self(value))
        } else {
            Err(TryFromInstantError::OutOfRange)
        }
    }
}

impl From<Instant> for i64 {
    fn from(instant: Instant) -> Self {
        instant.0 as i64
    }
}

impl From<Instant> for u64 {
    fn from(instant: Instant) -> Self {
        instant.0
    }
}

impl Add<Days> for Instant {
    type Output = Instant;

    fn add(self, rhs: Days) -> Self::Output {
        self + Seconds::from(rhs)
    }
}

impl Add<Seconds> for Instant {
    type Output = Instant;

    fn add(self, rhs: Seconds) -> Self::Output {
        // TODO: unwrap
        Instant::try_from(self.0.checked_add(u64::from(rhs)).unwrap()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn now_test() {
        assert_ne!(Instant::now().to_string(), "");
    }

    #[test]
    fn i64_conversion_test() -> anyhow::Result<()> {
        assert!(Instant::try_from(i64::from(Instant::min()) - 1_i64).is_err());
        assert_eq!(i64::from(Instant::min()), 0_i64);
        assert_eq!(i64::from(Instant::max()), 253_402_300_799_i64);
        assert!(Instant::try_from(i64::from(Instant::max()) + 1_i64).is_err());
        Ok(())
    }

    #[test]
    fn u64_conversion_test() -> anyhow::Result<()> {
        assert_eq!(u64::from(Instant::min()), 0_u64);
        assert_eq!(u64::from(Instant::max()), 253_402_300_799_u64);
        assert!(Instant::try_from(u64::from(Instant::max()) + 1_u64).is_err());
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

    #[test]
    fn add_days_test() -> anyhow::Result<()> {
        let seconds_per_day = 24_u64 * 60_u64 * 60_u64;
        assert_eq!(
            Instant::try_from(1_u64)? + Days::from(1_u32),
            Instant::try_from(1_u64 + seconds_per_day)?
        );
        Ok(())
    }

    #[test]
    fn add_seconds_test() -> anyhow::Result<()> {
        assert_eq!(
            Instant::try_from(1_u64)? + Seconds::from(2_u64),
            Instant::try_from(3_u64)?
        );
        assert_eq!(
            Instant::try_from(0_u64)? + Seconds::from(u64::from(Instant::max())),
            Instant::max()
        );
        Ok(())
    }
}
