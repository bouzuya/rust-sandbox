use std::convert::TryFrom;
use thiserror::Error;

use crate::{DayOfYear, Days};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Year(u16);

impl Year {
    pub fn first_day_of_year(&self) -> DayOfYear {
        DayOfYear::min()
    }

    pub fn last_day_of_year(&self) -> DayOfYear {
        if self.is_leap_year() {
            DayOfYear::max_in_leap_year()
        } else {
            DayOfYear::max_in_common_year()
        }
    }

    pub fn days(&self) -> Days {
        Days::from(if self.is_leap_year() { 366 } else { 365 })
    }

    pub fn pred(&self) -> Option<Self> {
        if self.0 > 1970 {
            Some(Self(self.0 - 1))
        } else {
            None
        }
    }

    pub fn succ(&self) -> Option<Self> {
        if self.0 < 9999 {
            Some(Self(self.0 + 1))
        } else {
            None
        }
    }
}

#[derive(Debug, Eq, Error, PartialEq)]
pub enum ParseYearError {
    #[error("invalid digit")]
    InvalidDigit,
    #[error("invalid length")]
    InvalidLength,
    #[error("out of range")]
    OutOfRange,
}

#[derive(Debug, Eq, Error, PartialEq)]
pub enum TryFromYearError {
    #[error("out of range")]
    OutOfRange,
}

impl Year {
    pub fn is_leap_year(&self) -> bool {
        (self.0 % 400 == 0) || ((self.0 % 100 != 0) && (self.0 % 4 == 0))
    }
}

impl std::fmt::Display for Year {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04}", self.0)
    }
}

impl std::str::FromStr for Year {
    type Err = ParseYearError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err(Self::Err::InvalidLength);
        }
        let mut y = 0_u16;
        for c in s.chars() {
            let d = match c {
                '0'..='9' => c as u16 - '0' as u16,
                _ => return Err(Self::Err::InvalidDigit),
            };
            y = y * 10 + d;
        }
        Self::try_from(y).map_err(|_| Self::Err::OutOfRange)
    }
}

impl From<Year> for u16 {
    fn from(year: Year) -> Self {
        year.0
    }
}

impl std::convert::TryFrom<u16> for Year {
    type Error = TryFromYearError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if !(1970..=9999).contains(&value) {
            return Err(Self::Error::OutOfRange);
        }
        Ok(Self(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_leap_year() {
        let f = |y: u16| Year::try_from(y).map(|y| Year::is_leap_year(&y));
        assert_eq!(f(2000), Ok(true));
        assert_eq!(f(2004), Ok(true));
        assert_eq!(f(2100), Ok(false));
    }

    #[test]
    fn str_conversion_test() {
        type E = ParseYearError;
        let f = |s: &str| s.parse::<Year>();
        assert_eq!(f("1970").map(|y| y.to_string()), Ok("1970".to_string()));
        assert_eq!(f("9999").map(|y| y.to_string()), Ok("9999".to_string()));
        assert_eq!(f(""), Err(E::InvalidLength));
        assert_eq!(f("0"), Err(E::InvalidLength));
        assert_eq!(f("10000"), Err(E::InvalidLength));
        assert_eq!(f("000a"), Err(E::InvalidDigit));
        assert_eq!(f("+000"), Err(E::InvalidDigit));
        assert_eq!(f("1969"), Err(E::OutOfRange));
    }

    #[test]
    fn u16_conversion_test() {
        type E = TryFromYearError;
        let f = |y: u16| Year::try_from(y);
        assert_eq!(f(1969_u16), Err(E::OutOfRange));
        assert_eq!(f(1970_u16).map(u16::from), Ok(1970_u16));
        assert_eq!(f(9999_u16).map(u16::from), Ok(9999_u16));
        assert_eq!(f(10000_u16), Err(E::OutOfRange));
    }

    #[test]
    fn first_day_of_year_test() -> anyhow::Result<()> {
        assert_eq!(Year::try_from(2000)?.first_day_of_year(), DayOfYear::min());
        Ok(())
    }

    #[test]
    fn last_day_of_year_test() -> anyhow::Result<()> {
        let leap_year = Year::try_from(2000)?;
        assert_eq!(leap_year.last_day_of_year(), DayOfYear::max_in_leap_year());
        let common_year = Year::try_from(2001)?;
        assert_eq!(
            common_year.last_day_of_year(),
            DayOfYear::max_in_common_year()
        );
        Ok(())
    }

    #[test]
    fn pred_test() -> anyhow::Result<()> {
        assert_eq!(Year::try_from(9999)?.pred(), Some(Year::try_from(9998)?));
        assert_eq!(Year::try_from(1971)?.pred(), Some(Year::try_from(1970)?));
        assert_eq!(Year::try_from(1970)?.pred(), None);
        Ok(())
    }

    #[test]
    fn succ_test() -> anyhow::Result<()> {
        assert_eq!(Year::try_from(1970)?.succ(), Some(Year::try_from(1971)?));
        assert_eq!(Year::try_from(9998)?.succ(), Some(Year::try_from(9999)?));
        assert_eq!(Year::try_from(9999)?.succ(), None);
        Ok(())
    }

    #[test]
    fn days_test() -> anyhow::Result<()> {
        assert_eq!(Year::try_from(2000)?.days(), Days::from(366));
        assert_eq!(Year::try_from(2001)?.days(), Days::from(365));
        Ok(())
    }
}
