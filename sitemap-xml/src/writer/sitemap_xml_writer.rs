use std::{borrow::Cow, io::Write};

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("io")]
    Io(#[from] std::io::Error),
    #[error("max byte length")]
    MaxByteLength,
}

pub(crate) type Result<T, E = Error> = std::result::Result<T, E>;

pub(crate) struct SitemapXmlWriter<W: Write> {
    write: W,
    byte_length: usize,
    indent_level: usize,
    pretty: bool,
}

impl<W: Write> SitemapXmlWriter<W> {
    const MAX_BYTE_LENGTH: usize = 52_428_800;

    pub(crate) fn new(write: W, pretty: bool) -> Self {
        Self {
            write,
            byte_length: 0,
            indent_level: 0,
            pretty,
        }
    }

    pub(crate) fn into_inner(self) -> W {
        self.write
    }

    pub(crate) fn declaration(&mut self) -> Result<()> {
        self.write(br#"<?xml version="1.0" encoding="UTF-8"?>"#)
    }

    pub(crate) fn element(&mut self, name: &[u8], content: &str) -> Result<()> {
        self.indent()?;
        self.start_tag_without_indent(name)?;
        self.write(entity_escape(content).as_bytes())?;
        self.end_tag_without_indent(name)?;
        Ok(())
    }

    pub(crate) fn end_tag(&mut self, name: &[u8]) -> Result<()> {
        self.indent_level -= 1;
        self.indent()?;
        self.end_tag_without_indent(name)?;
        Ok(())
    }

    pub(crate) fn start_tag(&mut self, name: &[u8]) -> Result<()> {
        self.indent()?;
        self.start_tag_without_indent(name)?;
        self.indent_level += 1;
        Ok(())
    }

    pub(crate) fn start_tag_with_default_ns(&mut self, name: &[u8]) -> Result<()> {
        self.indent()?;
        self.write(b"<")?;
        self.write(name)?;
        self.write(br#" xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#)?;
        self.indent_level += 1;
        Ok(())
    }

    fn end_tag_without_indent(&mut self, name: &[u8]) -> Result<()> {
        self.write(b"</")?;
        self.write(name)?;
        self.write(b">")?;
        Ok(())
    }

    fn indent(&mut self) -> Result<()> {
        if self.pretty {
            self.write(b"\n")?;
            for _ in 0..self.indent_level {
                self.write(b"  ")?;
            }
        }
        Ok(())
    }

    fn start_tag_without_indent(&mut self, name: &[u8]) -> Result<()> {
        self.write(b"<")?;
        self.write(name)?;
        self.write(b">")?;
        Ok(())
    }

    fn write(&mut self, buf: &[u8]) -> Result<()> {
        let l = buf.len();
        if self.byte_length + l > Self::MAX_BYTE_LENGTH {
            return Err(Error::MaxByteLength);
        }
        self.byte_length += l;

        self.write.write_all(buf)?;
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
        escaped.extend_from_slice(escape(bytes[index]));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(entity_escape("abc"), "abc");
        assert_eq!(entity_escape("\"&'<>"), "&quot;&amp;&apos;&lt;&gt;");
        assert_eq!(
            entity_escape(r#"<h1 class="title">"#),
            "&lt;h1 class=&quot;title&quot;&gt;"
        );
    }
}
