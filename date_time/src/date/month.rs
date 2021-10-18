use std::convert::TryFrom;
use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Month(u8);

impl Month {
    pub fn january() -> Self {
        Self(1)
    }

    pub fn february() -> Self {
        Self(2)
    }

    pub fn march() -> Self {
        Self(3)
    }

    pub fn april() -> Self {
        Self(4)
    }

    pub fn may() -> Self {
        Self(5)
    }

    pub fn june() -> Self {
        Self(6)
    }

    pub fn july() -> Self {
        Self(7)
    }

    pub fn august() -> Self {
        Self(8)
    }

    pub fn september() -> Self {
        Self(9)
    }

    pub fn october() -> Self {
        Self(10)
    }

    pub fn november() -> Self {
        Self(11)
    }

    pub fn december() -> Self {
        Self(12)
    }

    pub fn pred(&self) -> Option<Self> {
        if self.0 > 1 {
            Some(Self(self.0 - 1))
        } else {
            None
        }
    }

    pub fn succ(&self) -> Option<Self> {
        if self.0 < 12 {
            Some(Self(self.0 + 1))
        } else {
            None
        }
    }
}

#[derive(Debug, Eq, Error, PartialEq)]
pub enum ParseMonthError {
    #[error("invalid digit")]
    InvalidDigit,
    #[error("invalid length")]
    InvalidLength,
    #[error("out of range")]
    OutOfRange,
}

#[derive(Debug, Eq, Error, PartialEq)]
pub enum TryFromMonthError {
    #[error("out of range")]
    OutOfRange,
}

impl std::fmt::Display for Month {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}", self.0)
    }
}

impl std::str::FromStr for Month {
    type Err = ParseMonthError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            return Err(Self::Err::InvalidLength);
        }
        let mut m = 0_u8;
        for c in s.chars() {
            let d = match c {
                '0'..='9' => c as u8 - b'0',
                _ => return Err(Self::Err::InvalidDigit),
            };
            m = m * 10 + d;
        }
        Self::try_from(m).map_err(|_| Self::Err::OutOfRange)
    }
}

impl From<Month> for u8 {
    fn from(month: Month) -> Self {
        month.0
    }
}

impl std::convert::TryFrom<u8> for Month {
    type Error = TryFromMonthError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(1..=12).contains(&value) {
            return Err(Self::Error::OutOfRange);
        }
        Ok(Self(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_conversion_test() {
        type E = ParseMonthError;
        let f = |s: &str| s.parse::<Month>();
        assert_eq!(f("01").map(|m| m.to_string()), Ok("01".to_string()));
        assert_eq!(f("12").map(|m| m.to_string()), Ok("12".to_string()));
        assert_eq!(f(""), Err(E::InvalidLength));
        assert_eq!(f("1"), Err(E::InvalidLength));
        assert_eq!(f("100"), Err(E::InvalidLength));
        assert_eq!(f("0a"), Err(E::InvalidDigit));
        assert_eq!(f("+1"), Err(E::InvalidDigit));
        assert_eq!(f("00"), Err(E::OutOfRange));
        assert_eq!(f("13"), Err(E::OutOfRange));
    }

    #[test]
    fn u8_conversion_test() {
        type E = TryFromMonthError;
        let f = |d: u8| Month::try_from(d);
        assert_eq!(f(0_u8), Err(E::OutOfRange));
        assert_eq!(f(1_u8).map(u8::from), Ok(1_u8));
        assert_eq!(f(12_u8).map(u8::from), Ok(12_u8));
        assert_eq!(f(13_u8), Err(E::OutOfRange));
    }

    #[test]
    fn pred_test() -> anyhow::Result<()> {
        assert_eq!(Month::try_from(12)?.pred(), Some(Month::try_from(11)?));
        assert_eq!(Month::try_from(11)?.pred(), Some(Month::try_from(10)?));
        assert_eq!(Month::try_from(10)?.pred(), Some(Month::try_from(9)?));
        assert_eq!(Month::try_from(9)?.pred(), Some(Month::try_from(8)?));
        assert_eq!(Month::try_from(8)?.pred(), Some(Month::try_from(7)?));
        assert_eq!(Month::try_from(7)?.pred(), Some(Month::try_from(6)?));
        assert_eq!(Month::try_from(6)?.pred(), Some(Month::try_from(5)?));
        assert_eq!(Month::try_from(5)?.pred(), Some(Month::try_from(4)?));
        assert_eq!(Month::try_from(4)?.pred(), Some(Month::try_from(3)?));
        assert_eq!(Month::try_from(3)?.pred(), Some(Month::try_from(2)?));
        assert_eq!(Month::try_from(2)?.pred(), Some(Month::try_from(1)?));
        assert_eq!(Month::try_from(1)?.pred(), None);
        Ok(())
    }

    #[test]
    fn succ_test() -> anyhow::Result<()> {
        assert_eq!(Month::try_from(1)?.succ(), Some(Month::try_from(2)?));
        assert_eq!(Month::try_from(2)?.succ(), Some(Month::try_from(3)?));
        assert_eq!(Month::try_from(3)?.succ(), Some(Month::try_from(4)?));
        assert_eq!(Month::try_from(4)?.succ(), Some(Month::try_from(5)?));
        assert_eq!(Month::try_from(5)?.succ(), Some(Month::try_from(6)?));
        assert_eq!(Month::try_from(6)?.succ(), Some(Month::try_from(7)?));
        assert_eq!(Month::try_from(7)?.succ(), Some(Month::try_from(8)?));
        assert_eq!(Month::try_from(8)?.succ(), Some(Month::try_from(9)?));
        assert_eq!(Month::try_from(9)?.succ(), Some(Month::try_from(10)?));
        assert_eq!(Month::try_from(10)?.succ(), Some(Month::try_from(11)?));
        assert_eq!(Month::try_from(11)?.succ(), Some(Month::try_from(12)?));
        assert_eq!(Month::try_from(12)?.succ(), None);
        Ok(())
    }

    #[test]
    fn name_test() -> anyhow::Result<()> {
        assert_eq!(Month::try_from(1)?, Month::january());
        assert_eq!(Month::try_from(2)?, Month::february());
        assert_eq!(Month::try_from(3)?, Month::march());
        assert_eq!(Month::try_from(4)?, Month::april());
        assert_eq!(Month::try_from(5)?, Month::may());
        assert_eq!(Month::try_from(6)?, Month::june());
        assert_eq!(Month::try_from(7)?, Month::july());
        assert_eq!(Month::try_from(8)?, Month::august());
        assert_eq!(Month::try_from(9)?, Month::september());
        assert_eq!(Month::try_from(10)?, Month::october());
        assert_eq!(Month::try_from(11)?, Month::november());
        assert_eq!(Month::try_from(12)?, Month::december());
        Ok(())
    }
}
