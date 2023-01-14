use std::{borrow::Cow, io::Write};

use crate::writer::{url::Url, Error, Result};

use self::private::SealedTryIntoUrl;

pub struct SitemapWriter<W: Write> {
    write: W,
    byte_length: usize,
    number_of_urls: usize,
    pretty: bool,
}

impl<W: Write> SitemapWriter<W> {
    const MAX_BYTE_LENGTH: usize = 52_428_800;
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
        self.write_indent(1)?;
        self.write_inner(br#"<url>"#)?;

        let content = url.loc;
        self.write_element(b"loc", content.as_ref())?;

        if let Some(content) = url.lastmod {
            self.write_element(b"lastmod", content.as_ref())?;
        }

        if let Some(content) = url.changefreq {
            self.write_element(b"changefreq", content.as_ref())?;
        }

        if let Some(content) = url.priority {
            self.write_element(b"priority", content.as_ref())?;
        }

        self.write_indent(1)?;
        self.write_inner(br#"</url>"#)?;
        Ok(())
    }

    pub fn end(&mut self) -> Result<()> {
        self.write_indent(0)?;
        self.write_inner(br#"</urlset>"#)
    }

    pub fn into_inner(self) -> W {
        self.write
    }

    fn start_inner(inner: W, pretty: bool) -> Result<Self> {
        let mut s = Self {
            write: inner,
            byte_length: 0_usize,
            number_of_urls: 0_usize,
            pretty,
        };
        s.write_inner(br#"<?xml version="1.0" encoding="UTF-8"?>"#)?;
        s.write_indent(0)?;
        s.write_inner(br#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#)?;
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
    use crate::writer::Url;

    pub trait SealedTryIntoUrl<'a> {
        fn try_into_url(self) -> Result<Url<'a>, crate::writer::Error>;
    }

    impl<'a> SealedTryIntoUrl<'a> for Url<'a> {
        fn try_into_url(self) -> Result<Url<'a>, crate::writer::Error> {
            Ok(self)
        }
    }

    impl<'a> SealedTryIntoUrl<'a> for &'a str {
        fn try_into_url(self) -> Result<Url<'a>, crate::writer::Error> {
            Url::loc(self)
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
