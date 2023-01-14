use sitemap_xml::writer::{Lastmod, Loc, Sitemap, SitemapIndexWriter};

use std::io::Cursor;

#[test]
fn test_sitemap_index_writer_start_with_indent() -> anyhow::Result<()> {
    let mut writer = SitemapIndexWriter::start_with_indent(Cursor::new(Vec::new()))?;
    writer.write("http://www.example.com/sitemap1.xml.gz")?;
    writer.end()?;
    let actual = String::from_utf8(writer.into_inner().into_inner())?;
    let expected = r#"<?xml version="1.0" encoding="UTF-8"?>
<sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <sitemap>
    <loc>http://www.example.com/sitemap1.xml.gz</loc>
  </sitemap>
</sitemapindex>"#;
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn test_sitemap_index_writer_write_str() -> anyhow::Result<()> {
    let mut writer = SitemapIndexWriter::start(Cursor::new(Vec::new()))?;
    writer.write("http://www.example.com/sitemap1.xml.gz")?;
    writer.end()?;
    let actual = String::from_utf8(writer.into_inner().into_inner())?;
    let expected = concat!(
        r#"<?xml version="1.0" encoding="UTF-8"?>"#,
        r#"<sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#,
        r#"<sitemap>"#,
        r#"<loc>http://www.example.com/sitemap1.xml.gz</loc>"#,
        r#"</sitemap>"#,
        r#"</sitemapindex>"#
    );
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn test_sitemap_index_writer_write_sitemap() -> anyhow::Result<()> {
    let mut writer = SitemapIndexWriter::start(Cursor::new(Vec::new()))?;
    writer.write(
        Sitemap::loc("http://www.example.com/sitemap1.xml.gz")?
            .lastmod("2004-10-01T18:23:17+00:00")?,
    )?;
    #[rustfmt::skip]
    writer.write(
        // <https://crates.io/crates/url> support
        // If you want to ensure that the URL is Valid, use `::url::Url`.
        // If you use &str, the URL is assumed to be valid and only the length check and XML entity escaping are performed.
        Sitemap::loc(::url::Url::parse("http://www.example.com/sitemap2.xml.gz")?)?
            // <https://crates.io/crates/time> support (`time::Date`, `time::DateTime`)
            .lastmod(::time::macros::date!(2005-01-01))?,
    )?;
    writer.end()?;
    let actual = String::from_utf8(writer.into_inner().into_inner())?;
    // Sample XML Sitemap Index in <https://sitemaps.org/protocol.html>
    let expected = concat!(
        r#"<?xml version="1.0" encoding="UTF-8"?>"#,
        r#"<sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#,
        r#"<sitemap>"#,
        r#"<loc>http://www.example.com/sitemap1.xml.gz</loc>"#,
        r#"<lastmod>2004-10-01T18:23:17+00:00</lastmod>"#,
        r#"</sitemap>"#,
        r#"<sitemap>"#,
        r#"<loc>http://www.example.com/sitemap2.xml.gz</loc>"#,
        r#"<lastmod>2005-01-01</lastmod>"#,
        r#"</sitemap>"#,
        r#"</sitemapindex>"#
    );
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn test_sitemap_index_writer_write_sitemap_typed() -> anyhow::Result<()> {
    let mut writer = SitemapIndexWriter::start(Cursor::new(Vec::new()))?;
    writer.write(
        Sitemap::loc(Loc::try_from("http://www.example.com/sitemap1.xml.gz")?)?
            .lastmod(Lastmod::try_from("2004-10-01T18:23:17+00:00")?)?,
    )?;
    writer.end()?;
    let actual = String::from_utf8(writer.into_inner().into_inner())?;
    let expected = concat!(
        r#"<?xml version="1.0" encoding="UTF-8"?>"#,
        r#"<sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#,
        r#"<sitemap>"#,
        r#"<loc>http://www.example.com/sitemap1.xml.gz</loc>"#,
        r#"<lastmod>2004-10-01T18:23:17+00:00</lastmod>"#,
        r#"</sitemap>"#,
        r#"</sitemapindex>"#
    );
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn test_sitemap_index_writer_write_sitemap_loc_only() -> anyhow::Result<()> {
    let mut writer = SitemapIndexWriter::start(Cursor::new(Vec::new()))?;
    writer.write(Sitemap::loc("http://www.example.com/sitemap1.xml.gz")?)?;
    writer.end()?;
    let actual = String::from_utf8(writer.into_inner().into_inner())?;
    let expected = concat!(
        r#"<?xml version="1.0" encoding="UTF-8"?>"#,
        r#"<sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#,
        r#"<sitemap>"#,
        r#"<loc>http://www.example.com/sitemap1.xml.gz</loc>"#,
        r#"</sitemap>"#,
        r#"</sitemapindex>"#
    );
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn test_sitemap_index_writer_max_byte_length() -> anyhow::Result<()> {
    let head_and_tail_length = concat!(
        r#"<?xml version="1.0" encoding="UTF-8"?>"#,
        r#"<sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#,
        r#"</sitemapindex>"#
    )
    .len();
    let url = format!("http://www.example.com/{}", "x".repeat(1027));
    let url_length = format!(r#"<sitemap><loc>{}</loc></sitemap>"#, url).len();
    let url2 = format!("http://www.example.com/{}", "x".repeat(28));
    let url2_length = format!(r#"<sitemap><loc>{}</loc></sitemap>"#, url2).len();
    let url3 = format!("http://www.example.com/{}", "x".repeat(29));
    let url3_length = format!(r#"<sitemap><loc>{}</loc></sitemap>"#, url3).len();
    assert_eq!(head_and_tail_length, 119);
    assert_eq!(url_length, 1_080);
    assert_eq!(url2_length, 81);
    assert_eq!(url3_length, 82);

    // 119 + 1_080 * 48_545 = 52_428_719
    // MAX_BYTE_LENGTH      = 52_428_800
    // 52_428_800 - 52_428_707 = 81

    let mut writer = SitemapIndexWriter::start(Cursor::new(Vec::new()))?;
    for _ in 0..48_545 {
        writer.write(url.as_str())?;
    }
    writer.write(url2.as_str())?;
    writer.end()?;

    let mut writer = SitemapIndexWriter::start(Cursor::new(Vec::new()))?;
    for _ in 0..48_545 {
        writer.write(url.as_str())?;
    }
    writer.write(url3.as_str())?;
    assert!(writer.end().is_err());
    Ok(())
}

#[test]
fn test_sitemap_index_writer_max_number_of_urls() -> anyhow::Result<()> {
    let mut writer = SitemapIndexWriter::start(Cursor::new(Vec::new()))?;
    for _ in 0..50_000 {
        writer.write("http://www.example.com/sitemap1.xml.gz")?;
    }
    writer.end()?;

    let mut writer = SitemapIndexWriter::start(Cursor::new(Vec::new()))?;
    for _ in 0..50_000 {
        writer.write("http://www.example.com/sitemap1.xml.gz")?;
    }
    assert!(writer
        .write("http://www.example.com/sitemap1.xml.gz")
        .is_err());
    writer.end()?;
    Ok(())
}
