use std::convert::TryFrom;
use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Year(u16);

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
        let f = |y: u16| Year::try_from(y).unwrap().is_leap_year();
        assert_eq!(f(2000), true);
        assert_eq!(f(2004), true);
        assert_eq!(f(2100), false);
    }

    #[test]
    fn str_convert() {
        // str -(from_str / parse)-> Year
        // str <-(to_string & as_str)- Year
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
    fn u16_convert() {
        // u16 -(try_from)-> Year
        // u16 <-(from)- Year
        type E = TryFromYearError;
        let f = |y: u16| Year::try_from(y);
        assert_eq!(f(1969_u16), Err(E::OutOfRange));
        assert_eq!(f(1970_u16).map(|m| u16::from(m)), Ok(1970_u16));
        assert_eq!(f(9999_u16).map(|m| u16::from(m)), Ok(9999_u16));
        assert_eq!(f(10000_u16), Err(E::OutOfRange));
    }
}
