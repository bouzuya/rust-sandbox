use crate::{point::Point, size::Size};

#[derive(Clone, Copy, Debug)]
pub struct Cursor {
    size: Size,
    x: u8,
    y: u8,
}

impl Cursor {
    pub fn new(size: Size, x: u8, y: u8) -> Self {
        Self { size, x, y }
    }

    pub fn is_bottom_edge(&self) -> bool {
        self.y == self.size.height() - 1
    }

    pub fn is_left_edge(&self) -> bool {
        self.x == 0
    }

    pub fn is_right_edge(&self) -> bool {
        self.x == self.size.width() - 1
    }

    pub fn is_top_edge(&self) -> bool {
        self.y == 0
    }

    pub fn move_down(&mut self) {
        if !self.is_bottom_edge() {
            self.y += 1
        }
    }

    pub fn move_left(&mut self) {
        if !self.is_left_edge() {
            self.x -= 1
        }
    }

    pub fn move_right(&mut self) {
        if !self.is_right_edge() {
            self.x += 1
        }
    }

    pub fn move_up(&mut self) {
        if !self.is_top_edge() {
            self.y -= 1
        }
    }

    pub fn x(&self) -> u8 {
        self.x
    }

    pub fn y(&self) -> u8 {
        self.y
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

        cursor.move_down();
        assert_eq!(cursor.y(), 1);
        cursor.move_down();
        assert_eq!(cursor.y(), 2);
        cursor.move_down();
        assert_eq!(cursor.y(), 2);
        assert!(cursor.is_bottom_edge());

        cursor.move_right();
        assert_eq!(cursor.x(), 1);
        cursor.move_right();
        assert_eq!(cursor.x(), 2);
        cursor.move_right();
        assert_eq!(cursor.x(), 2);
        assert!(cursor.is_right_edge());

        cursor.move_left();
        assert_eq!(cursor.x(), 1);
        cursor.move_left();
        assert_eq!(cursor.x(), 0);
        cursor.move_left();
        assert_eq!(cursor.x(), 0);
        assert!(cursor.is_left_edge());

        cursor.move_up();
        assert_eq!(cursor.y(), 1);
        cursor.move_up();
        assert_eq!(cursor.y(), 0);
        cursor.move_up();
        assert_eq!(cursor.y(), 0);
        assert!(cursor.is_top_edge());

        cursor.move_down();
        cursor.move_right();
        assert_eq!(Point::from(cursor), Point::new(1, 1));

        Ok(())
    }
}
