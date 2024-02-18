use super::{map_chips::MapChip, room::Room};

pub struct Passage {
    pub steps: Vec<(usize, usize)>,
}

impl Passage {
    pub fn new(from: Room, to: Room) -> Passage {
        let mut steps = vec![];
        let (from_x, from_y) = (from.x + from.width / 2, from.y + from.height / 2);
        let (to_x, to_y) = (to.x + to.width / 2, to.y + to.height / 2);
        let (mut x, mut y) = (from_x, from_y);
        steps.push((x, y));
        while x != to_x {
            x = (x as i64 + (if x < to_x { 1 } else { -1 })) as usize;
            steps.push((x, y));
        }
        while y != to_y {
            y = (y as i64 + (if y < to_y { 1 } else { -1 })) as usize;
            steps.push((x, y));
        }
        Passage { steps }
    }

    // 中央の部屋とその上下左右の部屋のうちランダムな部屋を配列で返す
    pub fn get_random_central_passage(rooms: &[Room]) -> Vec<Room> {
        let center = rooms[4].clone();
        let center_top = rooms[1].clone();
        let center_left = rooms[3].clone();
        let center_right = rooms[5].clone();
        let center_bottom = rooms[7].clone();
        let candidates = vec![center_top, center_left, center_right, center_bottom];
        let choice: Room =
            rand::seq::SliceRandom::choose(candidates.as_slice(), &mut rand::thread_rng())
                .expect("candidates is not empty")
                .clone();
        vec![center, choice]
    }

    // 外周順 (0, 1, 2, 5, 8, 7, 6, 3, 0) に並べる
    pub fn get_outer_perimeter(rooms: &[Room]) -> Vec<Room> {
        vec![
            rooms[0].clone(),
            rooms[1].clone(),
            rooms[2].clone(),
            rooms[5].clone(),
            rooms[8].clone(),
            rooms[7].clone(),
            rooms[6].clone(),
            rooms[3].clone(),
            rooms[0].clone(),
        ]
    }

    pub fn write_to_map(&self, map: &mut [Vec<MapChip>]) {
        for (x, y) in &self.steps {
            map[*y][*x] = MapChip::Passage;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dungeon::{map_chips::MapChip, map_generator::MapGenerator, room::Room};

    use super::*;

    #[test]
    fn test_new_2つの部屋の中心をつなぐ通路を生成できること() {
        let from = Room {
            x: 0,
            y: 0,
            width: 3,
            height: 3,
        };
        let to = Room {
            x: 4,
            y: 2,
            width: 4,
            height: 4,
        };
        let expected = [
            (1, 1),
            (2, 1),
            (3, 1),
            (4, 1),
            (5, 1),
            (6, 1),
            (6, 2),
            (6, 3),
            (6, 4),
        ];
        let actual = Passage::new(from, to);
        assert_eq!(actual.steps, expected);
    }

    #[test]
    fn test_write_to_map_通路をマップ配列に書き込めること() {
        let from = Room {
            x: 0,
            y: 0,
            width: 3,
            height: 3,
        };
        let to = Room {
            x: 4,
            y: 2,
            width: 4,
            height: 4,
        };
        let mut map = MapGenerator::new(8, 6);
        let expected = map_util_parse(
            r#"
WWWWWWWW
WPPPPPPW
WWWWWWPW
WWWWWWPW
WWWWWWPW
WWWWWWWW
"#
            .trim(),
        );
        let passage = Passage::new(from, to);
        passage.write_to_map(&mut map.map);
        assert_eq!(map.map, expected);
    }

    #[test]
    fn test_get_outer_perimeter_外周の部屋を連結順に並べた配列が変えること() {
        let rooms = build_rooms();
        let expeted = vec![0, 1, 2, 5, 8, 7, 6, 3, 0]
            .into_iter()
            .map(|i| rooms[i].clone())
            .collect::<Vec<Room>>();
        let actual = Passage::get_outer_perimeter(&rooms);
        assert_eq!(actual, expeted);
    }

    #[test]
    fn test_get_random_centeral_passage_中央の部屋とランダムな部屋を通路でつなぐ配列が返ること() {
        let rooms = build_rooms();
        let center = rooms[4].clone();
        let center_top = rooms[1].clone();
        let center_left = rooms[3].clone();
        let center_right = rooms[5].clone();
        let center_bottom = rooms[7].clone();
        let actual = Passage::get_random_central_passage(&rooms);
        assert_eq!(actual[0], center);
        assert!([center_top, center_left, center_right, center_bottom].contains(&actual[1]));
    }

    fn build_rooms() -> Vec<Room> {
        vec![
            (0, 0),
            (2, 0),
            (4, 0),
            (0, 2),
            (2, 2),
            (4, 2),
            (0, 4),
            (2, 4),
            (4, 4),
        ]
        .into_iter()
        .map(|(x, y)| Room {
            x,
            y,
            width: 2,
            height: 2,
        })
        .collect::<Vec<Room>>()
    }

    fn map_util_parse(s: &str) -> Vec<Vec<MapChip>> {
        s.lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        'P' => MapChip::Passage,
                        'W' => MapChip::Wall,
                        _ => unreachable!(),
                    })
                    .collect::<Vec<MapChip>>()
            })
            .collect::<Vec<Vec<MapChip>>>()
    }
}
