mod changefreq;
mod lastmod;
mod priority;

use std::{borrow::Cow, io::Write};

use self::{changefreq::Changefreq, lastmod::Lastmod, priority::Priority};

// TODO: improve error
#[derive(Clone, Debug, thiserror::Error)]
pub enum Error {
    #[error("max byte length is 50 MB (52,428,800 bytes)")]
    MaxByteLength,
    #[error("max number of urls is 50,000")]
    MaxNumberOfUrls,
    #[error("uncategorized")]
    Uncategorized,
}

type Result<T, E = Error> = std::result::Result<T, E>;

pub struct SitemapWriter<W: Write> {
    write: W,
    byte_length: usize,
    number_of_urls: usize,
}

impl<W: Write> SitemapWriter<W> {
    const MAX_BYTE_LENGTH: usize = 52_428_800;
    const MAX_NUMBER_OF_URLS: usize = 50_000;

    pub fn start(inner: W) -> Result<Self> {
        let mut s = Self {
            write: inner,
            byte_length: 0_usize,
            number_of_urls: 0_usize,
        };
        s.write_inner(br#"<?xml version="1.0" encoding="UTF-8"?>"#)?;
        s.write_inner(br#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#)?;
        Ok(s)
    }

    pub fn write<'a, U>(&mut self, url: U) -> Result<()>
    where
        U: Into<Url<'a>>,
    {
        if self.number_of_urls + 1 > Self::MAX_NUMBER_OF_URLS {
            return Err(Error::MaxNumberOfUrls);
        }
        self.number_of_urls += 1;

        let url = url.into();
        self.write_inner(br#"<url>"#)?;

        let content = url.loc;
        self.write_inner(br#"<loc>"#)?;
        self.write_inner(entity_escape(content).as_bytes())?;
        self.write_inner(br#"</loc>"#)?;

        if let Some(lastmod) = url.lastmod {
            let content = lastmod.to_string();
            let content = content.as_ref();
            self.write_inner(br#"<lastmod>"#)?;
            self.write_inner(entity_escape(content).as_bytes())?;
            self.write_inner(br#"</lastmod>"#)?;
        }

        if let Some(changefreq) = url.changefreq {
            let content = changefreq.as_ref();
            self.write_inner(br#"<changefreq>"#)?;
            self.write_inner(entity_escape(content).as_bytes())?;
            self.write_inner(br#"</changefreq>"#)?;
        }

        if let Some(priority) = url.priority {
            let content = priority.to_string();
            let content = content.as_str();
            self.write_inner(br#"<priority>"#)?;
            self.write_inner(entity_escape(content).as_bytes())?;
            self.write_inner(br#"</priority>"#)?;
        }

        self.write_inner(br#"</url>"#)?;
        Ok(())
    }

    pub fn end(&mut self) -> Result<()> {
        self.write_inner(br#"</urlset>"#)
    }

    pub fn into_inner(self) -> W {
        self.write
    }

    fn write_inner(&mut self, buf: &[u8]) -> Result<()> {
        let l = buf.len();
        if self.byte_length + l > Self::MAX_BYTE_LENGTH {
            return Err(Error::MaxByteLength);
        }
        self.byte_length += l;

        self.write
            .write_all(buf)
            .map_err(|_| Error::Uncategorized)?;
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

pub struct Url<'a> {
    loc: &'a str,
    lastmod: Option<Lastmod>,
    changefreq: Option<Changefreq>,
    priority: Option<Priority>,
}

impl<'a> From<&'a str> for Url<'a> {
    fn from(loc: &'a str) -> Self {
        Self::loc(loc)
    }
}

impl<'a> Url<'a> {
    pub fn loc(loc: &'a str) -> Self {
        Self {
            loc,
            lastmod: None,
            changefreq: None,
            priority: None,
        }
    }

    pub fn changefreq<S>(mut self, s: S) -> Result<Self>
    where
        S: TryInto<Changefreq>,
    {
        let changefreq = s.try_into().map_err(|_| Error::Uncategorized)?;
        self.changefreq = Some(changefreq);
        Ok(self)
    }

    pub fn lastmod<S>(mut self, s: S) -> Result<Self>
    where
        S: TryInto<Lastmod>,
    {
        let lastmod = s.try_into().map_err(|_| Error::Uncategorized)?;
        self.lastmod = Some(lastmod);
        Ok(self)
    }

    pub fn priority<S>(mut self, s: S) -> Result<Self>
    where
        S: TryInto<Priority>,
    {
        let priority = s.try_into().map_err(|_| Error::Uncategorized)?;
        self.priority = Some(priority);
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_url_from_str() -> anyhow::Result<()> {
        let mut writer = SitemapWriter::start(Cursor::new(Vec::new()))?;
        writer.write("http://www.example.com/")?;
        writer.end()?;
        let actual = String::from_utf8(writer.into_inner().into_inner())?;
        let expected = concat!(
            r#"<?xml version="1.0" encoding="UTF-8"?>"#,
            r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#,
            r#"<url>"#,
            r#"<loc>http://www.example.com/</loc>"#,
            r#"</url>"#,
            r#"</urlset>"#
        );
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_url_builder() -> anyhow::Result<()> {
        let mut writer = SitemapWriter::start(Cursor::new(Vec::new()))?;
        writer.write(
            Url::loc("http://www.example.com/")
                .lastmod("2005-01-01")?
                .changefreq("monthly")?
                .priority("0.8")?,
        )?;
        writer.end()?;
        let actual = String::from_utf8(writer.into_inner().into_inner())?;
        let expected = concat!(
            r#"<?xml version="1.0" encoding="UTF-8"?>"#,
            r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#,
            r#"<url>"#,
            r#"<loc>http://www.example.com/</loc>"#,
            r#"<lastmod>2005-01-01</lastmod>"#,
            r#"<changefreq>monthly</changefreq>"#,
            r#"<priority>0.8</priority>"#,
            r#"</url>"#,
            r#"</urlset>"#
        );
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_url_builder_loc() -> anyhow::Result<()> {
        let mut writer = SitemapWriter::start(Cursor::new(Vec::new()))?;
        writer.write(Url::loc("http://www.example.com/"))?;
        writer.end()?;
        let actual = String::from_utf8(writer.into_inner().into_inner())?;
        let expected = concat!(
            r#"<?xml version="1.0" encoding="UTF-8"?>"#,
            r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#,
            r#"<url>"#,
            r#"<loc>http://www.example.com/</loc>"#,
            r#"</url>"#,
            r#"</urlset>"#
        );
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_url_builder_lastmod() -> anyhow::Result<()> {
        let mut writer = SitemapWriter::start(Cursor::new(Vec::new()))?;
        writer.write(
            Url::loc("http://www.example.com/").lastmod(Lastmod::try_from("2005-01-01")?)?,
        )?;
        writer.end()?;
        let actual = String::from_utf8(writer.into_inner().into_inner())?;
        let expected = concat!(
            r#"<?xml version="1.0" encoding="UTF-8"?>"#,
            r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#,
            r#"<url>"#,
            r#"<loc>http://www.example.com/</loc>"#,
            r#"<lastmod>2005-01-01</lastmod>"#,
            r#"</url>"#,
            r#"</urlset>"#
        );
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_url_builder_lastmod_str() -> anyhow::Result<()> {
        let mut writer = SitemapWriter::start(Cursor::new(Vec::new()))?;
        writer.write(Url::loc("http://www.example.com/").lastmod("2005-01-01")?)?;
        writer.end()?;
        let actual = String::from_utf8(writer.into_inner().into_inner())?;
        let expected = concat!(
            r#"<?xml version="1.0" encoding="UTF-8"?>"#,
            r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#,
            r#"<url>"#,
            r#"<loc>http://www.example.com/</loc>"#,
            r#"<lastmod>2005-01-01</lastmod>"#,
            r#"</url>"#,
            r#"</urlset>"#
        );
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_url_builder_changefreq() -> anyhow::Result<()> {
        let mut writer = SitemapWriter::start(Cursor::new(Vec::new()))?;
        writer.write(Url::loc("http://www.example.com/").changefreq(Changefreq::Monthly)?)?;
        writer.end()?;
        let actual = String::from_utf8(writer.into_inner().into_inner())?;
        let expected = concat!(
            r#"<?xml version="1.0" encoding="UTF-8"?>"#,
            r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#,
            r#"<url>"#,
            r#"<loc>http://www.example.com/</loc>"#,
            r#"<changefreq>monthly</changefreq>"#,
            r#"</url>"#,
            r#"</urlset>"#
        );
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_url_builder_changefreq_str() -> anyhow::Result<()> {
        let mut writer = SitemapWriter::start(Cursor::new(Vec::new()))?;
        writer.write(Url::loc("http://www.example.com/").changefreq("monthly")?)?;
        writer.end()?;
        let actual = String::from_utf8(writer.into_inner().into_inner())?;
        let expected = concat!(
            r#"<?xml version="1.0" encoding="UTF-8"?>"#,
            r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#,
            r#"<url>"#,
            r#"<loc>http://www.example.com/</loc>"#,
            r#"<changefreq>monthly</changefreq>"#,
            r#"</url>"#,
            r#"</urlset>"#
        );
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_url_builder_priority() -> anyhow::Result<()> {
        let mut writer = SitemapWriter::start(Cursor::new(Vec::new()))?;
        writer.write(Url::loc("http://www.example.com/").priority(Priority::try_from("0.8")?)?)?;
        writer.end()?;
        let actual = String::from_utf8(writer.into_inner().into_inner())?;
        let expected = concat!(
            r#"<?xml version="1.0" encoding="UTF-8"?>"#,
            r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#,
            r#"<url>"#,
            r#"<loc>http://www.example.com/</loc>"#,
            r#"<priority>0.8</priority>"#,
            r#"</url>"#,
            r#"</urlset>"#
        );
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_url_builder_priority_f64() -> anyhow::Result<()> {
        let mut writer = SitemapWriter::start(Cursor::new(Vec::new()))?;
        writer.write(Url::loc("http://www.example.com/").priority(0.8_f64)?)?;
        writer.end()?;
        let actual = String::from_utf8(writer.into_inner().into_inner())?;
        let expected = concat!(
            r#"<?xml version="1.0" encoding="UTF-8"?>"#,
            r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#,
            r#"<url>"#,
            r#"<loc>http://www.example.com/</loc>"#,
            r#"<priority>0.8</priority>"#,
            r#"</url>"#,
            r#"</urlset>"#
        );
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_url_builder_priority_str() -> anyhow::Result<()> {
        let mut writer = SitemapWriter::start(Cursor::new(Vec::new()))?;
        writer.write(Url::loc("http://www.example.com/").priority("0.8")?)?;
        writer.end()?;
        let actual = String::from_utf8(writer.into_inner().into_inner())?;
        let expected = concat!(
            r#"<?xml version="1.0" encoding="UTF-8"?>"#,
            r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#,
            r#"<url>"#,
            r#"<loc>http://www.example.com/</loc>"#,
            r#"<priority>0.8</priority>"#,
            r#"</url>"#,
            r#"</urlset>"#
        );
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_max_byte_length() -> anyhow::Result<()> {
        let head_and_tail_length = concat!(
            r#"<?xml version="1.0" encoding="UTF-8"?>"#,
            r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#,
            r#"</urlset>"#
        )
        .len();
        let url = format!("http://www.example.com/{}", "x".repeat(1035));
        let url_length = format!(r#"<url><loc>{}</loc></url>"#, url).len();
        let url2 = format!("http://www.example.com/{}", "x".repeat(48));
        let url2_length = format!(r#"<url><loc>{}</loc></url>"#, url2).len();
        let url3 = format!("http://www.example.com/{}", "x".repeat(49));
        let url3_length = format!(r#"<url><loc>{}</loc></url>"#, url3).len();
        assert_eq!(head_and_tail_length, 107);
        assert_eq!(url_length, 1_080);
        assert_eq!(url2_length, 93);
        assert_eq!(url3_length, 94);

        // 107 + 1_080 * 48_545 = 52_428_707
        // MAX_BYTE_LENGTH      = 52_428_800
        // 52_428_800 - 52_428_707 = 93

        let mut writer = SitemapWriter::start(Cursor::new(Vec::new()))?;
        for _ in 0..48_545 {
            writer.write(url.as_str())?;
        }
        writer.write(url2.as_str())?;
        writer.end()?;

        let mut writer = SitemapWriter::start(Cursor::new(Vec::new()))?;
        for _ in 0..48_545 {
            writer.write(url.as_str())?;
        }
        writer.write(url3.as_str())?;
        assert!(writer.end().is_err());
        Ok(())
    }

    #[test]
    fn test_max_number_of_urls() -> anyhow::Result<()> {
        let mut writer = SitemapWriter::start(Cursor::new(Vec::new()))?;
        for _ in 0..50_000 {
            writer.write("http://www.example.com/")?;
        }
        assert!(writer.write("http://www.example.com/").is_err());
        writer.end()?;
        Ok(())
    }

    // TODO: test_entity_escape
}
