use std::borrow::Cow;

use crate::writer::{
    changefreq::Changefreq, lastmod::Lastmod, loc::Loc, priority::Priority, sitemap_writer::Error,
};

type Result<T, E = Error> = std::result::Result<T, E>;

pub struct Url<'a> {
    pub(in crate::writer) loc: Cow<'a, str>,
    pub(in crate::writer) lastmod: Option<Cow<'a, str>>,
    pub(in crate::writer) changefreq: Option<Changefreq>,
    pub(in crate::writer) priority: Option<Cow<'a, str>>,
}

impl<'a> TryFrom<&'a str> for Url<'a> {
    type Error = Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::loc(value)
    }
}

impl<'a> Url<'a> {
    pub fn loc<S>(loc: S) -> Result<Self>
    where
        S: TryInto<Loc<'a>>,
    {
        let loc = loc.try_into().map_err(|_| Error::InvalidLoc)?.into_inner();
        Ok(Self {
            loc,
            lastmod: None,
            changefreq: None,
            priority: None,
        })
    }

    pub fn changefreq<S>(mut self, s: S) -> Result<Self>
    where
        S: TryInto<Changefreq>,
    {
        let changefreq = s.try_into().map_err(|_| Error::InvalidChangefreq)?;
        self.changefreq = Some(changefreq);
        Ok(self)
    }

    pub fn lastmod<S>(mut self, s: S) -> Result<Self>
    where
        S: TryInto<Lastmod<'a>>,
    {
        let lastmod = s
            .try_into()
            .map_err(|_| Error::InvalidLastmod)?
            .into_inner();
        self.lastmod = Some(lastmod);
        Ok(self)
    }

    pub fn priority<S>(mut self, s: S) -> Result<Self>
    where
        S: TryInto<Priority<'a>>,
    {
        let priority = s
            .try_into()
            .map_err(|_| Error::InvalidPriority)?
            .into_inner();
        self.priority = Some(priority);
        Ok(self)
    }
}
