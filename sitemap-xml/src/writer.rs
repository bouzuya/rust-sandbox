//! provides writers for [`sitemap.xml`](https://www.sitemaps.org/).
//!
//! ## Writers
//!
//! - [`SitemapWriter`]: A writer for sitemap file.
//! - [`SitemapIndexWriter`]: A writer for sitemap index file.
//!
//! ## Example: `SitemapWriter`
//!
//! The following example is a sitemap containing only one URL specified by `&str`.
//!
//! ```rust
//! use sitemap_xml::writer::{Changefreq, Lastmod, Loc, Priority, SitemapWriter, Url};
//! use std::io::Cursor;
//!
//! # fn main() -> anyhow::Result<()> {
//! let mut writer = SitemapWriter::start(Cursor::new(Vec::new()))?;
//! writer.write("http://www.example.com/")?;
//! writer.end()?;
//!
//! assert_eq!(
//!     String::from_utf8(writer.into_inner().into_inner())?,
//!     concat!(
//!         r#"<?xml version="1.0" encoding="UTF-8"?>"#,
//!         r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#,
//!         r#"<url>"#,
//!         r#"<loc>http://www.example.com/</loc>"#,
//!         r#"</url>"#,
//!         r#"</urlset>"#
//!     )
//! );
//! #    Ok(())
//! # }
//! ```
//!
//! The following example is a sitemap that uses all the optional tags. It also includes an example using non-string types.
//!
//! ```rust
//! use sitemap_xml::writer::{Changefreq, SitemapWriter, Url};
//! use std::io::Cursor;
//!
//! # fn main() -> anyhow::Result<()> {
//! let mut writer = SitemapWriter::start(Cursor::new(Vec::new()))?;
//! writer.write(
//!     Url::loc("http://www.example.com/")?
//!         .lastmod("2005-01-01")?
//!         .changefreq("monthly")?
//!         .priority("0.8")?,
//! )?;
//! writer.write(
//!     // <https://crates.io/crates/url> support
//!     // You can specify `::url::Url`.
//!     // If you want to ensure that the URL is valid, use `::url::Url`.
//!     // If you use &str, the URL is assumed to be valid and only the length
//!     // check and XML entity escaping are performed.
//!     Url::loc(::url::Url::parse("http://www.example.com/")?)?
//!         // <https://crates.io/crates/time> support
//!         // You can specify `::time::Date` and `::time::OffsetDateTime`.
//!         .lastmod(::time::macros::date!(2005-01-01))?
//!         .changefreq(Changefreq::Monthly)?
//!         .priority(0.8)?
//! )?;
//! writer.end()?;
//!
//! assert_eq!(
//!     String::from_utf8(writer.into_inner().into_inner())?,
//!     concat!(
//!         r#"<?xml version="1.0" encoding="UTF-8"?>"#,
//!         r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#,
//!         r#"<url>"#,
//!         r#"<loc>http://www.example.com/</loc>"#,
//!         r#"<lastmod>2005-01-01</lastmod>"#,
//!         r#"<changefreq>monthly</changefreq>"#,
//!         r#"<priority>0.8</priority>"#,
//!         r#"</url>"#,
//!         r#"<url>"#,
//!         r#"<loc>http://www.example.com/</loc>"#,
//!         r#"<lastmod>2005-01-01</lastmod>"#,
//!         r#"<changefreq>monthly</changefreq>"#,
//!         r#"<priority>0.8</priority>"#,
//!         r#"</url>"#,
//!         r#"</urlset>"#
//!     )
//! );
//! #     Ok(())
//! # }
//! ```
//!
//! ## Example: `SitemapIndexWriter`
//!
//! The following example is a sitemap index containing only one URL specified by `&str`.
//!
//! ```rust
//! use sitemap_xml::writer::{SitemapIndexWriter, Sitemap};
//! use std::io::Cursor;
//!
//! # fn main() -> anyhow::Result<()> {
//! let mut writer = SitemapIndexWriter::start(Cursor::new(Vec::new()))?;
//! writer.write("http://www.example.com/sitemap1.xml.gz")?;
//! writer.end()?;
//!
//! assert_eq!(
//!     String::from_utf8(writer.into_inner().into_inner())?,
//!     concat!(
//!         r#"<?xml version="1.0" encoding="UTF-8"?>"#,
//!         r#"<sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#,
//!         r#"<sitemap>"#,
//!         r#"<loc>http://www.example.com/sitemap1.xml.gz</loc>"#,
//!         r#"</sitemap>"#,
//!         r#"</sitemapindex>"#
//!     )
//! );
//! #    Ok(())
//! # }
//! ```
//!
//! The following example is a sitemap that uses all the optional tags. It also
//! includes an example using non-string types.
//!
//! ```rust
//! use sitemap_xml::writer::{SitemapIndexWriter, Sitemap};
//! use std::io::Cursor;
//!
//! # fn main() -> anyhow::Result<()> {
//! let mut writer = SitemapIndexWriter::start(Cursor::new(Vec::new()))?;
//! writer.write(
//!     Sitemap::loc("http://www.example.com/sitemap1.xml.gz")?
//!         .lastmod("2004-10-01T18:23:17+00:00")?
//! )?;
//! writer.write(
//!     // <https://crates.io/crates/url> support
//!     // If you want to ensure that the URL is Valid, use `::url::Url`.
//!     // If you use &str, the URL is assumed to be valid and only the length
//!     // check and XML entity escaping are performed.
//!     Sitemap::loc(::url::Url::parse("http://www.example.com/sitemap2.xml.gz")?)?
//!         // <https://crates.io/crates/time> support (`time::Date`, `time::DateTime`)
//!         .lastmod(::time::macros::date!(2005-01-01))?,
//! )?;
//! writer.end()?;
//!
//! assert_eq!(
//!     String::from_utf8(writer.into_inner().into_inner())?,
//!     concat!(
//!         r#"<?xml version="1.0" encoding="UTF-8"?>"#,
//!         r#"<sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#,
//!         r#"<sitemap>"#,
//!         r#"<loc>http://www.example.com/sitemap1.xml.gz</loc>"#,
//!         r#"<lastmod>2004-10-01T18:23:17+00:00</lastmod>"#,
//!         r#"</sitemap>"#,
//!         r#"<sitemap>"#,
//!         r#"<loc>http://www.example.com/sitemap2.xml.gz</loc>"#,
//!         r#"<lastmod>2005-01-01</lastmod>"#,
//!         r#"</sitemap>"#,
//!         r#"</sitemapindex>"#
//!     )
//! );
//! #     Ok(())
//! # }

mod changefreq;
mod lastmod;
mod loc;
mod priority;
mod sitemap;
mod sitemap_index_writer;
mod sitemap_writer;
mod sitemap_xml_writer;
mod url;

pub use self::changefreq::Changefreq;
pub use self::lastmod::Lastmod;
pub use self::loc::Loc;
pub use self::priority::Priority;
pub use self::sitemap::Sitemap;
pub use self::sitemap_index_writer::SitemapIndexWriter;
pub use self::sitemap_writer::SitemapWriter;
pub use self::url::Url;
