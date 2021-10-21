use std::convert::TryFrom;
use thiserror::Error;

use crate::Days;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DayOfMonth(u8);

impl DayOfMonth {
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
        if self.0 < 31 {
            Some(Self(self.0 + 1))
        } else {
            None
        }
    }
}

#[derive(Debug, Eq, Error, PartialEq)]
pub enum ParseDayOfMonthError {
    #[error("invalid digit")]
    InvalidDigit,
    #[error("invalid length")]
    InvalidLength,
    #[error("out of range")]
    OutOfRange,
}

#[derive(Debug, Eq, Error, PartialEq)]
pub enum TryFromDayOfMonthError {
    #[error("out of range")]
    OutOfRange,
}

impl std::fmt::Display for DayOfMonth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}", self.0)
    }
}

impl std::str::FromStr for DayOfMonth {
    type Err = ParseDayOfMonthError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            return Err(Self::Err::InvalidLength);
        }
        let mut dom = 0_u8;
        for c in s.chars() {
            let d = match c {
                '0'..='9' => c as u8 - b'0',
                _ => return Err(Self::Err::InvalidDigit),
            };
            dom = dom * 10 + d;
        }
        Self::try_from(dom).map_err(|_| Self::Err::OutOfRange)
    }
}

impl From<DayOfMonth> for u8 {
    fn from(day_of_month: DayOfMonth) -> Self {
        day_of_month.0
    }
}

impl std::convert::TryFrom<u8> for DayOfMonth {
    type Error = TryFromDayOfMonthError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(1..=31).contains(&value) {
            return Err(Self::Error::OutOfRange);
        }
        Ok(Self(value))
    }
}

impl std::ops::Add<Days> for DayOfMonth {
    type Output = DayOfMonth;

    fn add(self, rhs: Days) -> Self::Output {
        u32::from(self.0)
            .checked_add(u32::from(rhs))
            .and_then(|d| u8::try_from(d).ok())
            .and_then(|d| DayOfMonth::try_from(d).ok())
            .unwrap_or_else(|| panic!("overflow"))
    }
}

impl std::ops::Add<DayOfMonth> for Days {
    type Output = DayOfMonth;

    fn add(self, rhs: DayOfMonth) -> Self::Output {
        rhs + self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_conversion_test() {
        type E = ParseDayOfMonthError;
        let f = |s: &str| s.parse::<DayOfMonth>();
        assert_eq!(f("01").map(|d| d.to_string()), Ok("01".to_string()));
        assert_eq!(f("31").map(|d| d.to_string()), Ok("31".to_string()));
        assert_eq!(f(""), Err(E::InvalidLength));
        assert_eq!(f("1"), Err(E::InvalidLength));
        assert_eq!(f("100"), Err(E::InvalidLength));
        assert_eq!(f("0a"), Err(E::InvalidDigit));
        assert_eq!(f("+1"), Err(E::InvalidDigit));
        assert_eq!(f("00"), Err(E::OutOfRange));
        assert_eq!(f("32"), Err(E::OutOfRange));
    }

    #[test]
    fn u8_conversion_test() {
        type E = TryFromDayOfMonthError;
        let f = |d: u8| DayOfMonth::try_from(d);
        assert_eq!(f(0_u8), Err(E::OutOfRange));
        assert_eq!(f(1_u8).map(u8::from), Ok(1_u8));
        assert_eq!(f(31_u8).map(u8::from), Ok(31_u8));
        assert_eq!(f(32_u8), Err(E::OutOfRange));
    }

    #[test]
    fn pred_test() -> anyhow::Result<()> {
        assert_eq!(
            DayOfMonth::try_from(31)?.pred(),
            Some(DayOfMonth::try_from(30)?)
        );
        assert_eq!(
            DayOfMonth::try_from(2)?.pred(),
            Some(DayOfMonth::try_from(1)?)
        );
        assert_eq!(DayOfMonth::try_from(1)?.pred(), None);
        Ok(())
    }

    #[test]
    fn succ_test() -> anyhow::Result<()> {
        assert_eq!(
            DayOfMonth::try_from(1)?.succ(),
            Some(DayOfMonth::try_from(2)?)
        );
        assert_eq!(
            DayOfMonth::try_from(30)?.succ(),
            Some(DayOfMonth::try_from(31)?)
        );
        assert_eq!(DayOfMonth::try_from(31)?.succ(), None);
        Ok(())
    }

    #[test]
    fn add_days_test() -> anyhow::Result<()> {
        let d1 = DayOfMonth::try_from(1)?;
        let d2 = DayOfMonth::try_from(2)?;
        let d31 = DayOfMonth::try_from(31)?;
        assert_eq!(d1 + Days::from(0), d1);
        assert_eq!(d1 + Days::from(1), d2);
        assert_eq!(d1 + Days::from(30), d31);
        // should_panic
        // assert_eq!(d1 + Days::from(31), d31);
        assert_eq!(Days::from(0) + d1, d1);
        assert_eq!(Days::from(1) + d1, d2);
        assert_eq!(Days::from(30) + d1, d31);
        Ok(())
    }

    #[test]
    fn days_test() -> anyhow::Result<()> {
        assert_eq!(DayOfMonth::try_from(1)?.days(), Days::from(1));
        Ok(())
    }
}
