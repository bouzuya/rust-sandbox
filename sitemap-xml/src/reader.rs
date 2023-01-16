use std::io::Read;

pub struct SitemapReader<R: Read> {
    inner: R,
}

impl<R: Read> SitemapReader<R> {
    pub fn new(inner: R) -> Self {
        Self { inner }
    }
}
