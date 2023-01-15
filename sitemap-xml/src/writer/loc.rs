use std::borrow::Cow;

// TODO: Error
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
#[error("error")]
pub struct Error;

/// A `loc` child entry.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Loc<'a>(Cow<'a, str>);

impl<'a> Loc<'a> {
    pub(crate) fn into_inner(self) -> Cow<'a, str> {
        self.0
    }
}

impl<'a> TryFrom<&'a str> for Loc<'a> {
    type Error = Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if value.chars().count() >= 2048 {
            return Err(Error);
        }
        // too slow
        // let u = url::Url::parse(value).map_err(|_| Error)?;
        // if u.as_str() != value {
        //     return Err(Error);
        // }
        Ok(Self(Cow::Borrowed(value)))
    }
}

impl<'a> TryFrom<url::Url> for Loc<'a> {
    type Error = Error;

    fn try_from(value: url::Url) -> Result<Self, Self::Error> {
        let s = value.to_string();
        if s.chars().count() >= 2048 {
            return Err(Error);
        }
        Ok(Self(Cow::Owned(s)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let s = "https://example.com/";
        assert_eq!(Loc::try_from(s)?.into_inner(), s);

        let s = "https://example.com";
        assert_eq!(Loc::try_from(s)?.into_inner(), s);

        let s = format!("https://example.com/{}", "a".repeat(2028));
        assert_eq!(s.len(), 2048);
        assert!(Loc::try_from(s.as_str()).is_err());

        let s = format!("https://example.com/{}", "a".repeat(2027));
        assert_eq!(s.len(), 2047);
        assert_eq!(Loc::try_from(s.as_str())?.into_inner(), s);

        let s = "https://example.com/path";
        assert_eq!(Loc::try_from(s)?.into_inner(), s);
        Ok(())
    }
}
