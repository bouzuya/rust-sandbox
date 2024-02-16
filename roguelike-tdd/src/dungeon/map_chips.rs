#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MapChip {
    Wall,
    Room,
    Passage,
    Door,
    UpStairs,
    DownStairs,
}
