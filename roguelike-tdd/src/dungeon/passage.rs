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
