use crate::dungeon::map_chips::MapChip;

pub trait MapChipsExtensions {
    fn draw(&self);
}

impl MapChipsExtensions for Vec<Vec<MapChip>> {
    fn draw(&self) {
        for row in self.iter() {
            for col in row.iter().copied() {
                match col {
                    MapChip::Wall => print!(" "),
                    MapChip::Room => print!("."),
                    MapChip::Passage => print!("."),
                    MapChip::Door => print!("+"),
                    MapChip::UpStairs => print!("<"),
                    MapChip::DownStairs => print!(">"),
                }
                println!();
            }
        }
    }
}
