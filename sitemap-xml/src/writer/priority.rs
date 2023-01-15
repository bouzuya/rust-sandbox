use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::Cow;

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("format")]
    Format,
    #[error("range")]
    Range,
}

/// A `priority` child entry.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Priority<'a>(Cow<'a, str>);

impl<'a> Priority<'a> {
    pub(crate) fn into_inner(self) -> Cow<'a, str> {
        self.0
    }

    fn is_valid_format(s: &str) -> Result<(), Error> {
        // <https://www.w3.org/TR/xmlschema11-2/#decimal>
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r#"\A(\+|-)?([0-9]+(\.[0-9]*)?|\.[0-9]+)\z"#).unwrap();
        }
        if RE.is_match(s) {
            Ok(())
        } else {
            Err(Error::Format)
        }
    }

    fn is_valid_range(f: f64) -> Result<(), Error> {
        if (0.0..=1.0).contains(&f) {
            Ok(())
        } else {
            Err(Error::Range)
        }
    }
}

impl<'a> TryFrom<&'a str> for Priority<'a> {
    type Error = Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::is_valid_format(value)?;
        let f = value.parse::<f64>().map_err(|_| Error::Format)?;
        Self::is_valid_range(f)?;
        Ok(Self(Cow::Borrowed(value)))
    }
}

impl<'a> TryFrom<f64> for Priority<'a> {
    type Error = Error;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::is_valid_range(value)?;
        let s = value.to_string();
        Self::is_valid_format(s.as_str())?;
        Ok(Self(Cow::Owned(s)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_format() {
        assert!(Priority::is_valid_format("-inf").is_err());
        assert!(Priority::is_valid_format("-0.1").is_ok());
        assert!(Priority::is_valid_format("-0.0").is_ok());
        assert!(Priority::is_valid_format("0.0").is_ok());
        assert!(Priority::is_valid_format("1.0").is_ok());
        assert!(Priority::is_valid_format("1.1").is_ok());
        assert!(Priority::is_valid_format("+0.0").is_ok());
        assert!(Priority::is_valid_format(".0").is_ok());
        assert!(Priority::is_valid_format("+.0").is_ok());
        assert!(Priority::is_valid_format("2.5E10").is_err());
        assert!(Priority::is_valid_format("2E10").is_err());
        assert!(Priority::is_valid_format("inf").is_err());
        assert!(Priority::is_valid_format("+infinity").is_err());
        assert!(Priority::is_valid_format("NaN").is_err());
    }

    #[test]
    fn test_is_valid_range() {
        assert!(Priority::is_valid_range(f64::NEG_INFINITY).is_err());
        assert!(Priority::is_valid_range(-0.1).is_err());
        assert!(Priority::is_valid_range(-0.0).is_ok());
        assert!(Priority::is_valid_range(0.0).is_ok());
        assert!(Priority::is_valid_range(1.0).is_ok());
        assert!(Priority::is_valid_range(1.1).is_err());
        assert!(Priority::is_valid_range(f64::INFINITY).is_err());
        assert!(Priority::is_valid_range(f64::NAN).is_err());
    }

    #[test]
    fn test_try_from_f64() -> anyhow::Result<()> {
        assert!(Priority::try_from(f64::NEG_INFINITY).is_err());
        assert!(Priority::try_from(-0.1_f64).is_err());
        assert_eq!(Priority::try_from(-0.0_f64)?.into_inner(), "-0");
        assert_eq!(Priority::try_from(0.0_f64)?.into_inner(), "0");
        assert_eq!(Priority::try_from(0.5_f64)?.into_inner(), "0.5");
        assert_eq!(Priority::try_from(1.0_f64)?.into_inner(), "1");
        assert!(Priority::try_from(1.1_f64).is_err());
        assert!(Priority::try_from(f64::INFINITY).is_err());
        assert!(Priority::try_from(f64::NAN).is_err());
        Ok(())
    }

    #[test]
    fn test_try_from_str() -> anyhow::Result<()> {
        assert!(Priority::try_from("a").is_err());
        assert_eq!(Priority::try_from("-0.0")?.into_inner(), "-0.0");
        assert_eq!(Priority::try_from("0.0")?.into_inner(), "0.0");
        assert_eq!(Priority::try_from("+0.0")?.into_inner(), "+0.0");
        assert_eq!(Priority::try_from("0.5")?.into_inner(), "0.5");
        assert_eq!(Priority::try_from("1.0")?.into_inner(), "1.0");
        assert!(Priority::try_from("1.1").is_err());
        Ok(())
    }
}
