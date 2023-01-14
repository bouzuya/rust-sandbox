use sitemap_xml::writer::{Changefreq, Lastmod, Loc, Priority, SitemapWriter, Url};

use std::io::Cursor;

#[test]
fn test_sitemap_writer_start_with_indent() -> anyhow::Result<()> {
    let mut sitemap_writer = SitemapWriter::start_with_indent(Cursor::new(Vec::new()))?;
    sitemap_writer.write("http://www.example.com/")?;
    sitemap_writer.end()?;
    let actual = String::from_utf8(sitemap_writer.into_inner().into_inner())?;
    let expected = r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url>
    <loc>http://www.example.com/</loc>
  </url>
</urlset>"#;
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn test_sitemap_writer_write_str() -> anyhow::Result<()> {
    let mut sitemap_writer = SitemapWriter::start(Cursor::new(Vec::new()))?;
    sitemap_writer.write("http://www.example.com/")?;
    sitemap_writer.end()?;
    let actual = String::from_utf8(sitemap_writer.into_inner().into_inner())?;
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
fn test_sitemap_writer_write_url() -> anyhow::Result<()> {
    let mut sitemap_writer = SitemapWriter::start(Cursor::new(Vec::new()))?;
    sitemap_writer.write(
        Url::loc("http://www.example.com/")?
            .lastmod("2005-01-01")?
            .changefreq("monthly")?
            .priority("0.8")?,
    )?;
    sitemap_writer.write(
        // <https://crates.io/crates/url> support
        // If you want to ensure that the URL is Valid, use `::url::Url`.
        // If you use &str, the URL is assumed to be valid and only the length check and XML entity escaping are performed.
        Url::loc(::url::Url::parse(
            "http://www.example.com/catalog?item=12&desc=vacation_hawaii",
        )?)?
        .changefreq(Changefreq::Monthly)?,
    )?;
    #[rustfmt::skip]
    sitemap_writer.write(
        Url::loc("http://www.example.com/catalog?item=73&desc=vacation_new_zealand")?
            // <https://crates.io/crates/time> support (`time::Date`)
            .lastmod(::time::macros::date!(2004-12-23))?
            .changefreq(Changefreq::Weekly)?
    )?;
    #[rustfmt::skip]
    sitemap_writer.write(
        Url::loc("http://www.example.com/catalog?item=74&desc=vacation_newfoundland")?
            // <https://crates.io/crates/time> support (`time::DateTime`)
            .lastmod(::time::macros::datetime!(2004-12-23 18:00:15 +00:00))?
            .priority(0.3)?
    )?;
    sitemap_writer.write(
        Url::loc("http://www.example.com/catalog?item=83&desc=vacation_usa")?
            .lastmod("2004-11-23")?,
    )?;
    sitemap_writer.end()?;
    let actual = String::from_utf8(sitemap_writer.into_inner().into_inner())?;
    // Sample XML Sitemap in <https://sitemaps.org/protocol.html>
    let expected = concat!(
        r#"<?xml version="1.0" encoding="UTF-8"?>"#,
        r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#,
        r#"<url>"#,
        r#"<loc>http://www.example.com/</loc>"#,
        r#"<lastmod>2005-01-01</lastmod>"#,
        r#"<changefreq>monthly</changefreq>"#,
        r#"<priority>0.8</priority>"#,
        r#"</url>"#,
        r#"<url>"#,
        r#"<loc>http://www.example.com/catalog?item=12&amp;desc=vacation_hawaii</loc>"#,
        r#"<changefreq>monthly</changefreq>"#,
        r#"</url>"#,
        r#"<url>"#,
        r#"<loc>http://www.example.com/catalog?item=73&amp;desc=vacation_new_zealand</loc>"#,
        r#"<lastmod>2004-12-23</lastmod>"#,
        r#"<changefreq>weekly</changefreq>"#,
        r#"</url>"#,
        r#"<url>"#,
        r#"<loc>http://www.example.com/catalog?item=74&amp;desc=vacation_newfoundland</loc>"#,
        // r#"<lastmod>2004-12-23T18:00:15+00:00</lastmod>"#,
        r#"<lastmod>2004-12-23T18:00:15.000000000Z</lastmod>"#,
        r#"<priority>0.3</priority>"#,
        r#"</url>"#,
        r#"<url>"#,
        r#"<loc>http://www.example.com/catalog?item=83&amp;desc=vacation_usa</loc>"#,
        r#"<lastmod>2004-11-23</lastmod>"#,
        r#"</url>"#,
        r#"</urlset>"#
    );
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn test_sitemap_writer_write_url_typed() -> anyhow::Result<()> {
    let mut sitemap_writer = SitemapWriter::start(Cursor::new(Vec::new()))?;
    sitemap_writer.write(
        Url::loc(Loc::try_from("http://www.example.com/")?)?
            .lastmod(Lastmod::try_from("2005-01-01")?)?
            .changefreq(Changefreq::try_from("monthly")?)?
            .priority(Priority::try_from("0.8")?)?,
    )?;
    sitemap_writer.end()?;
    let actual = String::from_utf8(sitemap_writer.into_inner().into_inner())?;
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
fn test_sitemap_writer_write_url_loc() -> anyhow::Result<()> {
    let mut sitemap_writer = SitemapWriter::start(Cursor::new(Vec::new()))?;
    sitemap_writer.write(Url::loc("http://www.example.com/")?)?;
    sitemap_writer.end()?;
    let actual = String::from_utf8(sitemap_writer.into_inner().into_inner())?;
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
fn test_sitemap_writer_write_url_lastmod() -> anyhow::Result<()> {
    let mut sitemap_writer = SitemapWriter::start(Cursor::new(Vec::new()))?;
    sitemap_writer.write(Url::loc("http://www.example.com/")?.lastmod("2005-01-01")?)?;
    sitemap_writer.end()?;
    let actual = String::from_utf8(sitemap_writer.into_inner().into_inner())?;
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
fn test_sitemap_writer_write_url_changefreq() -> anyhow::Result<()> {
    let mut sitemap_writer = SitemapWriter::start(Cursor::new(Vec::new()))?;
    sitemap_writer.write(Url::loc("http://www.example.com/")?.changefreq("monthly")?)?;
    sitemap_writer.end()?;
    let actual = String::from_utf8(sitemap_writer.into_inner().into_inner())?;
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
fn test_sitemap_writer_write_url_priority() -> anyhow::Result<()> {
    let mut sitemap_writer = SitemapWriter::start(Cursor::new(Vec::new()))?;
    sitemap_writer.write(Url::loc("http://www.example.com/")?.priority("0.8")?)?;
    sitemap_writer.end()?;
    let actual = String::from_utf8(sitemap_writer.into_inner().into_inner())?;
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
fn test_sitemap_writer_max_byte_length() -> anyhow::Result<()> {
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
fn test_sitemap_writer_max_number_of_urls() -> anyhow::Result<()> {
    let mut writer = SitemapWriter::start(Cursor::new(Vec::new()))?;
    for _ in 0..50_000 {
        writer.write("http://www.example.com/")?;
    }
    writer.end()?;

    let mut writer = SitemapWriter::start(Cursor::new(Vec::new()))?;
    for _ in 0..50_000 {
        writer.write("http://www.example.com/")?;
    }
    assert!(writer.write("http://www.example.com/").is_err());
    writer.end()?;
    Ok(())
}
