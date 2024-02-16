use super::map_chips::MapChip;

struct MapGenerator {
    map: Vec<Vec<MapChip>>,
}

impl MapGenerator {
    fn new(width: usize, height: usize) -> Self {
        Self {
            map: vec![vec![MapChip::Wall; width]; height],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dungeon::map_chips::MapChip;

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
}
