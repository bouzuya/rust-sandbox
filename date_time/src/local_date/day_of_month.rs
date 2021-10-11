use std::convert::TryFrom;
use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DayOfMonth(u8);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_convert() {
        // str -(from_str / parse)-> DayOfMonth
        // str <-(to_string & as_str)- DayOfMonth
        type PDE = ParseDayOfMonthError;
        let f = |s: &str| s.parse::<DayOfMonth>();
        assert_eq!(f("01").map(|d| d.to_string()), Ok("01".to_string()));
        assert_eq!(f("31").map(|d| d.to_string()), Ok("31".to_string()));
        assert_eq!(f(""), Err(PDE::InvalidLength));
        assert_eq!(f("1"), Err(PDE::InvalidLength));
        assert_eq!(f("100"), Err(PDE::InvalidLength));
        assert_eq!(f("0a"), Err(PDE::InvalidDigit));
        assert_eq!(f("+1"), Err(PDE::InvalidDigit));
        assert_eq!(f("00"), Err(PDE::OutOfRange));
        assert_eq!(f("32"), Err(PDE::OutOfRange));
    }

    #[test]
    fn u8_convert() {
        // u8 -(try_from)-> DayOfMonth
        // u8 <-(from)- DayOfMonth
        type E = TryFromDayOfMonthError;
        let f = |d: u8| DayOfMonth::try_from(d);
        assert_eq!(f(0_u8), Err(E::OutOfRange));
        assert_eq!(f(1_u8).map(|d| u8::from(d)), Ok(1_u8));
        assert_eq!(f(31_u8).map(|d| u8::from(d)), Ok(31_u8));
        assert_eq!(f(32_u8), Err(E::OutOfRange));
    }
}
