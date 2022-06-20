#[allow(clippy::enum_variant_names)]
#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("too long height")]
    TooLongHeight,
    #[error("too long width")]
    TooLongWidth,
    #[error("too short height")]
    TooShortHeight,
    #[error("too short width")]
    TooShortWidth,
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Size(u8);

impl Size {
    pub fn new(width: u8, height: u8) -> Result<Self, Error> {
        if height < 1 {
            return Err(Error::TooShortHeight);
        }
        if height > 16 {
            return Err(Error::TooLongHeight);
        }
        if width < 1 {
            return Err(Error::TooShortWidth);
        }
        if width > 16 {
            return Err(Error::TooLongWidth);
        }
        Ok(Self(((width - 1) << 4) | (height - 1)))
    }

    pub fn height(&self) -> u8 {
        (self.0 & 0x0F) + 1
    }

    pub fn width(&self) -> u8 {
        ((self.0 >> 4) & 0x0F) + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        assert_eq!(Size::new(0, 10), Err(Error::TooShortWidth));
        assert_eq!(Size::new(16, 10), Err(Error::TooShortWidth));
        assert_eq!(Size::new(10, 0), Err(Error::TooShortHeight));
        assert_eq!(Size::new(10, 16), Err(Error::TooShortHeight));
        let size = Size::new(6, 8)?;
        assert_eq!(size.width(), 6);
        assert_eq!(size.height(), 8);
        Ok(())
    }
}
