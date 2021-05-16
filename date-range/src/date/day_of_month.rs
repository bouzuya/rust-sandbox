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
    use std::str::FromStr;

    use super::*;

    #[test]
    fn from_str() {
        type DOM = DayOfMonth;
        type PDE = ParseDayOfMonthError;
        assert_eq!(DOM::from_str("01"), Ok(DayOfMonth(1)));
        assert_eq!(DOM::from_str("31"), Ok(DayOfMonth(31)));
        assert_eq!(DOM::from_str(""), Err(PDE::InvalidLength));
        assert_eq!(DOM::from_str("1"), Err(PDE::InvalidLength));
        assert_eq!(DOM::from_str("100"), Err(PDE::InvalidLength));
        assert_eq!(DOM::from_str("0a"), Err(PDE::InvalidDigit));
        assert_eq!(DOM::from_str("+1"), Err(PDE::InvalidDigit));
        assert_eq!(DOM::from_str("00"), Err(PDE::OutOfRange));
        assert_eq!(DOM::from_str("32"), Err(PDE::OutOfRange));
    }

    #[test]
    fn to_string() {
        for s in vec!["01", "31"] {
            assert_eq!(DayOfMonth::from_str(s).unwrap().to_string(), s.to_string());
        }
    }

    #[test]
    fn try_from() {
        type DOM = DayOfMonth;
        type TDE = TryFromDayOfMonthError;
        assert_eq!(DOM::try_from(0), Err(TDE::OutOfRange));
        assert_eq!(DOM::try_from(1), Ok(DayOfMonth(1)));
        assert_eq!(DOM::try_from(31), Ok(DayOfMonth(31)));
        assert_eq!(DOM::try_from(32), Err(TDE::OutOfRange));
    }
}
