use super::{map_chips::MapChip, passage::Passage, room::Room};

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Door {
    pub x: usize,
    pub y: usize,
}

impl Door {
    pub fn new(x: usize, y: usize) -> Self {
        Door { x, y }
    }

    // 部屋の外周を走査して、隣接した通路の座標にドアをつくる
    pub fn create_doors(room: Room, passages: Vec<Passage>) -> Vec<Door> {
        let mut doors = vec![];
        for y in room.y - 1..room.y + room.height + 1 {
            for x in room.x - 1..room.x + room.width + 1 {
                // 角にはドアをつくらない
                if x == room.x - 1
                    || x == room.x + room.width
                    || y == room.y - 1
                    || y == room.y + room.height
                {
                    #[allow(clippy::nonminimal_bool)]
                    if x == room.x - 1 && y == room.y - 1
                        || x == room.x + room.width && y == room.y - 1
                        || x == room.x - 1 && y == room.y + room.height
                        || x == room.x + room.width && y == room.y + room.height
                    {
                        continue;
                    }
                    for passage in &passages {
                        if passage.is_point_on_passage(x, y) {
                            doors.push(Door::new(x, y));
                        }
                    }
                }
            }
        }
        doors
    }

    pub fn write_to_map(&self, map: &mut Vec<Vec<MapChip>>) {
        map[self.y][self.x] = MapChip::Door;
    }
}

#[cfg(test)]
mod tests {
    use crate::dungeon::{
        map_chips::MapChip, map_generator::MapGenerator, passage::Passage, room::Room,
    };

    use super::*;

    #[test]
    fn test_create_doors_部屋に隣接した通路があるとき_ドアを生成() {
        let room1 = Room {
            x: 1,
            y: 1,
            width: 3,
            height: 3,
        };
        let room2 = Room {
            x: 5,
            y: 2,
            width: 1,
            height: 1,
        };
        let passages = vec![Passage::new(room1.clone(), room2)];
        let expected = vec![Door::new(4, 2)];
        let actual = Door::create_doors(room1, passages);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_create_doors_部屋の上下左右に隣接した通路があるとき_ドアを複数生成() {
        let room1 = Room {
            x: 2,
            y: 2,
            width: 1,
            height: 1,
        };
        let room2 = Room {
            x: 2,
            y: 0,
            width: 1,
            height: 1,
        };
        let room3 = Room {
            x: 4,
            y: 2,
            width: 1,
            height: 1,
        };
        let room4 = Room {
            x: 2,
            y: 4,
            width: 1,
            height: 1,
        };
        let room5 = Room {
            x: 0,
            y: 2,
            width: 1,
            height: 1,
        };
        let passages = vec![
            Passage::new(room1.clone(), room2),
            Passage::new(room1.clone(), room3),
            Passage::new(room1.clone(), room4),
            Passage::new(room1.clone(), room5),
        ];
        let mut expected = vec![
            Door::new(2, 1),
            Door::new(3, 2),
            Door::new(2, 3),
            Door::new(1, 2),
        ];
        expected.sort();
        let mut actual = Door::create_doors(room1, passages);
        actual.sort();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_create_doors_部屋の角に隣接した通路があるとき_ドアは生成されない() {
        let room1 = Room {
            x: 1,
            y: 1,
            width: 3,
            height: 3,
        };
        let passages = vec![
            Passage {
                steps: vec![(0, 0)],
            },
            Passage {
                steps: vec![(4, 0)],
            },
            Passage {
                steps: vec![(0, 4)],
            },
            Passage {
                steps: vec![(4, 4)],
            },
        ];
        let expected = vec![];
        let actual = Door::create_doors(room1, passages);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_write_to_map_ドアをマップ配列に書き込めること() {
        let door = Door { x: 1, y: 1 };
        let mut map = MapGenerator::new(3, 2);
        door.write_to_map(&mut map.map);
        assert_eq!(
            map.map,
            vec![
                vec![MapChip::Wall, MapChip::Wall, MapChip::Wall],
                vec![MapChip::Wall, MapChip::Door, MapChip::Wall],
            ]
        );
    }
}
