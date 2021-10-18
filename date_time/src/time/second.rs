use std::convert::TryFrom;
use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Second(u8);

#[derive(Debug, Eq, Error, PartialEq)]
pub enum ParseSecondError {
    #[error("invalid digit")]
    InvalidDigit,
    #[error("invalid length")]
    InvalidLength,
    #[error("out of range")]
    OutOfRange,
}

#[derive(Debug, Eq, Error, PartialEq)]
pub enum TryFromSecondError {
    #[error("out of range")]
    OutOfRange,
}

impl std::fmt::Display for Second {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}", self.0)
    }
}

impl std::str::FromStr for Second {
    type Err = ParseSecondError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            return Err(Self::Err::InvalidLength);
        }
        let mut h = 0_u8;
        for c in s.chars() {
            let d = match c {
                '0'..='9' => c as u8 - b'0',
                _ => return Err(Self::Err::InvalidDigit),
            };
            h = h * 10 + d;
        }
        Self::try_from(h).map_err(|_| Self::Err::OutOfRange)
    }
}

impl From<Second> for u8 {
    fn from(second: Second) -> Self {
        second.0
    }
}

impl std::convert::TryFrom<u8> for Second {
    type Error = TryFromSecondError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(0..=59).contains(&value) {
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
        type E = ParseSecondError;
        let f = |s: &str| s.parse::<Second>();
        assert_eq!(f("00").map(|d| d.to_string()), Ok("00".to_string()));
        assert_eq!(f("59").map(|d| d.to_string()), Ok("59".to_string()));
        assert_eq!(f(""), Err(E::InvalidLength));
        assert_eq!(f("1"), Err(E::InvalidLength));
        assert_eq!(f("100"), Err(E::InvalidLength));
        assert_eq!(f("0a"), Err(E::InvalidDigit));
        assert_eq!(f("+1"), Err(E::InvalidDigit));
        assert_eq!(f("60"), Err(E::OutOfRange));
    }

    #[test]
    fn u8_conversion_test() {
        type E = TryFromSecondError;
        let f = |d: u8| Second::try_from(d);
        assert_eq!(f(0_u8).map(u8::from), Ok(0_u8));
        assert_eq!(f(59_u8).map(u8::from), Ok(59_u8));
        assert_eq!(f(60_u8), Err(E::OutOfRange));
    }
}
