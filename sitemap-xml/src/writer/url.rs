use std::borrow::Cow;

use crate::writer::{
    changefreq::Changefreq, lastmod::Lastmod, loc::Loc, priority::Priority, sitemap_writer::Error,
};

type Result<T, E = Error> = std::result::Result<T, E>;

/// A builder for `url` entry.
///
/// # Examples
///
/// ```rust
/// # use sitemap_xml::writer::{Changefreq, Url};
/// # fn main() -> anyhow::Result<()> {
/// Url::loc("http://www.example.com/")?
///     .lastmod("2005-01-01")?
///     .changefreq("monthly")?
///     .priority("0.8")?;
///
/// Url::loc(::url::Url::parse("http://www.example.com/")?)?
///     .lastmod(::time::macros::date!(2005-01-01))?
///     .changefreq(Changefreq::Monthly)?
///     .priority(0.8)?;
/// #     Ok(())
/// # }
/// ```
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
    /// Builds a `url` entry with the specified URL as the content of the
    /// `loc` child entry.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sitemap_xml::writer::Url;
    /// # fn main() -> anyhow::Result<()> {
    /// Url::loc("http://www.example.com/")?;
    ///
    /// Url::loc(::url::Url::parse("http://www.example.com/")?)?;
    /// #     Ok(())
    /// # }
    /// ```
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

    /// Changes the `changefreq` child entry to the specified value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sitemap_xml::writer::{Changefreq, Url};
    /// # fn main() -> anyhow::Result<()> {
    /// Url::loc("http://www.example.com/")?
    ///     .changefreq("monthly")?;
    ///
    /// Url::loc("http://www.example.com/")?
    ///     .changefreq(Changefreq::Monthly)?;
    /// #     Ok(())
    /// # }
    /// ```
    pub fn changefreq<S>(mut self, s: S) -> Result<Self>
    where
        S: TryInto<Changefreq>,
    {
        let changefreq = s.try_into().map_err(|_| Error::InvalidChangefreq)?;
        self.changefreq = Some(changefreq);
        Ok(self)
    }

    /// Changes the `lastmod` child entry to the specified date or datetime.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sitemap_xml::writer::Url;
    /// # fn main() -> anyhow::Result<()> {
    /// Url::loc("http://www.example.com/")?
    ///     .lastmod("2004-10-01T18:23:17+00:00")?;
    ///
    /// Url::loc("http://www.example.com/")?
    ///     .lastmod(::time::macros::datetime!(2004-10-01 18:23:17+00:00))?;
    /// #     Ok(())
    /// # }
    /// ```
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

    /// Changes the `priority` child entry to the specified value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sitemap_xml::writer::Url;
    /// # fn main() -> anyhow::Result<()> {
    /// Url::loc("http://www.example.com/")?
    ///     .priority("0.8")?;
    ///
    /// Url::loc("http://www.example.com/")?
    ///     .priority(0.8)?;
    /// #     Ok(())
    /// # }
    /// ```
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
