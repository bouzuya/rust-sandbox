use std::fmt::Display;

enum Block {
    I(u8),
    L(u8),
    T(u8),
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

fn main() {
    let width = 2;
    let height = 2;
    let board = vec![Block::I(1), Block::L(2), Block::T(0), Block::L(0)];
    for i in 0..height {
        for j in 0..width {
            print!("{}", board[i * width + j]);
        }
        println!();
    }
}
