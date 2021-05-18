use std::convert::TryFrom;
use thiserror::Error;

use super::Week;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WeekYear(u16);

#[derive(Debug, Eq, Error, PartialEq)]
pub enum ParseWeekYearError {
    #[error("invalid digit")]
    InvalidDigit,
    #[error("invalid length")]
    InvalidLength,
    #[error("out of range")]
    OutOfRange,
}

#[derive(Debug, Eq, Error, PartialEq)]
pub enum TryFromWeekYearError {
    #[error("out of range")]
    OutOfRange,
}

impl WeekYear {
    pub fn last_week(&self) -> Week {
        let f = |y: u16| (y + y / 4 - y / 100 + y / 400) % 7;
        let p = f(self.0) == 4 || f(self.0 - 1) == 3;
        Week::try_from(52 + if p { 1 } else { 0 }).unwrap()
    }
}

impl std::fmt::Display for WeekYear {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04}", self.0)
    }
}

impl std::str::FromStr for WeekYear {
    type Err = ParseWeekYearError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 1970-01-01 (1970-W01-4) / 9999-12-31 (9999-W52-5)
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

impl From<WeekYear> for u16 {
    fn from(week_year: WeekYear) -> Self {
        week_year.0
    }
}

impl std::convert::TryFrom<u16> for WeekYear {
    type Error = TryFromWeekYearError;

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
    fn last_week() {
        let f = |y: u16| WeekYear::try_from(y).unwrap().last_week();
        let w = |w: u8| Week::try_from(w).unwrap();
        assert_eq!(f(2020), w(53));
        assert_eq!(f(2021), w(52));
    }

    #[test]
    fn str_convert() {
        // str -(from_str / parse)-> WeekYear
        // str <-(to_string & as_str)- WeekYear
        type E = ParseWeekYearError;
        let f = |s: &str| s.parse::<WeekYear>();
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
        // u16 -(try_from)-> WeekYear
        // u16 <-(from)- WeekYear
        type E = TryFromWeekYearError;
        let f = |y: u16| WeekYear::try_from(y);
        assert_eq!(f(1969_u16), Err(E::OutOfRange));
        assert_eq!(f(1970_u16).map(|m| u16::from(m)), Ok(1970_u16));
        assert_eq!(f(9999_u16).map(|m| u16::from(m)), Ok(9999_u16));
        assert_eq!(f(10000_u16), Err(E::OutOfRange));
    }
}
