use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::Cow;

#[cfg(feature = "time")]
use time::format_description::well_known::Iso8601;

// TODO: Error
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
#[error("error")]
pub struct Error;

/// A `lastmod` child entry.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Lastmod<'a>(Cow<'a, str>);

impl<'a> Lastmod<'a> {
    pub(crate) fn into_inner(self) -> Cow<'a, str> {
        self.0
    }
}

impl<'a> TryFrom<&'a str> for Lastmod<'a> {
    type Error = Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        // <https://www.w3.org/TR/xmlschema11-2/#date>
        // <https://www.w3.org/TR/xmlschema11-2/#dateTime>
        lazy_static! {
            static ref DATE_RE: Regex = Regex::new(
                r#"\A-?([1-9][0-9]{3,}|0[0-9]{3})-(0[1-9]|1[0-2])-(0[1-9]|[12][0-9]|3[01])(Z|(\+|-)((0[0-9]|1[0-3]):[0-5][0-9]|14:00))?\z"#
            ).unwrap();

            static ref DATE_TIME_RE: Regex = Regex::new(
                r#"\A-?([1-9][0-9]{3,}|0[0-9]{3})-(0[1-9]|1[0-2])-(0[1-9]|[12][0-9]|3[01])T(([01][0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9](\.[0-9]+)?|(24:00:00(\.0+)?))(Z|(\+|-)((0[0-9]|1[0-3]):[0-5][0-9]|14:00))?\z"#
            ).unwrap();
        }
        if !DATE_RE.is_match(value) && !DATE_TIME_RE.is_match(value) {
            return Err(Error);
        }
        Ok(Self(Cow::Borrowed(value)))
    }
}

#[cfg(feature = "time")]
impl<'a> TryFrom<time::Date> for Lastmod<'a> {
    type Error = Error;

    fn try_from(value: time::Date) -> Result<Self, Self::Error> {
        let format = time::macros::format_description!("[year]-[month]-[day]");
        let s = value.format(&format).map_err(|_| Error)?;
        Ok(Self(Cow::Owned(s)))
    }
}

#[cfg(feature = "time")]
impl<'a> TryFrom<time::OffsetDateTime> for Lastmod<'a> {
    type Error = Error;

    fn try_from(value: time::OffsetDateTime) -> Result<Self, Self::Error> {
        let s = value.format(&Iso8601::DEFAULT).map_err(|_| Error)?;
        Ok(Self(Cow::Owned(s)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let lastmod = Lastmod::try_from("2005-01-01")?;
        assert_eq!(lastmod.into_inner(), "2005-01-01");

        let lastmod = Lastmod::try_from("2004-12-23T18:00:15+00:00")?;
        assert_eq!(lastmod.into_inner(), "2004-12-23T18:00:15+00:00");
        Ok(())
    }

    #[cfg(feature = "time")]
    #[test]
    fn test_date() -> anyhow::Result<()> {
        #[rustfmt::skip]
        let lastmod = Lastmod::try_from(time::macros::date!(2005-01-01))?;
        assert_eq!(lastmod.into_inner(), "2005-01-01");
        Ok(())
    }

    #[cfg(feature = "time")]
    #[test]
    fn test_date_time() -> anyhow::Result<()> {
        #[rustfmt::skip]
        let lastmod = Lastmod::try_from(time::macros::datetime!(2004-12-23 18:00:15 +00:00))?;
        assert_eq!(lastmod.into_inner(), "2004-12-23T18:00:15.000000000Z");
        Ok(())
    }
}
