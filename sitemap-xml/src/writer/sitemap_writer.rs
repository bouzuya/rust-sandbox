use std::io::Write;

use crate::writer::url::Url;

use self::private::SealedTryIntoUrl;

use super::sitemap_xml_writer::SitemapXmlWriter;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid changefreq")]
    InvalidChangefreq,
    #[error("invalid lastmod")]
    InvalidLastmod,
    #[error("invalid loc")]
    InvalidLoc,
    #[error("invalid priority")]
    InvalidPriority,
    #[error("io")]
    Io(#[from] std::io::Error),
    #[error("max byte length is 50 MiB (52,428,800 bytes)")]
    MaxByteLength,
    #[error("max number of urls is 50,000")]
    MaxNumberOfUrls,
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

pub struct SitemapWriter<W: Write> {
    writer: SitemapXmlWriter<W>,
    number_of_urls: usize,
}

impl<W: Write> SitemapWriter<W> {
    const MAX_NUMBER_OF_URLS: usize = 50_000;

    pub fn start(inner: W) -> Result<Self> {
        Self::start_inner(inner, false)
    }

    pub fn start_with_indent(inner: W) -> Result<Self> {
        Self::start_inner(inner, true)
    }

    pub fn write<'a, U>(&mut self, url: U) -> Result<()>
    where
        U: SealedTryIntoUrl<'a>,
    {
        if self.number_of_urls + 1 > Self::MAX_NUMBER_OF_URLS {
            return Err(Error::MaxNumberOfUrls);
        }
        self.number_of_urls += 1;

        let url: Url<'a> = url.try_into_url()?;
        self.writer.start_tag(b"url")?;

        let content = url.loc;
        self.writer.element(b"loc", content.as_ref())?;

        if let Some(content) = url.lastmod {
            self.writer.element(b"lastmod", content.as_ref())?;
        }

        if let Some(content) = url.changefreq {
            self.writer.element(b"changefreq", content.as_ref())?;
        }

        if let Some(content) = url.priority {
            self.writer.element(b"priority", content.as_ref())?;
        }

        self.writer.end_tag(b"url")?;
        Ok(())
    }

    pub fn end(&mut self) -> Result<()> {
        self.writer.end_tag(b"urlset")?;
        Ok(())
    }

    pub fn into_inner(self) -> W {
        self.writer.into_inner()
    }

    fn start_inner(inner: W, pretty: bool) -> Result<Self> {
        let mut s = Self {
            writer: SitemapXmlWriter::new(inner, pretty),
            number_of_urls: 0_usize,
        };
        s.writer.declaration()?;
        s.writer.start_tag_with_default_ns(b"urlset")?;
        Ok(s)
    }
}

mod private {
    use crate::writer::Url;

    use super::Error;

    pub trait SealedTryIntoUrl<'a> {
        fn try_into_url(self) -> Result<Url<'a>, Error>;
    }

    impl<'a> SealedTryIntoUrl<'a> for Url<'a> {
        fn try_into_url(self) -> Result<Url<'a>, Error> {
            Ok(self)
        }
    }

    impl<'a> SealedTryIntoUrl<'a> for &'a str {
        fn try_into_url(self) -> Result<Url<'a>, Error> {
            Url::loc(self)
        }
    }
}
