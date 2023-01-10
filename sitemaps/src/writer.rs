#[cfg(test)]
mod tests {
    use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
    use quick_xml::writer::Writer;
    use std::io::Cursor;

    #[test]
    fn test() -> anyhow::Result<()> {
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
