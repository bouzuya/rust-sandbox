use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Year(u16);

#[derive(Debug, Eq, Error, PartialEq)]
pub enum ParseYearError {
    #[error("invalid digit")]
    InvalidDigit,
    #[error("invalid length")]
    InvalidLength,
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
        Ok(Self(y))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn from_str() {
        type PYE = ParseYearError;
        assert_eq!(Year::from_str("0000"), Ok(Year(0)));
        assert_eq!(Year::from_str("9999"), Ok(Year(9999)));
        assert_eq!(Year::from_str(""), Err(PYE::InvalidLength));
        assert_eq!(Year::from_str("0"), Err(PYE::InvalidLength));
        assert_eq!(Year::from_str("10000"), Err(PYE::InvalidLength));
        assert_eq!(Year::from_str("000a"), Err(PYE::InvalidDigit));
        assert_eq!(Year::from_str("+000"), Err(PYE::InvalidDigit));
    }

    #[test]
    fn to_string() {
        for s in vec!["0000", "9999"] {
            assert_eq!(Year::from_str(s).unwrap().to_string(), s.to_string());
        }
    }

    #[test]
    fn is_leap_year() {
        assert_eq!(Year::from_str("2000").unwrap().is_leap_year(), true);
        assert_eq!(Year::from_str("2004").unwrap().is_leap_year(), true);
        assert_eq!(Year::from_str("1900").unwrap().is_leap_year(), false);
    }
}
