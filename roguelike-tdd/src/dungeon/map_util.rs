use super::map_chips::MapChip;

pub struct MapUtil;

impl MapUtil {
    pub fn parse(s: &str) -> Vec<Vec<MapChip>> {
        s.lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        'D' => MapChip::Door,
                        'P' => MapChip::Passage,
                        'R' => MapChip::Room,
                        'S' => MapChip::DownStairs,
                        'U' => MapChip::UpStairs,
                        'W' => MapChip::Wall,
                        _ => unreachable!(),
                    })
                    .collect::<Vec<MapChip>>()
            })
            .collect::<Vec<Vec<MapChip>>>()
    }
}
