use std::fmt::Display;

use crate::direction::Direction;

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("invalid char: {0}")]
    InvalidChar(char),
    #[error("invalid pipe type: {0}")]
    InvalidPipeType(u8),
    #[error("invalid pipe direction: {0}")]
    InvalidPipeDirection(u8),
    #[error("reserved bits are used")]
    ReservedBitsUsed(u8),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Pipe(u8);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PipeType {
    I,
    L,
    T,
}

impl From<Pipe> for u8 {
    fn from(pipe: Pipe) -> Self {
        pipe.0
    }
}

impl TryFrom<u8> for Pipe {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let dir = value & 0x03;
        let typ = (value & 0x0C) >> 2;
        let reserved = value & 0xF0;
        if reserved != 0 {
            return Err(Self::Error::ReservedBitsUsed(value));
        }
        match typ {
            0b00 => Err(Self::Error::InvalidPipeType(value)),
            0b01 => match dir {
                0b00 | 0b01 => Ok(Self(value)),
                _ => Err(Self::Error::InvalidPipeDirection(value)),
            },
            0b10 => Ok(Self(value)),
            0b11 => Ok(Self(value)),
            _ => unreachable!(),
        }
    }
}

impl From<Pipe> for char {
    fn from(pipe: Pipe) -> Self {
        match pipe.typ() {
            PipeType::I => match pipe.dir() {
                Direction::T | Direction::B => '│',
                Direction::L | Direction::R => '─',
            },
            PipeType::L => match pipe.dir() {
                Direction::T => '└',
                Direction::R => '┌',
                Direction::B => '┐',
                Direction::L => '┘',
            },
            PipeType::T => match pipe.dir() {
                Direction::T => '┬',
                Direction::R => '┤',
                Direction::B => '┴',
                Direction::L => '├',
            },
        }
    }
}

impl TryFrom<char> for Pipe {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(Self(match value {
            '│' => 0b00000100,
            '─' => 0b00000101,
            '└' => 0b00001000,
            '┌' => 0b00001001,
            '┐' => 0b00001010,
            '┘' => 0b00001011,
            '┬' => 0b00001100,
            '┤' => 0b00001101,
            '┴' => 0b00001110,
            '├' => 0b00001111,
            _ => return Err(Self::Error::InvalidChar(value)),
        }))
    }
}

impl Pipe {
    pub fn is_open(&self, dir: Direction) -> bool {
        let open: u8 = match self.typ() {
            PipeType::I => match self.dir() {
                Direction::T | Direction::B => 0b1010,
                Direction::L | Direction::R => 0b0101,
            },
            PipeType::L => match self.dir() {
                Direction::T => 0b1100,
                Direction::R => 0b0110,
                Direction::B => 0b0011,
                Direction::L => 0b1001,
            },
            PipeType::T => match self.dir() {
                Direction::T => 0b0111,
                Direction::R => 0b1011,
                Direction::B => 0b1101,
                Direction::L => 0b1110,
            },
        };
        match dir {
            Direction::T => (open & 0b1000) != 0,
            Direction::R => (open & 0b0100) != 0,
            Direction::B => (open & 0b0010) != 0,
            Direction::L => (open & 0b0001) != 0,
        }
    }

    pub fn rotate(&self) -> Pipe {
        let dir = ((self.0 & 0x03) + 1) & 0x03;
        Self(
            self.0 & 0xFC
                | match self.typ() {
                    PipeType::I => dir & 0x01,
                    PipeType::L => dir,
                    PipeType::T => dir,
                },
        )
    }

    fn dir(&self) -> Direction {
        let dir = self.0 & 0x03;
        match dir {
            0b00 => Direction::T,
            0b01 => Direction::R,
            0b10 => Direction::B,
            0b11 => Direction::L,
            _ => unreachable!(),
        }
    }

    fn typ(&self) -> PipeType {
        let typ = (self.0 & 0x0C) >> 2;
        match typ {
            0b01 => PipeType::I,
            0b10 => PipeType::L,
            0b11 => PipeType::T,
            _ => unreachable!(),
        }
    }
}

impl Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::bool_assert_comparison)]
    #[test]
    fn from_pipe_for_u8_test() -> anyhow::Result<()> {
        assert_eq!(0b00000100, u8::from(Pipe::try_from('│')?));
        assert_eq!(0b00000101, u8::from(Pipe::try_from('─')?));
        assert_eq!(0b00001000, u8::from(Pipe::try_from('└')?));
        assert_eq!(0b00001001, u8::from(Pipe::try_from('┌')?));
        assert_eq!(0b00001010, u8::from(Pipe::try_from('┐')?));
        assert_eq!(0b00001011, u8::from(Pipe::try_from('┘')?));
        assert_eq!(0b00001100, u8::from(Pipe::try_from('┬')?));
        assert_eq!(0b00001101, u8::from(Pipe::try_from('┤')?));
        assert_eq!(0b00001110, u8::from(Pipe::try_from('┴')?));
        assert_eq!(0b00001111, u8::from(Pipe::try_from('├')?));
        Ok(())
    }

    #[allow(clippy::bool_assert_comparison)]
    #[test]
    fn try_from_u8_test() -> anyhow::Result<()> {
        assert_eq!(Pipe::try_from(0b00000000).is_err(), true);
        assert_eq!(Pipe::try_from(0b00000100)?, Pipe::try_from('│')?);
        assert_eq!(Pipe::try_from(0b00000101)?, Pipe::try_from('─')?);
        assert_eq!(Pipe::try_from(0b00000110).is_err(), true);
        assert_eq!(Pipe::try_from(0b00000111).is_err(), true);
        assert_eq!(Pipe::try_from(0b00001000)?, Pipe::try_from('└')?);
        assert_eq!(Pipe::try_from(0b00001001)?, Pipe::try_from('┌')?);
        assert_eq!(Pipe::try_from(0b00001010)?, Pipe::try_from('┐')?);
        assert_eq!(Pipe::try_from(0b00001011)?, Pipe::try_from('┘')?);
        assert_eq!(Pipe::try_from(0b00001100)?, Pipe::try_from('┬')?);
        assert_eq!(Pipe::try_from(0b00001101)?, Pipe::try_from('┤')?);
        assert_eq!(Pipe::try_from(0b00001110)?, Pipe::try_from('┴')?);
        assert_eq!(Pipe::try_from(0b00001111)?, Pipe::try_from('├')?);
        assert_eq!(Pipe::try_from(0b00010000).is_err(), true);
        Ok(())
    }

    #[test]
    fn is_open_test() -> anyhow::Result<()> {
        assert!(Pipe::try_from('│')?.is_open(Direction::B));
        assert!(!Pipe::try_from('│')?.is_open(Direction::L));
        assert!(!Pipe::try_from('│')?.is_open(Direction::R));
        assert!(Pipe::try_from('│')?.is_open(Direction::T));
        // ...
        Ok(())
    }

    #[test]
    fn rotate_test() -> anyhow::Result<()> {
        assert_eq!(Pipe::try_from('│')?.rotate(), Pipe::try_from('─')?);
        assert_eq!(Pipe::try_from('─')?.rotate(), Pipe::try_from('│')?);
        assert_eq!(Pipe::try_from('└')?.rotate(), Pipe::try_from('┌')?);
        assert_eq!(Pipe::try_from('┌')?.rotate(), Pipe::try_from('┐')?);
        assert_eq!(Pipe::try_from('┐')?.rotate(), Pipe::try_from('┘')?);
        assert_eq!(Pipe::try_from('┘')?.rotate(), Pipe::try_from('└')?);
        assert_eq!(Pipe::try_from('┬')?.rotate(), Pipe::try_from('┤')?);
        assert_eq!(Pipe::try_from('┤')?.rotate(), Pipe::try_from('┴')?);
        assert_eq!(Pipe::try_from('┴')?.rotate(), Pipe::try_from('├')?);
        assert_eq!(Pipe::try_from('├')?.rotate(), Pipe::try_from('┬')?);
        Ok(())
    }

    #[test]
    fn to_string_test() -> anyhow::Result<()> {
        assert_eq!(Pipe::try_from('│')?.to_string(), "│");
        assert_eq!(Pipe::try_from('─')?.to_string(), "─");
        assert_eq!(Pipe::try_from('└')?.to_string(), "└");
        assert_eq!(Pipe::try_from('┌')?.to_string(), "┌");
        assert_eq!(Pipe::try_from('┐')?.to_string(), "┐");
        assert_eq!(Pipe::try_from('┘')?.to_string(), "┘");
        assert_eq!(Pipe::try_from('┬')?.to_string(), "┬");
        assert_eq!(Pipe::try_from('┤')?.to_string(), "┤");
        assert_eq!(Pipe::try_from('┴')?.to_string(), "┴");
        assert_eq!(Pipe::try_from('├')?.to_string(), "├");
        Ok(())
    }
}
