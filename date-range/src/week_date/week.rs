use std::convert::TryFrom;
use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Week(u8);

#[derive(Debug, Eq, Error, PartialEq)]
pub enum ParseWeekError {
    #[error("invalid digit")]
    InvalidDigit,
    #[error("invalid length")]
    InvalidLength,
    #[error("out of range")]
    OutOfRange,
}

#[derive(Debug, Eq, Error, PartialEq)]
pub enum TryFromWeekError {
    #[error("out of range")]
    OutOfRange,
}

impl std::fmt::Display for Week {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}", self.0)
    }
}

impl std::str::FromStr for Week {
    type Err = ParseWeekError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 1970-01-01 (1970-W01-4) / 9999-12-31 (9999-W52-5)
        if s.len() != 2 {
            return Err(Self::Err::InvalidLength);
        }
        let mut w = 0_u8;
        for c in s.chars() {
            let d = match c {
                '0'..='9' => c as u8 - '0' as u8,
                _ => return Err(Self::Err::InvalidDigit),
            };
            w = w * 10 + d;
        }
        Self::try_from(w).map_err(|_| Self::Err::OutOfRange)
    }
}

impl From<Week> for u8 {
    fn from(week: Week) -> Self {
        week.0
    }
}

impl std::convert::TryFrom<u8> for Week {
    type Error = TryFromWeekError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(1..=53).contains(&value) {
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
        // str -(from_str / parse)-> Week
        // str <-(to_string & as_str)- Week
        type E = ParseWeekError;
        let f = |s: &str| s.parse::<Week>();
        assert_eq!(f("01").map(|y| y.to_string()), Ok("01".to_string()));
        assert_eq!(f("53").map(|y| y.to_string()), Ok("53".to_string()));
        assert_eq!(f(""), Err(E::InvalidLength));
        assert_eq!(f("0"), Err(E::InvalidLength));
        assert_eq!(f("100"), Err(E::InvalidLength));
        assert_eq!(f("0a"), Err(E::InvalidDigit));
        assert_eq!(f("+1"), Err(E::InvalidDigit));
        assert_eq!(f("54"), Err(E::OutOfRange));
    }

    #[test]
    fn u8_convert() {
        // u8 -(try_from)-> Week
        // u8 <-(from)- Week
        type E = TryFromWeekError;
        let f = |w: u8| Week::try_from(w);
        assert_eq!(f(0_u8), Err(E::OutOfRange));
        assert_eq!(f(1_u8).map(|m| u8::from(m)), Ok(1_u8));
        assert_eq!(f(53_u8).map(|m| u8::from(m)), Ok(53_u8));
        assert_eq!(f(54_u8), Err(E::OutOfRange));
    }
}
