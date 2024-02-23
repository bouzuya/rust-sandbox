use super::{door::Door, map_chips::MapChip, passage::Passage, room::Room, stairs::Stairs};

pub struct MapGenerator {
    pub map: Vec<Vec<MapChip>>,
}

impl MapGenerator {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            map: vec![vec![MapChip::Wall; width]; height],
        }
    }

    pub fn write(
        &mut self,
        rooms: &[Room],
        passages: &[Passage],
        doors: &[Door],
        stairs: &[Stairs],
    ) {
        for passage in passages {
            passage.write_to_map(&mut self.map);
        }
        for room in rooms {
            room.write_to_map(&mut self.map);
        }
        for door in doors {
            door.write_to_map(&mut self.map);
        }
        for stair in stairs {
            stair.write_to_map(&mut self.map);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dungeon::{
        door::Door, map_chips::MapChip, map_util::MapUtil, passage::Passage, room::Room,
        stairs::Stairs,
    };

    use super::*;

    #[test]
    fn test_map_generator_指定サイズのマップ配列が生成されること() {
        for (width, height) in [(20, 10), (30, 20)] {
            let sut = MapGenerator::new(width, height);

            assert_eq!(sut.map.len(), height);
            assert!(sut.map.iter().all(|row| row.len() == width));
        }
    }

    #[test]
    fn test_map_generator_マップ配列が壁で埋められていること() {
        let (width, height) = (20, 10);
        let sut = MapGenerator::new(width, height);

        assert_eq!(sut.map, vec![vec![MapChip::Wall; width]; height]);
    }

    #[test]
    fn test_write_マップを構成する要素をマップ配列に書き込めること() {
        let mut sut = MapGenerator::new(10, 9);
        let rooms = vec![
            Room {
                x: 1,
                y: 1,
                width: 3,
                height: 3,
            },
            Room {
                x: 5,
                y: 5,
                width: 4,
                height: 3,
            },
        ];
        let passages = vec![Passage::new(rooms[0].clone(), rooms[1].clone())];
        let mut doors = Door::create_doors(rooms[0].clone(), passages.clone());
        doors.extend(Door::create_doors(rooms[1].clone(), passages.clone()));
        let stairs = vec![
            Stairs::new(
                vec![Room {
                    x: 1,
                    y: 1,
                    width: 1,
                    height: 1,
                }],
                MapChip::UpStairs,
            ),
            Stairs::new(
                vec![Room {
                    x: 6,
                    y: 6,
                    width: 1,
                    height: 1,
                }],
                MapChip::DownStairs,
            ),
        ];
        let expected = MapUtil::parse(
            r#"
WWWWWWWWWW
WURRWWWWWW
WRRRDPPPWW
WRRRWWWPWW
WWWWWWWDWW
WWWWWRRRRW
WWWWWRSRRW
WWWWWRRRRW
WWWWWWWWWW
"#
            .trim(),
        );
        sut.write(&rooms, &passages, &doors, &stairs);
        assert_eq!(sut.map, expected);
    }
}
