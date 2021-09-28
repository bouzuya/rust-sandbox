use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("parse digit4 error")]
pub struct ParseDigit4Error;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Digit4(u8, u8, u8, u8);

impl std::str::FromStr for Digit4 {
    type Err = ParseDigit4Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err(ParseDigit4Error);
        }
        let ds = s
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as u8))
            .collect::<Vec<u8>>();
        if ds.len() != 4 {
            return Err(ParseDigit4Error);
        }
        Ok(Self(ds[0], ds[1], ds[2], ds[3]))
    }
}

impl std::fmt::Display for Digit4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}{}", self.0, self.1, self.2, self.3)
    }
}

impl std::convert::TryFrom<u16> for Digit4 {
    type Error = ParseDigit4Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value > 9999 {
            return Err(ParseDigit4Error);
        }
        Ok(Self(
            (value % 10000 / 1000) as u8,
            (value % 1000 / 100) as u8,
            (value % 100 / 10) as u8,
            (value % 10) as u8,
        ))
    }
}

impl std::convert::From<Digit4> for u16 {
    fn from(d: Digit4) -> Self {
        let d0 = d.0 as u16 * 1000;
        let d1 = d.1 as u16 * 100;
        let d2 = d.2 as u16 * 10;
        let d3 = d.3 as u16;
        d0 + d1 + d2 + d3
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::*;

    #[test]
    fn string_conversion_test() -> anyhow::Result<()> {
        assert_eq!("1234".parse::<Digit4>()?.to_string(), "1234".to_string());
        assert_eq!("0123".parse::<Digit4>()?.to_string(), "0123".to_string());
        Ok(())
    }

    #[test]
    fn u16_conversion_test() -> anyhow::Result<()> {
        assert_eq!(u16::from(Digit4::try_from(1234_u16)?), 1234_u16);
        assert_eq!(u16::from(Digit4::try_from(123_u16)?), 123_u16);
        Ok(())
    }
}
