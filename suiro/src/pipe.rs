use std::fmt::Display;

use anyhow::bail;

use crate::direction::Direction;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Pipe {
    I(u8),
    L(u8),
    T(u8),
}

impl TryFrom<u8> for Pipe {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let dir = value & 0x03;
        let typ = (value & 0x0C) >> 2;
        let reserved = value & 0xF0;
        if reserved != 0 {
            bail!("invalid byte");
        }
        match typ {
            0b00 => bail!("invalid byte"),
            0b01 => match dir {
                d @ (0b00 | 0b01) => Ok(Pipe::I(d)),
                _ => bail!("invalid byte"),
            },
            0b10 => Ok(Pipe::L(dir)),
            0b11 => Ok(Pipe::T(dir)),
            _ => unreachable!(),
        }
    }
}

impl Pipe {
    pub fn is_open(&self, dir: Direction) -> bool {
        match dir {
            Direction::T => match self {
                Pipe::I(d) => match d {
                    0 => true,
                    1 => false,
                    _ => unreachable!(),
                },
                Pipe::L(d) => match d {
                    0 => true,
                    1 => false,
                    2 => false,
                    3 => true,
                    _ => unreachable!(),
                },
                Pipe::T(d) => match d {
                    0 => false,
                    1 => true,
                    2 => true,
                    3 => true,
                    _ => unreachable!(),
                },
            },
            Direction::B => match self {
                Pipe::I(d) => match d {
                    0 => true,
                    1 => false,
                    _ => unreachable!(),
                },
                Pipe::L(d) => match d {
                    0 => false,
                    1 => true,
                    2 => true,
                    3 => false,
                    _ => unreachable!(),
                },
                Pipe::T(d) => match d {
                    0 => true,
                    1 => true,
                    2 => false,
                    3 => true,
                    _ => unreachable!(),
                },
            },
            Direction::L => match self {
                Pipe::I(d) => match d {
                    0 => false,
                    1 => true,
                    _ => unreachable!(),
                },
                Pipe::L(d) => match d {
                    0 => false,
                    1 => false,
                    2 => true,
                    3 => true,
                    _ => unreachable!(),
                },
                Pipe::T(d) => match d {
                    0 => true,
                    1 => true,
                    2 => true,
                    3 => false,
                    _ => unreachable!(),
                },
            },
            Direction::R => match self {
                Pipe::I(d) => match d {
                    0 => false,
                    1 => true,
                    _ => unreachable!(),
                },
                Pipe::L(d) => match d {
                    0 => true,
                    1 => true,
                    2 => false,
                    3 => false,
                    _ => unreachable!(),
                },
                Pipe::T(d) => match d {
                    0 => true,
                    1 => false,
                    2 => true,
                    3 => true,
                    _ => unreachable!(),
                },
            },
        }
    }

    pub fn rotate(&self) -> Pipe {
        match self {
            Pipe::I(d) => Pipe::I((d + 1) % 2),
            Pipe::L(d) => Pipe::L((d + 1) % 4),
            Pipe::T(d) => Pipe::T((d + 1) % 4),
        }
    }
}

impl Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Pipe::I(d) => match d {
                0 => '│',
                1 => '─',
                _ => unreachable!(),
            },
            Pipe::L(d) => match d {
                0 => '└',
                1 => '┌',
                2 => '┐',
                3 => '┘',
                _ => unreachable!(),
            },
            Pipe::T(d) => match d {
                0 => '┬',
                1 => '┤',
                2 => '┴',
                3 => '├',
                _ => unreachable!(),
            },
        };
        write!(f, "{}", c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::bool_assert_comparison)]
    #[test]
    fn try_from_u8_test() -> anyhow::Result<()> {
        assert_eq!(Pipe::try_from(0b00000000).is_err(), true);
        assert_eq!(Pipe::try_from(0b00000100)?, Pipe::I(0));
        assert_eq!(Pipe::try_from(0b00000101)?, Pipe::I(1));
        assert_eq!(Pipe::try_from(0b00000110).is_err(), true);
        assert_eq!(Pipe::try_from(0b00000111).is_err(), true);
        assert_eq!(Pipe::try_from(0b00001000)?, Pipe::L(0));
        assert_eq!(Pipe::try_from(0b00001001)?, Pipe::L(1));
        assert_eq!(Pipe::try_from(0b00001010)?, Pipe::L(2));
        assert_eq!(Pipe::try_from(0b00001011)?, Pipe::L(3));
        assert_eq!(Pipe::try_from(0b00001100)?, Pipe::T(0));
        assert_eq!(Pipe::try_from(0b00001101)?, Pipe::T(1));
        assert_eq!(Pipe::try_from(0b00001110)?, Pipe::T(2));
        assert_eq!(Pipe::try_from(0b00001111)?, Pipe::T(3));
        assert_eq!(Pipe::try_from(0b00010000).is_err(), true);
        Ok(())
    }

    #[test]
    fn is_open_test() {
        assert!(Pipe::I(0).is_open(Direction::B));
        assert!(!Pipe::I(0).is_open(Direction::L));
        assert!(!Pipe::I(0).is_open(Direction::R));
        assert!(Pipe::I(0).is_open(Direction::T));
        // ...
    }

    #[test]
    fn rotate_test() {
        assert_eq!(Pipe::I(0).rotate(), Pipe::I(1));
        assert_eq!(Pipe::I(1).rotate(), Pipe::I(0));
        assert_eq!(Pipe::L(0).rotate(), Pipe::L(1));
        assert_eq!(Pipe::L(1).rotate(), Pipe::L(2));
        assert_eq!(Pipe::L(2).rotate(), Pipe::L(3));
        assert_eq!(Pipe::L(3).rotate(), Pipe::L(0));
        assert_eq!(Pipe::T(0).rotate(), Pipe::T(1));
        assert_eq!(Pipe::T(1).rotate(), Pipe::T(2));
        assert_eq!(Pipe::T(2).rotate(), Pipe::T(3));
        assert_eq!(Pipe::T(3).rotate(), Pipe::T(0));
    }

    #[test]
    fn to_string_test() {
        assert_eq!(Pipe::I(0).to_string(), "│");
        assert_eq!(Pipe::I(0).to_string(), "│");
        assert_eq!(Pipe::I(1).to_string(), "─");
        assert_eq!(Pipe::L(0).to_string(), "└");
        assert_eq!(Pipe::L(1).to_string(), "┌");
        assert_eq!(Pipe::L(2).to_string(), "┐");
        assert_eq!(Pipe::L(3).to_string(), "┘");
        assert_eq!(Pipe::T(0).to_string(), "┬");
        assert_eq!(Pipe::T(1).to_string(), "┤");
        assert_eq!(Pipe::T(2).to_string(), "┴");
        assert_eq!(Pipe::T(3).to_string(), "├");
    }
}
