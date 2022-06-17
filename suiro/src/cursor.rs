use crate::point::Point;

#[derive(Clone, Copy, Debug)]
pub struct Cursor {
    pub x: u8,
    pub y: u8,
}

impl Cursor {
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}

impl From<Cursor> for Point {
    fn from(cursor: Cursor) -> Self {
        Point::new(cursor.x, cursor.y)
    }
}
