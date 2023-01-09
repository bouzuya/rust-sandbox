#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("invalid loc")]
    InvalidLoc,
    #[error("too many urls")]
    TooManyUrls,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Sitemaps {
    urls: Vec<Url>,
}

impl Sitemaps {
    pub fn new(urls: Vec<Url>) -> Result<Self, Error> {
        if urls.len() > 50_000 {
            return Err(Error::TooManyUrls);
        }
        // TODO: check <= 52,428,800 bytes
        Ok(Self { urls })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Url {
    pub loc: Loc,
    // pub lastmod: Option<Lastmod>,
    // pub changefreq: Option<Changefreq>,
    // pub priority: Option<Priority>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Loc(url::Url);

impl std::fmt::Display for Loc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::str::FromStr for Loc {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let u = url::Url::parse(s).map_err(|_| Error::InvalidLoc)?;
        if u.as_str() != s {
            return Err(Error::InvalidLoc);
        }
        if u.as_str().chars().count() >= 2048 {
            return Err(Error::InvalidLoc);
        }
        Ok(Self(u))
    }
}

impl TryFrom<&str> for Loc {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        <Self as std::str::FromStr>::from_str(value)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_loc() -> anyhow::Result<()> {
        let s = "https://example.com/";
        let loc = Loc::from_str(s)?;
        assert_eq!(loc, Loc::try_from(s)?);
        assert_eq!(loc.to_string(), s);

        let s = "https://example.com";
        assert!(Loc::from_str(s).is_err());

        let s = format!("https://example.com/{}", "a".repeat(2028));
        assert_eq!(s.len(), 2048);
        assert!(Loc::from_str(s.as_str()).is_err());

        let s = format!("https://example.com/{}", "a".repeat(2027));
        assert_eq!(s.len(), 2047);
        assert_eq!(Loc::from_str(s.as_str())?.to_string(), s);

        let s = "https://example.com/path";
        let loc = Loc::from_str(s)?;
        assert_eq!(loc.to_string(), s);
        Ok(())
    }
}
