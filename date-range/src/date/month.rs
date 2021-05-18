use std::convert::TryFrom;
use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Month(u8);

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
    fn str_convert() {
        // str -(from_str / parse)-> Month
        // str <-(to_string & as_str)- Month
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
    fn u8_convert() {
        // u8 -(try_from)-> Month
        // u8 <-(from)- Month
        type E = TryFromMonthError;
        let f = |d: u8| Month::try_from(d);
        assert_eq!(f(0_u8), Err(E::OutOfRange));
        assert_eq!(f(1_u8).map(|m| u8::from(m)), Ok(1_u8));
        assert_eq!(f(12_u8).map(|m| u8::from(m)), Ok(12_u8));
        assert_eq!(f(13_u8), Err(E::OutOfRange));
    }
}
