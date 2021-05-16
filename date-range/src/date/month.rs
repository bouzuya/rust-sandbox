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
        if !(1..=12).contains(&m) {
            return Err(Self::Err::OutOfRange);
        }
        Ok(Self(m))
    }
}

impl From<Month> for u8 {
    fn from(month: Month) -> Self {
        month.0
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn from_str() {
        type PME = ParseMonthError;
        assert_eq!(Month::from_str("01"), Ok(Month(1)));
        assert_eq!(Month::from_str("12"), Ok(Month(12)));
        assert_eq!(Month::from_str(""), Err(PME::InvalidLength));
        assert_eq!(Month::from_str("1"), Err(PME::InvalidLength));
        assert_eq!(Month::from_str("100"), Err(PME::InvalidLength));
        assert_eq!(Month::from_str("0a"), Err(PME::InvalidDigit));
        assert_eq!(Month::from_str("+1"), Err(PME::InvalidDigit));
        assert_eq!(Month::from_str("00"), Err(PME::OutOfRange));
        assert_eq!(Month::from_str("13"), Err(PME::OutOfRange));
    }

    #[test]
    fn to_string() {
        for s in vec!["01", "12"] {
            assert_eq!(Month::from_str(s).unwrap().to_string(), s.to_string());
        }
    }

    #[test]
    fn u8_from_month() {
        assert_eq!(u8::from(Month::from_str("01").unwrap()), 1_u8);
    }
}
