use rand::{seq::SliceRandom as _, Rng as _};

use super::{map_chips::MapChip, room::Room};

pub struct Stairs {
    pub room: Room,
    pub x: usize,
    pub y: usize,
    pub stairs_type: MapChip,
}

impl Stairs {
    pub fn new(rooms: Vec<Room>, stairs_type: MapChip) -> Self {
        let mut rng = rand::thread_rng();
        let room = rooms.choose(&mut rng).unwrap();
        let x = rng.gen_range(room.x..=room.right());
        let y = rng.gen_range(room.y..=room.bottom());
        Self {
            room: room.clone(),
            x,
            y,
            stairs_type,
        }
    }

    pub fn new_with_ignore_room(rooms: Vec<Room>, ignore_room: Room, stairs_type: MapChip) -> Self {
        Self::new(
            rooms.into_iter().filter(|r| r != &ignore_room).collect(),
            stairs_type,
        )
    }

    pub fn write_to_map(&self, map: &mut [Vec<MapChip>]) {
        map[self.y][self.x] = self.stairs_type;
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::dungeon::{map_chips::MapChip, map_generator::MapGenerator, room::Room};

    use super::*;

    #[test]
    fn test_stairs_ランダムな部屋のランダムな座標に階段を設置できること() {
        for _ in 0..10 {
            let room1 = Room {
                x: 0,
                y: 0,
                width: 2,
                height: 2,
            };
            let room2 = Room {
                x: 10,
                y: 10,
                width: 2,
                height: 2,
            };
            let room3 = Room {
                x: 20,
                y: 20,
                width: 2,
                height: 2,
            };
            let rooms = vec![room1, room2, room3];
            let stairs = Stairs::new(rooms.clone(), MapChip::DownStairs);
            assert!(rooms.contains(&stairs.room));
            assert!((stairs.room.x..=stairs.room.right()).contains(&stairs.x));
            assert!((stairs.room.y..=stairs.room.bottom()).contains(&stairs.x));
        }
    }

    #[test]
    fn test_stairs_選択肢の最小値が抽選されること() {
        let room1 = Room {
            x: 0,
            y: 0,
            width: 2,
            height: 2,
        };
        let room2 = Room {
            x: 10,
            y: 10,
            width: 2,
            height: 2,
        };
        let room3 = Room {
            x: 20,
            y: 20,
            width: 2,
            height: 2,
        };
        let rooms = vec![room1.clone(), room2, room3];
        let mut set = HashSet::new();
        for _ in 0..100 {
            let stairs = Stairs::new(rooms.clone(), MapChip::DownStairs);
            set.insert(stairs.room);
        }
        assert!(set.contains(&room1));
    }

    #[test]
    fn test_stairs_選択肢の最大値が抽選されること() {
        let room1 = Room {
            x: 0,
            y: 0,
            width: 2,
            height: 2,
        };
        let room2 = Room {
            x: 10,
            y: 10,
            width: 2,
            height: 2,
        };
        let room3 = Room {
            x: 20,
            y: 20,
            width: 2,
            height: 2,
        };
        let rooms = vec![room1, room2, room3.clone()];
        let mut set = HashSet::new();
        for _ in 0..100 {
            let stairs = Stairs::new(rooms.clone(), MapChip::DownStairs);
            set.insert(stairs.room);
        }
        assert!(set.contains(&room3));
    }

    #[test]
    fn test_stairs_抽選から除外する部屋を指定_除外指定した部屋は抽選されない() {
        let room1 = Room {
            x: 0,
            y: 0,
            width: 2,
            height: 2,
        };
        let room2 = Room {
            x: 10,
            y: 10,
            width: 2,
            height: 2,
        };
        let room3 = Room {
            x: 20,
            y: 20,
            width: 2,
            height: 2,
        };
        let rooms = vec![room1.clone(), room2.clone(), room3.clone()];
        let stairs = Stairs::new_with_ignore_room(rooms, room2, MapChip::DownStairs);
        assert!([room1, room3].contains(&stairs.room));
    }

    #[test]
    fn test_write_to_map_登り階段をマップに書き込めること() {
        let room = Room {
            x: 1,
            y: 1,
            width: 1,
            height: 1,
        };
        let stairs_type = MapChip::UpStairs;
        let stairs = Stairs::new(vec![room], stairs_type);
        let mut map = MapGenerator::new(2, 2);
        let expected = map_util_parse(
            r#"
WW
WU
"#
            .trim(),
        );
        stairs.write_to_map(&mut map.map);
        assert_eq!(map.map, expected);
    }

    #[test]
    fn test_write_to_map_降り階段をマップに書き込めること() {
        let room = Room {
            x: 1,
            y: 1,
            width: 1,
            height: 1,
        };
        let stairs_type = MapChip::DownStairs;
        let stairs = Stairs::new(vec![room], stairs_type);
        let mut map = MapGenerator::new(2, 2);
        let expected = map_util_parse(
            r#"
WW
WD
"#
            .trim(),
        );
        stairs.write_to_map(&mut map.map);
        assert_eq!(map.map, expected);
    }

    fn map_util_parse(s: &str) -> Vec<Vec<MapChip>> {
        s.lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        'P' => MapChip::Passage,
                        'W' => MapChip::Wall,
                        'U' => MapChip::UpStairs,
                        'D' => MapChip::DownStairs,
                        _ => unreachable!("invalid map chip: {}", c),
                    })
                    .collect::<Vec<MapChip>>()
            })
            .collect::<Vec<Vec<MapChip>>>()
    }
}
