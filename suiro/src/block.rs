use std::fmt::Display;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Block {
    I(u8),
    L(u8),
    T(u8),
}

impl Block {
    pub fn rotate(&self) -> Block {
        match self {
            Block::I(d) => Block::I((d + 1) % 2),
            Block::L(d) => Block::L((d + 1) % 4),
            Block::T(d) => Block::T((d + 1) % 4),
        }
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Block::I(d) => match d {
                0 => '│',
                1 => '─',
                _ => unreachable!(),
            },
            Block::L(d) => match d {
                0 => '└',
                1 => '┌',
                2 => '┐',
                3 => '┘',
                _ => unreachable!(),
            },
            Block::T(d) => match d {
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

    #[test]
    fn rotate_test() {
        assert_eq!(Block::I(0).rotate(), Block::I(1));
        assert_eq!(Block::I(1).rotate(), Block::I(0));
        assert_eq!(Block::L(0).rotate(), Block::L(1));
        assert_eq!(Block::L(1).rotate(), Block::L(2));
        assert_eq!(Block::L(2).rotate(), Block::L(3));
        assert_eq!(Block::L(3).rotate(), Block::L(0));
        assert_eq!(Block::T(0).rotate(), Block::T(1));
        assert_eq!(Block::T(1).rotate(), Block::T(2));
        assert_eq!(Block::T(2).rotate(), Block::T(3));
        assert_eq!(Block::T(3).rotate(), Block::T(0));
    }

    #[test]
    fn to_string_test() {
        assert_eq!(Block::I(0).to_string(), "│");
        assert_eq!(Block::I(0).to_string(), "│");
        assert_eq!(Block::I(1).to_string(), "─");
        assert_eq!(Block::L(0).to_string(), "└");
        assert_eq!(Block::L(1).to_string(), "┌");
        assert_eq!(Block::L(2).to_string(), "┐");
        assert_eq!(Block::L(3).to_string(), "┘");
        assert_eq!(Block::T(0).to_string(), "┬");
        assert_eq!(Block::T(1).to_string(), "┤");
        assert_eq!(Block::T(2).to_string(), "┴");
        assert_eq!(Block::T(3).to_string(), "├");
    }
}
