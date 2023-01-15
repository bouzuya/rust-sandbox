# sitemaps

The sitemap-xml crate provides writers for [`sitemap.xml`](https://www.sitemaps.org/).

## Usage

```toml
[dependencies]
sitemap-xml = { git = "https://github.com/bouzuya/rust-sandbox.git", tag = "sitemap-xml/0.2.0" }
```

## Writing sitemap file

```rust
use sitemap_xml::{SitemapWriter, Url};
use std::io::Cursor;

let mut writer = SitemapWriter::start(Cursor::new(Vec::new()))?;
writer.write(
    Url::loc("http://www.example.com/")?
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
```

## Writing sitemap index file

```rust
use sitemap_xml::writer::{SitemapIndexWriter};
use std::io::Cursor;

let mut writer = SitemapIndexWriter::start(Cursor::new(Vec::new()))?;
writer.write(
    Sitemap::loc("http://www.example.com/sitemap1.xml.gz")?
        .lastmod("2004-10-01T18:23:17+00:00")?,
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
```

## Roadmap

- v0.1.0 SitemapWriter (`<urlset>`, `<url>`, ...)
- v0.2.0 SitemapIndexWriter (`<sitemapindex>`, `<sitemap>`, ...)
- v0.3.0 SitemapReader
- v0.4.0 SitemapIndexReader
