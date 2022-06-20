use crate::{point::Point, size::Size};

#[derive(Clone, Copy, Debug)]
pub struct Cursor {
    size: Size,
    pub x: u8,
    pub y: u8,
}

impl Cursor {
    pub fn new(size: Size, x: u8, y: u8) -> Self {
        Self { size, x, y }
    }

    pub fn down(&mut self) {
        if self.y < self.size.height() - 1 {
            self.y += 1
        }
    }

    pub fn left(&mut self) {
        if self.x > 0 {
            self.x -= 1
        }
    }

    pub fn right(&mut self) {
        if self.x < self.size.width() - 1 {
            self.x += 1
        }
    }

    pub fn up(&mut self) {
        if self.y > 0 {
            self.y -= 1
        }
    }
}

impl From<Cursor> for Point {
    fn from(cursor: Cursor) -> Self {
        Point::new(cursor.x, cursor.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let size = Size::new(3, 3)?;
        let mut cursor = Cursor::new(size, 0, 0);

        cursor.down();
        assert_eq!(cursor.y, 1);
        cursor.down();
        assert_eq!(cursor.y, 2);
        cursor.down();
        assert_eq!(cursor.y, 2);

        cursor.right();
        assert_eq!(cursor.x, 1);
        cursor.right();
        assert_eq!(cursor.x, 2);
        cursor.right();
        assert_eq!(cursor.x, 2);

        cursor.left();
        assert_eq!(cursor.x, 1);
        cursor.left();
        assert_eq!(cursor.x, 0);
        cursor.left();
        assert_eq!(cursor.x, 0);

        cursor.up();
        assert_eq!(cursor.y, 1);
        cursor.up();
        assert_eq!(cursor.y, 0);
        cursor.up();
        assert_eq!(cursor.y, 0);

        cursor.down();
        cursor.right();
        assert_eq!(Point::from(cursor), Point::new(1, 1));

        Ok(())
    }
}
