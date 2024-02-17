use crate::dungeon::room;

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Room {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
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

    fn bottom(&self) -> usize {
        self.y + self.height - 1
    }

    fn right(&self) -> usize {
        self.x + self.width - 1
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

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
}
