use std::io::Write;

use quick_xml::{
    events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event},
    Writer,
};

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

    pub fn write(&mut self, url_loc: &str) -> Result<()> {
        self.0.write_event(Event::Start(BytesStart::new("url")))?;
        self.0.write_event(Event::Start(BytesStart::new("loc")))?;
        self.0.write_event(Event::Text(BytesText::new(url_loc)))?;
        self.0.write_event(Event::End(BytesEnd::new("loc")))?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
    use quick_xml::writer::Writer;
    use std::io::Cursor;

    #[test]
    fn test() -> anyhow::Result<()> {
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
    fn test_quick_xml() -> anyhow::Result<()> {
        let mut writer = Writer::new(Cursor::new(Vec::new()));
        writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))?;
        let mut elm = BytesStart::new("urlset");
        elm.push_attribute(("xmlns", "http://www.sitemaps.org/schemas/sitemap/0.9"));
        writer.write_event(Event::Start(elm))?;

        writer.write_event(Event::Start(BytesStart::new("url")))?;
        writer.write_event(Event::Start(BytesStart::new("loc")))?;
        writer.write_event(Event::Text(BytesText::new("http://www.example.com/")))?;
        writer.write_event(Event::End(BytesEnd::new("loc")))?;
        writer.write_event(Event::End(BytesEnd::new("url")))?;

        writer.write_event(Event::End(BytesEnd::new("urlset")))?;
        let result = writer.into_inner().into_inner();
        let expected = concat!(
            r#"<?xml version="1.0" encoding="UTF-8"?>"#,
            r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#,
            r#"<url>"#,
            r#"<loc>http://www.example.com/</loc>"#,
            r#"</url>"#,
            r#"</urlset>"#
        );
        assert_eq!(result, expected.as_bytes());
        Ok(())
    }
}
