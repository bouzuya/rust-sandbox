use std::borrow::Cow;

use crate::writer::{sitemap_index_writer::Error, Lastmod, Loc};

type Result<T, E = Error> = std::result::Result<T, E>;

pub struct Sitemap<'a> {
    pub(in crate::writer) loc: Cow<'a, str>,
    pub(in crate::writer) lastmod: Option<Cow<'a, str>>,
}

impl<'a> TryFrom<&'a str> for Sitemap<'a> {
    type Error = Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::loc(value)
    }
}

impl<'a> Sitemap<'a> {
    pub fn loc<S>(loc: S) -> Result<Self>
    where
        S: TryInto<Loc<'a>>,
    {
        let loc = loc.try_into().map_err(|_| Error::InvalidLoc)?.into_inner();
        Ok(Self { loc, lastmod: None })
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
}
