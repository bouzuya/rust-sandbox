mod changefreq;
mod priority;

use std::io::Write;

use quick_xml::{
    events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event},
    Writer,
};

use self::{changefreq::Changefreq, priority::Priority};

// TODO: improve error
#[derive(Clone, Debug, thiserror::Error)]
#[error("error")]
pub struct Error;

impl From<quick_xml::Error> for Error {
    fn from(_value: quick_xml::Error) -> Self {
        Self
    }
}

type Result<T, E = Error> = std::result::Result<T, E>;

pub struct SitemapWriter<W: Write>(Writer<W>);

impl<W: Write> SitemapWriter<W> {
    pub fn start(inner: W) -> Result<Self> {
        let mut writer = Writer::new(inner);
        writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))?;
        writer.write_event(Event::Start({
            let mut elm = BytesStart::new("urlset");
            elm.push_attribute(("xmlns", "http://www.sitemaps.org/schemas/sitemap/0.9"));
            elm
        }))?;
        Ok(Self(writer))
    }

    pub fn write<'a, U>(&mut self, url: U) -> Result<()>
    where
        U: Into<Url<'a>>,
    {
        let url = url.into();
        self.0.write_event(Event::Start(BytesStart::new("url")))?;

        let name = "loc";
        let content = url.loc;
        self.0.write_event(Event::Start(BytesStart::new(name)))?;
        self.0.write_event(Event::Text(BytesText::new(content)))?;
        self.0.write_event(Event::End(BytesEnd::new(name)))?;

        if let Some(content) = url.lastmod {
            let name = "lastmod";
            self.0.write_event(Event::Start(BytesStart::new(name)))?;
            self.0.write_event(Event::Text(BytesText::new(content)))?;
            self.0.write_event(Event::End(BytesEnd::new(name)))?;
        }

        if let Some(changefreq) = url.changefreq {
            let name = "changefreq";
            let content = changefreq.as_ref();
            self.0.write_event(Event::Start(BytesStart::new(name)))?;
            self.0.write_event(Event::Text(BytesText::new(content)))?;
            self.0.write_event(Event::End(BytesEnd::new(name)))?;
        }

        if let Some(priority) = url.priority {
            let name = "priority";
            let content = priority.to_string();
            let content = content.as_str();
            self.0.write_event(Event::Start(BytesStart::new(name)))?;
            self.0.write_event(Event::Text(BytesText::new(content)))?;
            self.0.write_event(Event::End(BytesEnd::new(name)))?;
        }

        self.0.write_event(Event::End(BytesEnd::new("url")))?;
        Ok(())
    }

    pub fn end(&mut self) -> Result<()> {
        self.0.write_event(Event::End(BytesEnd::new("urlset")))?;
        Ok(())
    }

    pub fn into_inner(self) -> W {
        self.0.into_inner()
    }
}

pub struct Url<'a> {
    pub loc: &'a str,
    pub lastmod: Option<&'a str>,
    pub changefreq: Option<Changefreq>,
    pub priority: Option<Priority>,
}

impl<'a> From<&'a str> for Url<'a> {
    fn from(loc: &'a str) -> Self {
        Self::builder(loc).build()
    }
}

impl<'a> Url<'a> {
    pub fn builder(loc: &'a str) -> UrlBuilder {
        UrlBuilder {
            loc,
            lastmod: None,
            changefreq: None,
            priority: None,
        }
    }
}

pub struct UrlBuilder<'a> {
    loc: &'a str,
    lastmod: Option<&'a str>,
    changefreq: Option<Changefreq>,
    priority: Option<Priority>,
}

impl<'a> UrlBuilder<'a> {
    pub fn build(self) -> Url<'a> {
        Url {
            loc: self.loc,
            lastmod: self.lastmod,
            changefreq: self.changefreq,
            priority: self.priority,
        }
    }

    pub fn changefreq<S>(mut self, s: S) -> Result<Self>
    where
        S: TryInto<Changefreq>,
    {
        let changefreq = s.try_into().map_err(|_| Error)?;
        self.changefreq = Some(changefreq);
        Ok(self)
    }

    pub fn lastmod(mut self, s: &'a str) -> Self {
        self.lastmod = Some(s);
        self
    }

    pub fn priority<S>(mut self, s: S) -> Result<Self>
    where
        S: TryInto<Priority>,
    {
        let priority = s.try_into().map_err(|_| Error)?;
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
            Url::builder("http://www.example.com/")
                .lastmod("2005-01-01")
                .changefreq("monthly")?
                .priority("0.8")?
                .build(),
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
        writer.write(Url::builder("http://www.example.com/").build())?;
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
            Url::builder("http://www.example.com/")
                .lastmod("2005-01-01")
                .build(),
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
    fn test_url_builder_changefreq() -> anyhow::Result<()> {
        let mut writer = SitemapWriter::start(Cursor::new(Vec::new()))?;
        writer.write(
            Url::builder("http://www.example.com/")
                .changefreq(Changefreq::Monthly)?
                .build(),
        )?;
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
        writer.write(
            Url::builder("http://www.example.com/")
                .changefreq("monthly")?
                .build(),
        )?;
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
        writer.write(
            Url::builder("http://www.example.com/")
                .priority(Priority::try_from("0.8")?)?
                .build(),
        )?;
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
        writer.write(
            Url::builder("http://www.example.com/")
                .priority(0.8_f64)?
                .build(),
        )?;
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
        writer.write(
            Url::builder("http://www.example.com/")
                .priority("0.8")?
                .build(),
        )?;
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
}
