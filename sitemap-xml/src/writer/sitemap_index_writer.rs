use std::io::Write;

use self::private::SealedTryIntoSitemap;

use super::{sitemap_xml_writer::SitemapXmlWriter, Sitemap};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid lastmod")]
    InvalidLastmod,
    #[error("invalid loc")]
    InvalidLoc,
    #[error("io")]
    Io(#[from] std::io::Error),
    #[error("max byte length is 50 MiB (52,428,800 bytes)")]
    MaxByteLength,
    #[error("max number of sitemaps is 50,000")]
    MaxNumberOfSitemaps,
}

impl From<crate::writer::sitemap_xml_writer::Error> for Error {
    fn from(value: crate::writer::sitemap_xml_writer::Error) -> Self {
        match value {
            super::sitemap_xml_writer::Error::Io(e) => Error::Io(e),
            super::sitemap_xml_writer::Error::MaxByteLength => Error::MaxByteLength,
        }
    }
}

type Result<T, E = Error> = std::result::Result<T, E>;

/// A writer for sitemap index file.
pub struct SitemapIndexWriter<W: Write> {
    writer: SitemapXmlWriter<W>,
    number_of_sitemaps: usize,
}

impl<W: Write> SitemapIndexWriter<W> {
    const MAX_NUMBER_OF_SITEMAPS: usize = 50_000;

    pub fn start(inner: W) -> Result<Self> {
        Self::start_inner(inner, false)
    }

    pub fn start_with_indent(inner: W) -> Result<Self> {
        Self::start_inner(inner, true)
    }

    pub fn write<'a, S>(&mut self, sitemap: S) -> Result<()>
    where
        S: SealedTryIntoSitemap<'a>,
    {
        if self.number_of_sitemaps + 1 > Self::MAX_NUMBER_OF_SITEMAPS {
            return Err(Error::MaxNumberOfSitemaps);
        }
        self.number_of_sitemaps += 1;

        let sitemap: Sitemap<'a> = sitemap.try_into_sitemap()?;
        self.writer.start_tag(b"sitemap")?;

        let content = sitemap.loc;
        self.writer.element(b"loc", content.as_ref())?;

        if let Some(content) = sitemap.lastmod {
            self.writer.element(b"lastmod", content.as_ref())?;
        }

        self.writer.end_tag(b"sitemap")?;
        Ok(())
    }

    pub fn end(&mut self) -> Result<()> {
        self.writer.end_tag(b"sitemapindex")?;
        Ok(())
    }

    pub fn into_inner(self) -> W {
        self.writer.into_inner()
    }

    fn start_inner(inner: W, pretty: bool) -> Result<Self> {
        let mut s = Self {
            writer: SitemapXmlWriter::new(inner, pretty),
            number_of_sitemaps: 0_usize,
        };
        s.writer.declaration()?;
        s.writer.start_tag_with_default_ns(b"sitemapindex")?;
        Ok(s)
    }
}

mod private {
    use crate::writer::Sitemap;

    use super::Error;

    pub trait SealedTryIntoSitemap<'a> {
        fn try_into_sitemap(self) -> Result<Sitemap<'a>, Error>;
    }

    impl<'a> SealedTryIntoSitemap<'a> for Sitemap<'a> {
        fn try_into_sitemap(self) -> Result<Sitemap<'a>, Error> {
            Ok(self)
        }
    }

    impl<'a> SealedTryIntoSitemap<'a> for &'a str {
        fn try_into_sitemap(self) -> Result<Sitemap<'a>, Error> {
            Sitemap::loc(self)
        }
    }
}
