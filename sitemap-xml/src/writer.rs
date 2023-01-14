mod changefreq;
mod lastmod;
mod loc;
mod priority;
mod sitemap_writer;
mod url;

pub use self::changefreq::Changefreq;
pub use self::lastmod::Lastmod;
pub use self::loc::Loc;
pub use self::priority::Priority;
pub use self::sitemap_writer::SitemapWriter;
pub use self::url::Url;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid changefreq")]
    InvalidChangefreq,
    #[error("invalid lastmod")]
    InvalidLastmod,
    #[error("invalid loc")]
    InvalidLoc,
    #[error("invalid priority")]
    InvalidPriority,
    #[error("invalid url")]
    InvalidUrl,
    #[error("io")]
    Io(#[from] std::io::Error),
    #[error("max byte length is 50 MB (52,428,800 bytes)")]
    MaxByteLength,
    #[error("max number of urls is 50,000")]
    MaxNumberOfUrls,
}

type Result<T, E = Error> = std::result::Result<T, E>;
