#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Point {
    x: u8,
    y: u8,
}

impl Point {
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> u8 {
        self.x
    }

    pub fn y(&self) -> u8 {
        self.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let point = Point::new(0, 1);
        assert_eq!(point.x(), 0);
        assert_eq!(point.y(), 1);
    }
}
