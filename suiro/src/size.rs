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

impl From<Size> for u8 {
    fn from(value: Size) -> Self {
        value.0
    }
}

impl From<u8> for Size {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

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
    fn u8_conversion_test() -> anyhow::Result<()> {
        assert_eq!(u8::from(Size::new(1, 1)?), 0b00000000);
        assert_eq!(u8::from(Size::new(2, 1)?), 0b00010000);
        assert_eq!(u8::from(Size::new(1, 2)?), 0b00000001);
        assert_eq!(u8::from(Size::new(16, 16)?), 0b11111111);
        for byte in std::u8::MIN..=std::u8::MAX {
            assert_eq!(u8::from(Size::from(byte)), byte);
        }
        Ok(())
    }

    #[test]
    fn test() -> anyhow::Result<()> {
        assert_eq!(Size::new(0, 10), Err(Error::TooShortWidth));
        assert_eq!(Size::new(17, 10), Err(Error::TooLongWidth));
        assert_eq!(Size::new(10, 0), Err(Error::TooShortHeight));
        assert_eq!(Size::new(10, 17), Err(Error::TooLongHeight));
        let size = Size::new(6, 8)?;
        assert_eq!(size.width(), 6);
        assert_eq!(size.height(), 8);
        Ok(())
    }
}
