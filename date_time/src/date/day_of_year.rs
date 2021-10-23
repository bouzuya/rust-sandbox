use std::convert::TryFrom;
use thiserror::Error;

use crate::Days;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DayOfYear(u16);

impl DayOfYear {
    pub fn max() -> DayOfYear {
        Self::max_in_leap_year()
    }

    pub fn max_in_common_year() -> DayOfYear {
        DayOfYear(365)
    }

    pub fn max_in_leap_year() -> DayOfYear {
        DayOfYear(366)
    }

    pub fn min() -> DayOfYear {
        DayOfYear(1)
    }

    pub fn days(&self) -> Days {
        Days::from(1)
    }

    pub fn pred(&self) -> Option<Self> {
        if self.0 > 1 {
            Some(Self(self.0 - 1))
        } else {
            None
        }
    }

    pub fn succ(&self) -> Option<Self> {
        if self.0 < 366 {
            Some(Self(self.0 + 1))
        } else {
            None
        }
    }
}

#[derive(Debug, Eq, Error, PartialEq)]
pub enum ParseDayOfYearError {
    #[error("invalid digit")]
    InvalidDigit,
    #[error("invalid length")]
    InvalidLength,
    #[error("out of range")]
    OutOfRange,
}

#[derive(Debug, Eq, Error, PartialEq)]
pub enum TryFromDayOfYearError {
    #[error("out of range")]
    OutOfRange,
}

impl std::fmt::Display for DayOfYear {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:03}", self.0)
    }
}

impl std::str::FromStr for DayOfYear {
    type Err = ParseDayOfYearError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 3 {
            return Err(Self::Err::InvalidLength);
        }
        let mut doy = 0_u16;
        for c in s.chars() {
            let d = match c {
                '0'..='9' => c as u8 - b'0',
                _ => return Err(Self::Err::InvalidDigit),
            };
            doy = doy * 10 + d as u16;
        }
        Self::try_from(doy).map_err(|_| Self::Err::OutOfRange)
    }
}

impl From<DayOfYear> for u16 {
    fn from(day_of_month: DayOfYear) -> Self {
        day_of_month.0
    }
}

impl std::convert::TryFrom<u16> for DayOfYear {
    type Error = TryFromDayOfYearError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if !(1..=366).contains(&value) {
            return Err(Self::Error::OutOfRange);
        }
        Ok(Self(value))
    }
}

impl std::ops::Add<Days> for DayOfYear {
    type Output = DayOfYear;

    fn add(self, rhs: Days) -> Self::Output {
        u32::from(self.0)
            .checked_add(u32::from(rhs))
            .and_then(|d| u16::try_from(d).ok())
            .and_then(|d| DayOfYear::try_from(d).ok())
            .unwrap_or_else(|| panic!("overflow"))
    }
}

impl std::ops::Add<DayOfYear> for Days {
    type Output = DayOfYear;

    fn add(self, rhs: DayOfYear) -> Self::Output {
        rhs + self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_conversion_test() {
        type E = ParseDayOfYearError;
        let f = |s: &str| s.parse::<DayOfYear>();
        assert_eq!(f("001").map(|d| d.to_string()), Ok("001".to_string()));
        assert_eq!(f("366").map(|d| d.to_string()), Ok("366".to_string()));
        assert_eq!(f(""), Err(E::InvalidLength));
        assert_eq!(f("1"), Err(E::InvalidLength));
        assert_eq!(f("12"), Err(E::InvalidLength));
        assert_eq!(f("1234"), Err(E::InvalidLength));
        assert_eq!(f("00a"), Err(E::InvalidDigit));
        assert_eq!(f("+12"), Err(E::InvalidDigit));
        assert_eq!(f("000"), Err(E::OutOfRange));
        assert_eq!(f("367"), Err(E::OutOfRange));
    }

    #[test]
    fn u16_conversion_test() {
        type E = TryFromDayOfYearError;
        let f = |d: u16| DayOfYear::try_from(d);
        assert_eq!(f(0_u16), Err(E::OutOfRange));
        assert_eq!(f(1_u16).map(u16::from), Ok(1_u16));
        assert_eq!(f(366_u16).map(u16::from), Ok(366_u16));
        assert_eq!(f(367_u16), Err(E::OutOfRange));
    }

    #[test]
    fn max_min_test() -> anyhow::Result<()> {
        assert_eq!(DayOfYear::max(), DayOfYear::try_from(366)?);
        assert_eq!(DayOfYear::max_in_common_year(), DayOfYear::try_from(365)?);
        assert_eq!(DayOfYear::max_in_leap_year(), DayOfYear::try_from(366)?);
        assert_eq!(DayOfYear::min(), DayOfYear::try_from(1)?);
        Ok(())
    }

    #[test]
    fn pred_test() -> anyhow::Result<()> {
        assert_eq!(
            DayOfYear::try_from(366)?.pred(),
            Some(DayOfYear::try_from(365)?)
        );
        assert_eq!(
            DayOfYear::try_from(2)?.pred(),
            Some(DayOfYear::try_from(1)?)
        );
        assert_eq!(DayOfYear::try_from(1)?.pred(), None);
        Ok(())
    }

    #[test]
    fn succ_test() -> anyhow::Result<()> {
        assert_eq!(
            DayOfYear::try_from(1)?.succ(),
            Some(DayOfYear::try_from(2)?)
        );
        assert_eq!(
            DayOfYear::try_from(365)?.succ(),
            Some(DayOfYear::try_from(366)?)
        );
        assert_eq!(DayOfYear::try_from(366)?.succ(), None);
        Ok(())
    }

    #[test]
    fn add_days_test() -> anyhow::Result<()> {
        let d1 = DayOfYear::try_from(1)?;
        let d2 = DayOfYear::try_from(2)?;
        let d366 = DayOfYear::try_from(366)?;
        assert_eq!(d1 + Days::from(0), d1);
        assert_eq!(d1 + Days::from(1), d2);
        assert_eq!(d1 + Days::from(365), d366);
        // should_panic
        // assert_eq!(d1 + Days::from(366), d31);
        assert_eq!(Days::from(0) + d1, d1);
        assert_eq!(Days::from(1) + d1, d2);
        assert_eq!(Days::from(365) + d1, d366);
        Ok(())
    }

    #[test]
    fn days_test() -> anyhow::Result<()> {
        assert_eq!(DayOfYear::try_from(1)?.days(), Days::from(1));
        Ok(())
    }
}
