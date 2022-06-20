use std::collections::VecDeque;

use crate::{direction::Direction, point::Point, size::Size, Pipe};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("too few pipes")]
    TooFewPipes,
    #[error("invalid height")]
    InvalidSize(#[from] crate::size::Error),
    #[error("too many pipes")]
    TooManyPipes,
}

pub struct Area {
    size: Size,
    pipes: Vec<Pipe>,
}

impl Area {
    pub fn new(width: u8, height: u8, pipes: Vec<Pipe>) -> Result<Self, Error> {
        let size = Size::new(width, height)?;
        let length = u16::try_from(pipes.len()).map_err(|_| Error::TooManyPipes)?;
        match length.cmp(&(u16::from(width) * u16::from(height))) {
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

    pub fn test(&self) -> (Vec<bool>, Vec<bool>) {
        let w = usize::from(self.width());
        let h = usize::from(self.height());
        let mut ng = vec![false; w * h];
        let mut checked = vec![None; w * h];
        if self.pipes.is_empty() {
            return (ng, vec![]);
        }
        if !self.pipes[0].is_open(Direction::L) {
            ng[0] = true;
            return (
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
                                ng[y * w + x] = true;
                            }
                        }
                    }
                }
            }
        }
        (
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
    fn new_test() -> anyhow::Result<()> {
        let area = Area::new(2, 2, vec![Pipe::I(1), Pipe::L(0), Pipe::T(0), Pipe::L(0)])?;
        assert_eq!(area.pipe(Point::new(0, 0)), Pipe::I(1));
        assert_eq!(area.pipe(Point::new(1, 0)), Pipe::L(0));
        assert_eq!(area.pipe(Point::new(0, 1)), Pipe::T(0));
        assert_eq!(area.pipe(Point::new(1, 1)), Pipe::L(0));
        Ok(())
    }

    #[test]
    fn height_test() -> anyhow::Result<()> {
        let area = Area::new(1, 2, vec![Pipe::I(1), Pipe::L(0)])?;
        assert_eq!(area.height(), 2);
        Ok(())
    }

    #[test]
    fn rotate_test() -> anyhow::Result<()> {
        let mut area = Area::new(2, 2, vec![Pipe::I(1), Pipe::L(0), Pipe::T(0), Pipe::L(0)])?;
        assert_eq!(area.pipe(Point::new(0, 0)), Pipe::I(1));
        assert_eq!(area.pipe(Point::new(1, 0)), Pipe::L(0));
        assert_eq!(area.pipe(Point::new(0, 1)), Pipe::T(0));
        assert_eq!(area.pipe(Point::new(1, 1)), Pipe::L(0));
        area.rotate(Point::new(0, 1));
        assert_eq!(area.pipe(Point::new(0, 1)), Pipe::T(1));
        area.rotate(Point::new(0, 0));
        assert_eq!(area.pipe(Point::new(0, 0)), Pipe::I(0));
        Ok(())
    }

    #[test]
    fn test_test() -> anyhow::Result<()> {
        let area = Area::new(2, 2, vec![Pipe::I(1), Pipe::L(0), Pipe::T(0), Pipe::L(0)])?;
        let (ng, flow) = area.test();
        assert_eq!(ng, vec![true, false, false, false]);
        assert_eq!(flow, vec![true, false, false, false]);

        let area = Area::new(2, 2, vec![Pipe::I(1), Pipe::L(2), Pipe::T(0), Pipe::L(0)])?;
        let (ng, flow) = area.test();
        assert_eq!(ng, vec![false, false, false, false]);
        assert_eq!(flow, vec![true, true, false, true]);
        Ok(())
    }

    #[test]
    fn width_test() -> anyhow::Result<()> {
        let area = Area::new(1, 2, vec![Pipe::I(1), Pipe::L(0)])?;
        assert_eq!(area.width(), 1);
        Ok(())
    }
}
