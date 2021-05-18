use std::convert::TryFrom;
use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum DayOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Debug, Eq, Error, PartialEq)]
pub enum ParseDayOfWeekError {
    #[error("invalid digit")]
    InvalidDigit,
    #[error("invalid length")]
    InvalidLength,
    #[error("out of range")]
    OutOfRange,
}

#[derive(Debug, Eq, Error, PartialEq)]
pub enum TryFromDayOfWeekError {
    #[error("out of range")]
    OutOfRange,
}

impl std::fmt::Display for DayOfWeek {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", u8::from(self.clone()))
    }
}

impl std::str::FromStr for DayOfWeek {
    type Err = ParseDayOfWeekError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(Self::Err::InvalidLength);
        }
        let c = s.chars().next().unwrap();
        let d = match c {
            '0'..='9' => c as u8 - '0' as u8,
            _ => return Err(Self::Err::InvalidDigit),
        };
        Self::try_from(d).map_err(|_| Self::Err::OutOfRange)
    }
}

impl From<DayOfWeek> for u8 {
    fn from(day_of_week: DayOfWeek) -> Self {
        match day_of_week {
            DayOfWeek::Monday => 1,
            DayOfWeek::Tuesday => 2,
            DayOfWeek::Wednesday => 3,
            DayOfWeek::Thursday => 4,
            DayOfWeek::Friday => 5,
            DayOfWeek::Saturday => 6,
            DayOfWeek::Sunday => 7,
        }
    }
}

impl std::convert::TryFrom<u8> for DayOfWeek {
    type Error = TryFromDayOfWeekError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(DayOfWeek::Monday),
            2 => Ok(DayOfWeek::Tuesday),
            3 => Ok(DayOfWeek::Wednesday),
            4 => Ok(DayOfWeek::Thursday),
            5 => Ok(DayOfWeek::Friday),
            6 => Ok(DayOfWeek::Saturday),
            7 => Ok(DayOfWeek::Sunday),
            _ => Err(Self::Error::OutOfRange),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_convert() {
        // str -(from_str / parse)-> DayOfWeek
        // str <-(to_string & as_str)- DayOfWeek
        type E = ParseDayOfWeekError;
        let f = |s: &str| s.parse::<DayOfWeek>();
        assert_eq!(f("1").map(|y| y.to_string()), Ok("1".to_string()));
        assert_eq!(f("7").map(|y| y.to_string()), Ok("7".to_string()));
        assert_eq!(f(""), Err(E::InvalidLength));
        assert_eq!(f("10"), Err(E::InvalidLength));
        assert_eq!(f("a"), Err(E::InvalidDigit));
        assert_eq!(f("0"), Err(E::OutOfRange));
        assert_eq!(f("8"), Err(E::OutOfRange));
    }

    #[test]
    fn u8_convert() {
        // u8 -(try_from)-> DayOfWeek
        // u8 <-(from)- DayOfWeek
        type E = TryFromDayOfWeekError;
        let f = |w: u8| DayOfWeek::try_from(w);
        assert_eq!(f(0_u8), Err(E::OutOfRange));
        assert_eq!(f(1_u8).map(|m| u8::from(m)), Ok(1_u8));
        assert_eq!(f(7_u8).map(|m| u8::from(m)), Ok(7_u8));
        assert_eq!(f(8_u8), Err(E::OutOfRange));
    }
}
