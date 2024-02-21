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
}

#[cfg(test)]
mod tests {
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
}
