use rand::{seq::SliceRandom as _, Rng as _};

use super::room::Room;

pub struct Stairs {
    pub room: Room,
    pub x: usize,
    pub y: usize,
}

impl Stairs {
    pub fn new(rooms: Vec<Room>) -> Self {
        let mut rng = rand::thread_rng();
        let room = rooms.choose(&mut rng).unwrap();
        let x = rng.gen_range(room.x..=room.right());
        let y = rng.gen_range(room.y..=room.bottom());
        Self {
            room: room.clone(),
            x,
            y,
        }
    }

    pub fn new_with_ignore_room(rooms: Vec<Room>, ignore_room: Room) -> Self {
        Self::new(rooms.into_iter().filter(|r| r != &ignore_room).collect())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::dungeon::room::Room;

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
            let stairs = Stairs::new(rooms.clone());
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
            let stairs = Stairs::new(rooms.clone());
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
            let stairs = Stairs::new(rooms.clone());
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
        let stairs = Stairs::new_with_ignore_room(rooms, room2);
        assert!([room1, room3].contains(&stairs.room));
    }
}
