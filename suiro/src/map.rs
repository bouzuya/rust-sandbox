use std::{collections::VecDeque, str::FromStr};

use crate::{direction::Direction, point::Point, size::Size, Pipe};

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("too few pipes")]
    TooFewPipes,
    #[error("invalid format")]
    InvalidFormat,
    #[error("invalid pipe")]
    InvalidPipe(#[from] crate::pipe::Error),
    #[error("invalid size")]
    InvalidSize(#[from] crate::size::Error),
    #[error("too many pipes")]
    TooManyPipes,
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Eq, PartialEq)]
pub struct Map {
    size: Size,
    pipes: Vec<Pipe>,
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes =
            base32::decode(base32::Alphabet::Crockford, s).ok_or(Self::Err::InvalidFormat)?;
        if bytes.is_empty() {
            return Err(Self::Err::InvalidFormat);
        }
        let size = Size::from(bytes[0]);
        let pipes = bytes
            .iter()
            .skip(1)
            .copied()
            .map(|b| Pipe::try_from(b).map_err(Self::Err::from))
            .collect::<Result<Vec<Pipe>>>()?;
        Map::new(size, pipes)
    }
}

impl Map {
    pub fn gen(size: Size) -> Result<Self> {
        // TODO:
        let pipes = (0..u16::from(size.width()) * u16::from(size.height()))
            .into_iter()
            .map(|_| Pipe::try_from('─').expect("pipe broken"))
            .collect::<Vec<Pipe>>();
        Ok(Self { size, pipes })
    }

    pub fn new(size: Size, pipes: Vec<Pipe>) -> Result<Self> {
        let length = u16::try_from(pipes.len()).map_err(|_| Error::TooManyPipes)?;
        match length.cmp(&(u16::from(size.width()) * u16::from(size.height()))) {
            std::cmp::Ordering::Less => Err(Error::TooFewPipes),
            std::cmp::Ordering::Greater => Err(Error::TooManyPipes),
            std::cmp::Ordering::Equal => Ok(()),
        }?;
        Ok(Self { size, pipes })
    }

    pub fn height(&self) -> u8 {
        self.size.height()
    }

    pub fn pipe(&self, point: Point) -> Pipe {
        let x = usize::from(point.x());
        let y = usize::from(point.y());
        let w = usize::from(self.width());
        self.pipes[y * w + x]
    }

    pub fn rotate(&mut self, point: Point) {
        let x = usize::from(point.x());
        let y = usize::from(point.y());
        let w = usize::from(self.width());
        self.pipes[y * w + x] = self.pipes[y * w + x].rotate();
    }

    pub fn test(&self) -> (bool, Vec<bool>, Vec<bool>) {
        let w = usize::from(self.width());
        let h = usize::from(self.height());
        let mut ok = true;
        let mut ng = vec![false; w * h];
        let mut checked = vec![None; w * h];
        if self.pipes.is_empty() {
            return (ok, ng, vec![]);
        }
        if !self.pipes[0].is_open(Direction::L) {
            ok = false;
            ng[0] = true;
            return (
                ok,
                ng,
                checked
                    .into_iter()
                    .map(|i| i.unwrap_or_default())
                    .collect::<Vec<bool>>(),
            );
        }
        let mut deque = VecDeque::new();
        deque.push_back((0, 0));
        checked[0] = Some(true);
        while let Some((x, y)) = deque.pop_front() {
            let p = self.pipes[y * w + x];
            let dir = vec![Direction::T, Direction::B, Direction::L, Direction::R];
            for d in dir {
                if p.is_open(d) {
                    match d {
                        Direction::T => {
                            if y != 0 && self.pipes[(y - 1) * w + x].is_open(Direction::B) {
                                if checked[(y - 1) * w + x].is_none() {
                                    checked[(y - 1) * w + x] = Some(true);
                                    deque.push_back((x, y - 1));
                                }
                            } else {
                                ok = false;
                                ng[y * w + x] = true;
                            }
                        }
                        Direction::B => {
                            if y + 1 != h && self.pipes[(y + 1) * w + x].is_open(Direction::T) {
                                if checked[(y + 1) * w + x].is_none() {
                                    checked[(y + 1) * w + x] = Some(true);
                                    deque.push_back((x, y + 1));
                                }
                            } else {
                                ok = false;
                                ng[y * w + x] = true;
                            }
                        }
                        Direction::L => {
                            if x != 0 && self.pipes[y * w + x - 1].is_open(Direction::R) {
                                if checked[y * w + x - 1].is_none() {
                                    checked[y * w + x - 1] = Some(true);
                                    deque.push_back((x - 1, y));
                                }
                            } else if !(x == 0 && y == 0) {
                                ok = false;
                                ng[y * w + x] = true;
                            }
                        }
                        Direction::R => {
                            if x + 1 != w && self.pipes[y * w + x + 1].is_open(Direction::L) {
                                if checked[y * w + x + 1].is_none() {
                                    checked[y * w + x + 1] = Some(true);
                                    deque.push_back((x + 1, y));
                                }
                            } else if !(x + 1 == w && y + 1 == h) {
                                ok = false;
                                ng[y * w + x] = true;
                            }
                        }
                    }
                }
            }
        }
        (
            ok,
            ng,
            checked
                .into_iter()
                .map(|i| i.unwrap_or_default())
                .collect::<Vec<bool>>(),
        )
    }

    pub fn width(&self) -> u8 {
        self.size.width()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_test() -> anyhow::Result<()> {
        let pipe_i = Pipe::try_from('│')?;
        let pipe_l = Pipe::try_from('└')?;

        // │
        let s = base32::encode(base32::Alphabet::Crockford, &[0b00000000, 0b00000100]);
        assert_eq!(s, "0020");
        assert_eq!(
            Map::from_str(s.as_str()),
            Map::new(Size::new(1, 1)?, vec![pipe_i])
        );

        // ││
        let s = base32::encode(
            base32::Alphabet::Crockford,
            &[0b00010000, 0b00000100, 0b00000100],
        );
        assert_eq!(s, "20208");
        assert_eq!(
            Map::from_str(s.as_str()),
            Map::new(Size::new(2, 1)?, vec![pipe_i, pipe_i])
        );

        // │└
        // │└
        let s = base32::encode(
            base32::Alphabet::Crockford,
            &[0b00010001, 0b00000100, 0b00001000, 0b00000100, 0b00001000],
        );
        assert_eq!(s, "2420G108");
        assert_eq!(
            Map::from_str(s.as_str()),
            Map::new(Size::new(2, 2)?, vec![pipe_i, pipe_l, pipe_i, pipe_l])
        );

        Ok(())
    }

    #[test]
    fn new_test() -> anyhow::Result<()> {
        let pipe_i = Pipe::try_from('│')?;
        let pipe_l = Pipe::try_from('└')?;
        let pipe_t = Pipe::try_from('┬')?;

        let area = Map::new(Size::new(2, 2)?, vec![pipe_i, pipe_l, pipe_t, pipe_l])?;
        assert_eq!(area.pipe(Point::new(0, 0)), pipe_i);
        assert_eq!(area.pipe(Point::new(1, 0)), pipe_l);
        assert_eq!(area.pipe(Point::new(0, 1)), pipe_t);
        assert_eq!(area.pipe(Point::new(1, 1)), pipe_l);
        Ok(())
    }

    #[test]
    fn height_test() -> anyhow::Result<()> {
        let pipe_i = Pipe::try_from('│')?;
        let pipe_l = Pipe::try_from('└')?;

        let area = Map::new(Size::new(1, 2)?, vec![pipe_i, pipe_l])?;
        assert_eq!(area.height(), 2);
        Ok(())
    }

    #[test]
    fn rotate_test() -> anyhow::Result<()> {
        let pipe_i = Pipe::try_from('│')?;
        let pipe_l = Pipe::try_from('└')?;
        let pipe_t = Pipe::try_from('┬')?;

        let mut area = Map::new(Size::new(2, 2)?, vec![pipe_i, pipe_l, pipe_t, pipe_l])?;
        assert_eq!(area.pipe(Point::new(0, 0)), pipe_i);
        assert_eq!(area.pipe(Point::new(1, 0)), pipe_l);
        assert_eq!(area.pipe(Point::new(0, 1)), pipe_t);
        assert_eq!(area.pipe(Point::new(1, 1)), pipe_l);
        area.rotate(Point::new(0, 1));
        assert_eq!(area.pipe(Point::new(0, 1)), pipe_t.rotate());
        area.rotate(Point::new(0, 0));
        assert_eq!(area.pipe(Point::new(0, 0)), pipe_i.rotate());
        Ok(())
    }

    #[test]
    fn test_test() -> anyhow::Result<()> {
        let pipe_i = Pipe::try_from('│')?;
        let pipe_l = Pipe::try_from('└')?;
        let pipe_t = Pipe::try_from('┬')?;

        let area = Map::new(
            Size::new(2, 2)?,
            vec![pipe_i.rotate(), pipe_l, pipe_t, pipe_l],
        )?;
        let (ok, ng, flow) = area.test();
        assert!(!ok);
        assert_eq!(ng, vec![true, false, false, false]);
        assert_eq!(flow, vec![true, false, false, false]);

        let area = Map::new(
            Size::new(2, 2)?,
            vec![pipe_i.rotate(), pipe_l.rotate().rotate(), pipe_t, pipe_l],
        )?;
        let (ok, ng, flow) = area.test();
        assert!(ok);
        assert_eq!(ng, vec![false, false, false, false]);
        assert_eq!(flow, vec![true, true, false, true]);
        Ok(())
    }

    #[test]
    fn width_test() -> anyhow::Result<()> {
        let pipe_i = Pipe::try_from('│')?;
        let pipe_l = Pipe::try_from('└')?;
        let area = Map::new(Size::new(1, 2)?, vec![pipe_i, pipe_l])?;
        assert_eq!(area.width(), 1);
        Ok(())
    }
}
