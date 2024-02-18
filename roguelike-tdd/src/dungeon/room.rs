use crate::dungeon::room;

use super::map_chips::MapChip;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Room {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl Room {
    fn create_in_bounds(left: usize, top: usize, right: usize, bottom: usize) -> Self {
        Self::create_in_bounds_with_min_room_size(left, top, right, bottom, 1)
    }

    fn create_in_bounds_with_min_room_size(
        left: usize,
        top: usize,
        right: usize,
        bottom: usize,
        min_room_size: usize,
    ) -> Self {
        Self::create_in_bounds_with_min_room_size_and_padding(
            left,
            top,
            right,
            bottom,
            min_room_size,
            0,
        )
    }

    fn create_in_bounds_with_min_room_size_and_padding(
        left: usize,
        top: usize,
        right: usize,
        bottom: usize,
        min_room_size: usize,
        padding: usize,
    ) -> Self {
        use rand::Rng;

        let mut rng = rand::thread_rng();
        let room_max_width = right - left + 1 - padding * 2;
        let room_max_height = bottom - top + 1 - padding * 2;
        let width = rng.gen_range(min_room_size..=room_max_width);
        let height = rng.gen_range(min_room_size..=room_max_height);
        let x = rng.gen_range(left + padding..=(right + 1 - padding - width));
        let y = rng.gen_range(top + padding..=(bottom + 1 - padding - height));

        Self {
            x,
            y,
            width,
            height,
        }
    }

    fn create_rooms(
        map_width: usize,
        map_height: usize,
        min_room_size: usize,
        padding: usize,
    ) -> Vec<Self> {
        // 9 区画に分割し、それぞれに部屋を作成する
        let mut rooms = vec![];
        for y in 0..3 {
            for x in 0..3 {
                let left = x * (map_width / 3);
                let top = y * (map_height / 3);
                let right = left + map_width / 3 - 1;
                let bottom = top + map_height / 3 - 1;
                let room = Self::create_in_bounds_with_min_room_size_and_padding(
                    left,
                    top,
                    right,
                    bottom,
                    min_room_size,
                    padding,
                );
                rooms.push(room);
            }
        }
        rooms
    }

    fn bottom(&self) -> usize {
        self.y + self.height - 1
    }

    fn right(&self) -> usize {
        self.x + self.width - 1
    }

    fn write_to_map(&self, map: &mut [Vec<MapChip>]) {
        for y in self.y..=self.bottom() {
            for x in self.x..=self.right() {
                map[y][x] = crate::dungeon::map_chips::MapChip::Room;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::dungeon::{map_chips::MapChip, map_generator::MapGenerator};

    use super::*;

    #[test]
    fn test_create_in_bounds_部屋が指定区画内に作られること() {
        for case in [(0, 0, 19, 9), (20, 10, 39, 19)] {
            for _ in 0..100 {
                let (left, top, right, bottom) = case;
                let room_max_width = right - left + 1;
                let room_max_height = bottom - top + 1;
                let room = Room::create_in_bounds(left, top, right, bottom);

                assert!((left..=right).contains(&room.x));
                assert!((top..=bottom).contains(&room.y));
                assert!((1..=room_max_width).contains(&room.width));
                assert!((1..=room_max_height).contains(&room.height));
            }
        }
    }

    #[test]
    fn test_create_in_bounds_最小サイズ指定_部屋が指定区画内に作られること() {
        for case in [(0, 0, 19, 9, 1), (20, 10, 39, 19, 5)] {
            for _ in 0..100 {
                let (left, top, right, bottom, min_room_size) = case;
                let room_max_width = right - left + 1;
                let room_max_height = bottom - top + 1;
                let room = Room::create_in_bounds_with_min_room_size(
                    left,
                    top,
                    right,
                    bottom,
                    min_room_size,
                );

                assert!((left..=right).contains(&room.x));
                assert!((top..=bottom).contains(&room.y));
                assert!((min_room_size..=room_max_width).contains(&room.width));
                assert!((min_room_size..=room_max_height).contains(&room.height));
            }
        }
    }

    #[test]
    fn test_create_in_bounds_パディング指定_部屋が指定区画内に作られること() {
        for case in [(0, 0, 19, 9, 0), (20, 10, 39, 19, 2)] {
            for _ in 0..100 {
                let (left, top, right, bottom, padding) = case;
                let min_room_size = 1;
                let room_max_width = right - left + 1 - padding * 2;
                let room_max_height = bottom - top + 1 - padding * 2;
                let room_left = left + padding;
                let room_top = top + padding;
                let room_right = right - padding;
                let room_bottom = bottom - padding;
                let room = Room::create_in_bounds_with_min_room_size_and_padding(
                    left,
                    top,
                    right,
                    bottom,
                    min_room_size,
                    padding,
                );

                assert!((room_left..=room_right).contains(&room.x));
                assert!((room_top..=room_bottom).contains(&room.y));
                assert!((min_room_size..=room_max_width).contains(&room.width));
                assert!((min_room_size..=room_max_height).contains(&room.height));
            }
        }
    }

    #[test]
    fn test_create_in_bounds_部屋の位置とサイズがランダムであること() {
        let (left, top, right, bottom) = (0, 0, 19, 9);
        let mut x_set = std::collections::HashSet::new();
        let mut y_set = std::collections::HashSet::new();
        let mut width_set = std::collections::HashSet::new();
        let mut height_set = std::collections::HashSet::new();

        for _ in 0..100 {
            let room = Room::create_in_bounds(left, top, right, bottom);

            x_set.insert(room.x);
            y_set.insert(room.y);
            width_set.insert(room.width);
            height_set.insert(room.height);
        }

        assert!(x_set.len() > 1);
        assert!(y_set.len() > 1);
        assert!(width_set.len() > 1);
        assert!(height_set.len() > 1);
    }

    #[test]
    fn test_create_in_bounds_指定した最大サイズの部屋がつくられること() {
        let (left, top, right, bottom, min_room_size, padding) = (0, 0, 4, 4, 2, 1);
        let expected = [
            Room {
                x: 1,
                y: 1,
                width: 2,
                height: 2,
            },
            Room {
                x: 2,
                y: 1,
                width: 2,
                height: 2,
            },
            Room {
                x: 1,
                y: 2,
                width: 2,
                height: 2,
            },
            Room {
                x: 2,
                y: 2,
                width: 2,
                height: 2,
            },
            Room {
                x: 1,
                y: 1,
                width: 3,
                height: 2,
            },
            Room {
                x: 1,
                y: 2,
                width: 3,
                height: 2,
            },
            Room {
                x: 1,
                y: 1,
                width: 2,
                height: 3,
            },
            Room {
                x: 2,
                y: 1,
                width: 2,
                height: 3,
            },
            Room {
                x: 1,
                y: 1,
                width: 3,
                height: 3,
            },
        ]
        .into_iter()
        .collect::<HashSet<Room>>();
        let mut rooms = HashSet::new();
        for _ in 0..100 {
            let room = Room::create_in_bounds_with_min_room_size_and_padding(
                left,
                top,
                right,
                bottom,
                min_room_size,
                padding,
            );
            rooms.insert(room);
        }

        assert_eq!(rooms, expected);
    }

    #[test]
    fn test_right_右端座標を返す() {
        for (x, width, right) in [(0, 10, 9), (5, 15, 19)] {
            let room = Room {
                x,
                y: 2,
                width,
                height: 4,
            };
            assert_eq!(room.right(), right);
        }
    }

    #[test]
    fn test_height_下端座標を返す() {
        for (y, height, bottom) in [(0, 10, 9), (5, 15, 19)] {
            let room = Room {
                x: 1,
                y,
                width: 3,
                height,
            };
            assert_eq!(room.bottom(), bottom);
        }
    }

    #[test]
    fn test_write_to_map_部屋の位置をマップ配列に書き込めること() {
        let (map_width, map_height) = (4, 2);
        let (room_x, room_y, room_width, room_height) = (1, 0, 2, 1);
        let expected = vec![
            vec![MapChip::Wall, MapChip::Room, MapChip::Room, MapChip::Wall],
            vec![MapChip::Wall, MapChip::Wall, MapChip::Wall, MapChip::Wall],
        ];
        let mut map = MapGenerator::new(map_width, map_height);
        let room = Room {
            x: room_x,
            y: room_y,
            width: room_width,
            height: room_height,
        };
        room.write_to_map(&mut map.map);
        assert_eq!(map.map, expected);
    }

    #[test]
    fn create_rooms_部屋が9つ生成されること() {
        for (map_width, map_height) in [(15, 12), (18, 15)] {
            let min_room_size = 1;
            let padding = 0;
            let rooms = Room::create_rooms(map_width, map_height, min_room_size, padding);
            assert_eq!(rooms.len(), 9);
        }
    }

    #[test]
    fn create_rooms_各部屋は9つの区画内に生成されること() {
        for _ in 0..10 {
            let (width, height, min_room_size, padding) = (15, 12, 2, 1);
            let actual = Room::create_rooms(width, height, min_room_size, padding);
            for (index, left, top, right, bottom) in [
                (0, 1, 1, 3, 2),
                (1, 6, 1, 9, 2),
                (2, 11, 1, 13, 2),
                (3, 1, 5, 3, 6),
                (4, 6, 5, 9, 6),
                (5, 11, 5, 13, 6),
                (6, 1, 9, 3, 11),
                (7, 6, 9, 9, 11),
                (8, 11, 9, 13, 11),
            ] {
                assert!((left..=right).contains(&actual[index].x));
                assert!((top..=bottom).contains(&actual[index].y));
                assert!((left..=right).contains(&actual[index].right()));
                assert!((top..=bottom).contains(&actual[index].bottom()));
            }
        }
    }
}
