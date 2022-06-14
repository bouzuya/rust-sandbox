use crate::{point::Point, Pipe};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("too few pipes")]
    TooFewPipes,
    #[error("too many pipes")]
    TooManyPipes,
}

pub struct Area {
    width: u8,
    height: u8,
    pipes: Vec<Pipe>,
}

impl Area {
    pub fn new(width: u8, height: u8, pipes: Vec<Pipe>) -> Result<Self, Error> {
        let length = u16::try_from(pipes.len()).map_err(|_| Error::TooManyPipes)?;
        match length.cmp(&(u16::from(width) * u16::from(height))) {
            std::cmp::Ordering::Less => Err(Error::TooFewPipes),
            std::cmp::Ordering::Greater => Err(Error::TooManyPipes),
            std::cmp::Ordering::Equal => Ok(()),
        }?;
        Ok(Self {
            width,
            height,
            pipes,
        })
    }

    pub fn print(&self) {
        let w = usize::from(self.width);
        let h = usize::from(self.height);
        for i in 0..h {
            for j in 0..w {
                print!("{}", self.pipes[i * w + j]);
            }
            println!();
        }
    }

    pub fn pipe(&self, point: Point) -> Pipe {
        let x = usize::from(point.x());
        let y = usize::from(point.y());
        let w = usize::from(self.width);
        self.pipes[y * w + x]
    }

    pub fn rotate(&mut self, point: Point) {
        let x = usize::from(point.x());
        let y = usize::from(point.y());
        let w = usize::from(self.width);
        self.pipes[y * w + x] = self.pipes[y * w + x].rotate();
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
}
