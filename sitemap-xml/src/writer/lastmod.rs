use std::borrow::Cow;

use time::format_description::well_known::Iso8601;

// TODO: Error
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
#[error("error")]
pub struct Error;

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
        if value.len() == 10 {
            let format = time::macros::format_description!("[year]-[month]-[day]");
            time::Date::parse(value, &format).map_err(|_| Error)?;
        } else {
            time::OffsetDateTime::parse(value, &Iso8601::DEFAULT).map_err(|_| Error)?;
        }
        Ok(Self(Cow::Borrowed(value)))
    }
}

impl<'a> TryFrom<time::Date> for Lastmod<'a> {
    type Error = Error;

    fn try_from(value: time::Date) -> Result<Self, Self::Error> {
        let format = time::macros::format_description!("[year]-[month]-[day]");
        let s = value.format(&format).map_err(|_| Error)?;
        Ok(Self(Cow::Owned(s)))
    }
}

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

    #[test]
    fn test_date() -> anyhow::Result<()> {
        #[rustfmt::skip]
        let lastmod = Lastmod::try_from(time::macros::date!(2005-01-01))?;
        assert_eq!(lastmod.into_inner(), "2005-01-01");
        Ok(())
    }

    #[test]
    fn test_date_time() -> anyhow::Result<()> {
        #[rustfmt::skip]
        let lastmod = Lastmod::try_from(time::macros::datetime!(2004-12-23 18:00:15 +00:00))?;
        assert_eq!(lastmod.into_inner(), "2004-12-23T18:00:15.000000000Z");
        Ok(())
    }
}
