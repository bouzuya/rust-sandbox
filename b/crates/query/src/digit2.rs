use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("parse digit2 error")]
pub struct ParseDigit2Error;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Digit2(u8, u8);

impl std::str::FromStr for Digit2 {
    type Err = ParseDigit2Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            return Err(ParseDigit2Error);
        }
        let ds = s
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as u8))
            .collect::<Vec<u8>>();
        if ds.len() != 2 {
            return Err(ParseDigit2Error);
        }
        Ok(Self(ds[0], ds[1]))
    }
}

impl std::fmt::Display for Digit2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

impl std::convert::TryFrom<u8> for Digit2 {
    type Error = ParseDigit2Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 99 {
            return Err(ParseDigit2Error);
        }
        Ok(Self(value % 100 / 10, value % 10))
    }
}

impl std::convert::From<Digit2> for u8 {
    fn from(d: Digit2) -> Self {
        d.0 * 10 + d.1
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::*;

    #[test]
    fn string_conversion_test() -> anyhow::Result<()> {
        assert_eq!("12".parse::<Digit2>()?.to_string(), "12".to_string());
        assert_eq!("01".parse::<Digit2>()?.to_string(), "01".to_string());
        Ok(())
    }

    #[test]
    fn u8_conversion_test() -> anyhow::Result<()> {
        assert_eq!(u8::from(Digit2::try_from(12)?), 12);
        assert_eq!(u8::from(Digit2::try_from(1)?), 1);
        Ok(())
    }
}
