use std::{borrow::Cow, io::Write};

use self::private::SealedTryIntoSitemap;

use super::Sitemap;

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

type Result<T, E = Error> = std::result::Result<T, E>;

pub struct SitemapIndexWriter<W: Write> {
    write: W,
    byte_length: usize,
    number_of_sitemaps: usize,
    pretty: bool,
}

impl<W: Write> SitemapIndexWriter<W> {
    const MAX_BYTE_LENGTH: usize = 52_428_800;
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
        self.write_indent(1)?;
        self.write_inner(br#"<sitemap>"#)?;

        let content = sitemap.loc;
        self.write_element(b"loc", content.as_ref())?;

        if let Some(content) = sitemap.lastmod {
            self.write_element(b"lastmod", content.as_ref())?;
        }

        self.write_indent(1)?;
        self.write_inner(br#"</sitemap>"#)?;
        Ok(())
    }

    pub fn end(&mut self) -> Result<()> {
        self.write_indent(0)?;
        self.write_inner(br#"</sitemapindex>"#)
    }

    pub fn into_inner(self) -> W {
        self.write
    }

    fn start_inner(inner: W, pretty: bool) -> Result<Self> {
        let mut s = Self {
            write: inner,
            byte_length: 0_usize,
            number_of_sitemaps: 0_usize,
            pretty,
        };
        s.write_inner(br#"<?xml version="1.0" encoding="UTF-8"?>"#)?;
        s.write_indent(0)?;
        s.write_inner(br#"<sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#)?;
        Ok(s)
    }

    fn write_element(&mut self, name: &[u8], content: &str) -> Result<()> {
        self.write_indent(2)?;
        self.write_inner(b"<")?;
        self.write_inner(name)?;
        self.write_inner(b">")?;
        self.write_inner(entity_escape(content).as_bytes())?;
        self.write_inner(b"</")?;
        self.write_inner(name)?;
        self.write_inner(b">")?;
        Ok(())
    }

    fn write_indent(&mut self, level: usize) -> Result<()> {
        if !self.pretty {
            return Ok(());
        }

        self.write_inner(b"\n")?;
        for _ in 0..level {
            self.write_inner(b"  ")?;
        }
        Ok(())
    }

    fn write_inner(&mut self, buf: &[u8]) -> Result<()> {
        let l = buf.len();
        if self.byte_length + l > Self::MAX_BYTE_LENGTH {
            return Err(Error::MaxByteLength);
        }
        self.byte_length += l;

        self.write.write_all(buf)?;
        Ok(())
    }
}

fn entity_escape(s: &str) -> Cow<str> {
    let predicate = |b: &u8| -> bool { matches!(b, b'"' | b'&' | b'\'' | b'<' | b'>') };
    let escape = |b: u8| -> &'static [u8] {
        match b {
            b'"' => b"&quot;",
            b'&' => b"&amp;",
            b'\'' => b"&apos;",
            b'<' => b"&lt;",
            b'>' => b"&gt;",
            _ => unreachable!(),
        }
    };

    let bytes = s.as_bytes();
    let mut iter = bytes.iter();
    if let Some(index) = iter.position(predicate) {
        let mut escaped = Vec::with_capacity(bytes.len());

        escaped.extend_from_slice(&bytes[..index]);
        escaped.extend_from_slice(escape(bytes[index]));
        let mut start = index + 1;
        while let Some(index) = iter.position(predicate) {
            let index = start + index;
            escaped.extend_from_slice(&bytes[start..index]);
            escaped.extend_from_slice(escape(bytes[index]));
            start = index + 1;
        }
        if let Some(tail) = bytes.get(start..) {
            escaped.extend_from_slice(tail);
        }

        Cow::Owned(String::from_utf8(escaped).expect("valid UTF-8"))
    } else {
        Cow::Borrowed(s)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(entity_escape("abc"), "abc");
        assert_eq!(entity_escape("\"&'<>"), "&quot;&amp;&apos;&lt;&gt;");
        assert_eq!(
            entity_escape(r#"<h1 class="title">"#),
            "&lt;h1 class=&quot;title&quot;&gt;"
        );
    }
}
