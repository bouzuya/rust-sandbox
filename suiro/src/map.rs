use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
    hash::Hash,
    iter,
    str::FromStr,
};

use rand::{prelude::ThreadRng, Rng};

use crate::{direction::Direction, point::Point, size::Size, Pipe};

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("too few pipes")]
    TooFewPipes,
    #[error("invalid format")]
    InvalidFormat,
    #[error("invalid pipe")]
    InvalidPipe(#[from] crate::pipe::Error),
    #[error("invalid size")]
    InvalidSize(#[from] crate::size::Error),
    #[error("too many pipes")]
    TooManyPipes,
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Eq, PartialEq)]
pub struct Map {
    size: Size,
    pipes: Vec<Pipe>,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bytes = iter::once(u8::from(self.size))
            .chain(self.pipes.iter().map(|p| u8::from(*p)))
            .collect::<Vec<u8>>();
        let s = base32::encode(base32::Alphabet::Crockford, &bytes);
        write!(f, "{}", s)
    }
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes =
            base32::decode(base32::Alphabet::Crockford, s).ok_or(Self::Err::InvalidFormat)?;
        if bytes.is_empty() {
            return Err(Self::Err::InvalidFormat);
        }
        let size = Size::from(bytes[0]);
        let pipes = bytes
            .iter()
            .skip(1)
            .copied()
            .map(|b| Pipe::try_from(b).map_err(Self::Err::from))
            .collect::<Result<Vec<Pipe>>>()?;
        Map::new(size, pipes)
    }
}

impl Map {
    pub fn gen(size: Size) -> Result<Self> {
        let (w, h) = (usize::from(size.width()), usize::from(size.height()));
        loop {
            let mut ok = true;
            let mut b1 = generate_maze(w, h);
            remove_some_walls(w, h, &mut b1);
            let b2 = remove_walls(w, h, &b1);
            for i in 0..h {
                for j in 0..w {
                    if b2[i][j] == '┼' {
                        ok = false;
                    }
                }
            }
            if !ok {
                continue;
            }
            let mut pipes = vec![];
            let mut rng = rand::thread_rng();
            for i in 0..h {
                for j in 0..w {
                    let mut pipe = Pipe::try_from(b2[i][j])?;
                    for _ in 0..rng.gen_range(0..4) {
                        pipe = pipe.rotate();
                    }
                    pipes.push(pipe);
                }
            }
            return Ok(Self { size, pipes });
        }
    }

    pub fn new(size: Size, pipes: Vec<Pipe>) -> Result<Self> {
        let length = u16::try_from(pipes.len()).map_err(|_| Error::TooManyPipes)?;
        match length.cmp(&(u16::from(size.width()) * u16::from(size.height()))) {
            std::cmp::Ordering::Less => Err(Error::TooFewPipes),
            std::cmp::Ordering::Greater => Err(Error::TooManyPipes),
            std::cmp::Ordering::Equal => Ok(()),
        }?;
        Ok(Self { size, pipes })
    }

    pub fn height(&self) -> u8 {
        self.size.height()
    }

    pub fn pipe(&self, point: Point) -> Pipe {
        let x = usize::from(point.x());
        let y = usize::from(point.y());
        let w = usize::from(self.width());
        self.pipes[y * w + x]
    }

    pub fn rotate(&mut self, point: Point) {
        let x = usize::from(point.x());
        let y = usize::from(point.y());
        let w = usize::from(self.width());
        self.pipes[y * w + x] = self.pipes[y * w + x].rotate();
    }

    pub fn test(&self) -> (bool, Vec<bool>, Vec<bool>) {
        let w = usize::from(self.width());
        let h = usize::from(self.height());
        let mut ok = true;
        let mut ng = vec![false; w * h];
        let mut checked = vec![None; w * h];
        if self.pipes.is_empty() {
            return (ok, ng, vec![]);
        }
        if !self.pipes[0].is_open(Direction::L) {
            ok = false;
            ng[0] = true;
            return (
                ok,
                ng,
                checked
                    .into_iter()
                    .map(|i| i.unwrap_or_default())
                    .collect::<Vec<bool>>(),
            );
        }
        let mut deque = VecDeque::new();
        deque.push_back((0, 0));
        checked[0] = Some(true);
        while let Some((x, y)) = deque.pop_front() {
            let p = self.pipes[y * w + x];
            let dir = vec![Direction::T, Direction::B, Direction::L, Direction::R];
            for d in dir {
                if p.is_open(d) {
                    match d {
                        Direction::T => {
                            if y != 0 && self.pipes[(y - 1) * w + x].is_open(Direction::B) {
                                if checked[(y - 1) * w + x].is_none() {
                                    checked[(y - 1) * w + x] = Some(true);
                                    deque.push_back((x, y - 1));
                                }
                            } else {
                                ok = false;
                                ng[y * w + x] = true;
                            }
                        }
                        Direction::B => {
                            if y + 1 != h && self.pipes[(y + 1) * w + x].is_open(Direction::T) {
                                if checked[(y + 1) * w + x].is_none() {
                                    checked[(y + 1) * w + x] = Some(true);
                                    deque.push_back((x, y + 1));
                                }
                            } else {
                                ok = false;
                                ng[y * w + x] = true;
                            }
                        }
                        Direction::L => {
                            if x != 0 && self.pipes[y * w + x - 1].is_open(Direction::R) {
                                if checked[y * w + x - 1].is_none() {
                                    checked[y * w + x - 1] = Some(true);
                                    deque.push_back((x - 1, y));
                                }
                            } else if !(x == 0 && y == 0) {
                                ok = false;
                                ng[y * w + x] = true;
                            }
                        }
                        Direction::R => {
                            if x + 1 != w && self.pipes[y * w + x + 1].is_open(Direction::L) {
                                if checked[y * w + x + 1].is_none() {
                                    checked[y * w + x + 1] = Some(true);
                                    deque.push_back((x + 1, y));
                                }
                            } else if !(x + 1 == w && y + 1 == h) {
                                ok = false;
                                ng[y * w + x] = true;
                            }
                        }
                    }
                }
            }
        }
        (
            ok,
            ng,
            checked
                .into_iter()
                .map(|i| i.unwrap_or_default())
                .collect::<Vec<bool>>(),
        )
    }

    pub fn width(&self) -> u8 {
        self.size.width()
    }
}

struct RandomSet<T: Clone + Eq + Hash> {
    index: HashSet<T>,
    items: Vec<T>,
    rng: ThreadRng,
}

impl<T: Clone + Eq + Hash> RandomSet<T> {
    pub fn new() -> Self {
        Self {
            index: HashSet::default(),
            items: Vec::default(),
            rng: rand::thread_rng(),
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.items.is_empty() {
            return None;
        }
        let i = self.rng.gen_range(0..self.items.len());
        let j = self.items.len() - 1;
        self.items.swap(i, j);
        let item = self.items.pop().expect("vec is not empty");
        self.index.remove(&item);
        Some(item)
    }

    pub fn push(&mut self, item: T) {
        if self.index.contains(&item) {
            return;
        }
        self.index.insert(item.clone());
        self.items.push(item);
    }
}

fn generate_maze(w: usize, h: usize) -> Vec<Vec<char>> {
    assert!(w > 0 && h > 0);
    let w = w * 2 - 1;
    let h = h * 2 - 1;

    let mut board = vec![vec!['#'; w]; h];
    let mut start = RandomSet::new();
    start.push((0, 0));
    while let Some((x, y)) = start.pop() {
        let mut cand = RandomSet::new();

        let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
        for (dr, dc) in dir {
            let (r1, c1) = (y as i64 + dr, x as i64 + dc);
            if !(0..h as i64).contains(&r1) || !(0..w as i64).contains(&c1) {
                continue;
            }
            let (r1, c1) = (r1 as usize, c1 as usize);
            if board[r1][c1] == '.' {
                continue;
            }
            let (r2, c2) = (r1 as i64 + dr, c1 as i64 + dc);
            if !(0..h as i64).contains(&r2) || !(0..w as i64).contains(&c2) {
                continue;
            }
            let (r2, c2) = (r2 as usize, c2 as usize);
            if board[r2][c2] == '.' {
                continue;
            }
            cand.push(vec![(r1, c1), (r2, c2)]);
        }

        if let Some(cand) = cand.pop() {
            for (y, x) in iter::once((y, x)).chain(cand.into_iter()) {
                board[y][x] = '.';
                if y % 2 == 0 && x % 2 == 0 {
                    start.push((x, y));
                }
            }
        }
    }

    board
}

fn get_maze_route(w: usize, h: usize, board: &Vec<Vec<char>>) -> Vec<Vec<bool>> {
    fn dfs(
        w: usize,
        h: usize,
        board: &Vec<Vec<char>>,
        route: &mut Vec<Vec<bool>>,
        cur: (usize, usize),
        prev: (usize, usize),
    ) {
        if route[cur.0][cur.1] {
            return;
        }
        route[cur.0][cur.1] = true;
        let mut count = 0;
        let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
        for (dr, dc) in dir {
            let (nr, nc) = (cur.0 as i64 + dr, cur.1 as i64 + dc);
            if !(0..h as i64).contains(&nr) || !(0..w as i64).contains(&nc) {
                continue;
            }
            let (nr, nc) = (nr as usize, nc as usize);
            if nr == prev.0 && nc == prev.1 {
                count += 1;
                continue;
            }
            if board[nr][nc] == '#' {
                continue;
            }
            dfs(w, h, board, route, (nr, nc), cur);
            if route[nr][nc] {
                count += 1;
            }
        }
        route[cur.0][cur.1] = count >= 2;
    }

    let w = w * 2 - 1;
    let h = h * 2 - 1;
    let mut res = vec![vec![false; w]; h];
    res[h - 1][w - 1] = true;
    dfs(w, h, board, &mut res, (0, 0), (0, 0));
    res[0][0] = true;

    res
}

// 行き止まりを減らす
fn remove_some_walls(w: usize, h: usize, board: &mut Vec<Vec<char>>) {
    let route = get_maze_route(w, h, board);

    let w = w * 2 - 1;
    let h = h * 2 - 1;
    let mut end = vec![vec![false; w]; h];
    for i in 0..h {
        for j in 0..w {
            if i % 2 == 0 && j % 2 == 0 && !route[i][j] {
                let mut count = 0;
                let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
                for (dr, dc) in dir {
                    let (nr, nc) = (i as i64 + dr, j as i64 + dc);
                    if !(0..h as i64).contains(&nr) || !(0..w as i64).contains(&nc) {
                        continue;
                    }
                    let (nr, nc) = (nr as usize, nc as usize);
                    if board[nr][nc] == '.' {
                        count += 1;
                    }
                }
                if count <= 1 {
                    end[i][j] = true;
                }
            }
        }
    }

    let mut rng = rand::thread_rng();
    let mut target_walls = vec![];
    for i in 0..h {
        for j in 0..w {
            if !end[i][j] {
                continue;
            }

            let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
            for (dr, dc) in dir {
                let (r1, c1) = (i as i64 + dr, j as i64 + dc);
                if !(0..h as i64).contains(&r1) || !(0..w as i64).contains(&c1) {
                    continue;
                }
                let (r1, c1) = (r1 as usize, c1 as usize);
                if board[r1][c1] != '#' {
                    continue;
                }
                let (r2, c2) = (r1 as i64 + dr, c1 as i64 + dc);
                if !(0..h as i64).contains(&r2) || !(0..w as i64).contains(&c2) {
                    continue;
                }
                let (r2, c2) = (r2 as usize, c2 as usize);
                if board[r2][c2] != '.' {
                    continue;
                }

                if rng.gen_bool(0.2) {
                    target_walls.push((r1, c1));
                }
            }
        }
    }
    for (r, c) in target_walls {
        board[r][c] = '.';
    }
}

// 迷路の壁を削除し接続方向を表示する
fn remove_walls(w: usize, h: usize, board: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let route = get_maze_route(w, h, board);
    let w = w * 2 - 1;
    let h = h * 2 - 1;
    let mut b2 = vec![vec![0b0000_u8; (w + 1) / 2]; (h + 1) / 2];
    for i in 0..h {
        for j in 0..w {
            if i % 2 == 0 && j % 2 == 0 {
                let dir = vec![(-1, 0), (0, 1), (1, 0), (0, -1)]
                    .into_iter()
                    .enumerate()
                    .fold(0_u8, |acc, (index, (dr, dc))| {
                        let (nr, nc) = (i as i64 + dr, j as i64 + dc);
                        if !(0..h as i64).contains(&nr) || !(0..w as i64).contains(&nc) {
                            return acc;
                        }
                        let (nr, nc) = (nr as usize, nc as usize);
                        acc | if board[nr][nc] == '.' {
                            if route[i][j] && !route[nr][nc] {
                                0
                            } else {
                                1 << (3 - index)
                            }
                        } else {
                            0
                        }
                    });
                let dir = if i == 0 && j == 0 {
                    dir | 0b0001
                } else if i == h - 1 && j == w - 1 {
                    dir | 0b0100
                } else {
                    dir
                };
                b2[i / 2][j / 2] = dir;
            }
        }
    }

    let mut b3 = vec![vec![' '; (w + 1) / 2]; (h + 1) / 2];
    for i in 0..(h + 1) / 2 {
        for j in 0..(w + 1) / 2 {
            b3[i][j] = match b2[i][j] {
                0b0000 => unreachable!(),
                0b0001 => '─', // '╴',
                0b0010 => '│', // '╷',
                0b0100 => '─', // '╶',
                0b1000 => '│', // '╵',
                0b1010 => '│',
                0b0101 => '─',
                0b1100 => '└',
                0b0110 => '┌',
                0b0011 => '┐',
                0b1001 => '┘',
                0b0111 => '┬',
                0b1011 => '┤',
                0b1101 => '┴',
                0b1110 => '├',
                0b1111 => '┼',
                _ => unreachable!(),
            };
        }
    }
    b3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_test() -> anyhow::Result<()> {
        let pipe_i = Pipe::try_from('│')?;
        let pipe_l = Pipe::try_from('└')?;

        // │
        let s = base32::encode(base32::Alphabet::Crockford, &[0b00000000, 0b00000100]);
        assert_eq!(s, "0020");
        assert_eq!(
            Map::from_str(s.as_str()),
            Map::new(Size::new(1, 1)?, vec![pipe_i])
        );

        // ││
        let s = base32::encode(
            base32::Alphabet::Crockford,
            &[0b00010000, 0b00000100, 0b00000100],
        );
        assert_eq!(s, "20208");
        assert_eq!(
            Map::from_str(s.as_str()),
            Map::new(Size::new(2, 1)?, vec![pipe_i, pipe_i])
        );

        // │└
        // │└
        let s = base32::encode(
            base32::Alphabet::Crockford,
            &[0b00010001, 0b00000100, 0b00001000, 0b00000100, 0b00001000],
        );
        assert_eq!(s, "2420G108");
        assert_eq!(
            Map::from_str(s.as_str()),
            Map::new(Size::new(2, 2)?, vec![pipe_i, pipe_l, pipe_i, pipe_l])
        );

        Ok(())
    }

    #[test]
    fn new_test() -> anyhow::Result<()> {
        let pipe_i = Pipe::try_from('│')?;
        let pipe_l = Pipe::try_from('└')?;
        let pipe_t = Pipe::try_from('┬')?;

        let area = Map::new(Size::new(2, 2)?, vec![pipe_i, pipe_l, pipe_t, pipe_l])?;
        assert_eq!(area.pipe(Point::new(0, 0)), pipe_i);
        assert_eq!(area.pipe(Point::new(1, 0)), pipe_l);
        assert_eq!(area.pipe(Point::new(0, 1)), pipe_t);
        assert_eq!(area.pipe(Point::new(1, 1)), pipe_l);
        Ok(())
    }

    #[test]
    fn height_test() -> anyhow::Result<()> {
        let pipe_i = Pipe::try_from('│')?;
        let pipe_l = Pipe::try_from('└')?;

        let area = Map::new(Size::new(1, 2)?, vec![pipe_i, pipe_l])?;
        assert_eq!(area.height(), 2);
        Ok(())
    }

    #[test]
    fn rotate_test() -> anyhow::Result<()> {
        let pipe_i = Pipe::try_from('│')?;
        let pipe_l = Pipe::try_from('└')?;
        let pipe_t = Pipe::try_from('┬')?;

        let mut area = Map::new(Size::new(2, 2)?, vec![pipe_i, pipe_l, pipe_t, pipe_l])?;
        assert_eq!(area.pipe(Point::new(0, 0)), pipe_i);
        assert_eq!(area.pipe(Point::new(1, 0)), pipe_l);
        assert_eq!(area.pipe(Point::new(0, 1)), pipe_t);
        assert_eq!(area.pipe(Point::new(1, 1)), pipe_l);
        area.rotate(Point::new(0, 1));
        assert_eq!(area.pipe(Point::new(0, 1)), pipe_t.rotate());
        area.rotate(Point::new(0, 0));
        assert_eq!(area.pipe(Point::new(0, 0)), pipe_i.rotate());
        Ok(())
    }

    #[test]
    fn test_test() -> anyhow::Result<()> {
        let pipe_i = Pipe::try_from('│')?;
        let pipe_l = Pipe::try_from('└')?;
        let pipe_t = Pipe::try_from('┬')?;

        let area = Map::new(
            Size::new(2, 2)?,
            vec![pipe_i.rotate(), pipe_l, pipe_t, pipe_l],
        )?;
        let (ok, ng, flow) = area.test();
        assert!(!ok);
        assert_eq!(ng, vec![true, false, false, false]);
        assert_eq!(flow, vec![true, false, false, false]);

        let area = Map::new(
            Size::new(2, 2)?,
            vec![pipe_i.rotate(), pipe_l.rotate().rotate(), pipe_t, pipe_l],
        )?;
        let (ok, ng, flow) = area.test();
        assert!(ok);
        assert_eq!(ng, vec![false, false, false, false]);
        assert_eq!(flow, vec![true, true, false, true]);
        Ok(())
    }

    #[test]
    fn width_test() -> anyhow::Result<()> {
        let pipe_i = Pipe::try_from('│')?;
        let pipe_l = Pipe::try_from('└')?;
        let area = Map::new(Size::new(1, 2)?, vec![pipe_i, pipe_l])?;
        assert_eq!(area.width(), 1);
        Ok(())
    }

    #[allow(dead_code)]
    fn print_phase1(w: usize, h: usize, board: &[Vec<char>]) {
        let w = w * 2 - 1;
        let h = h * 2 - 1;
        for i in 0..h {
            for j in 0..w {
                print!("{}", board[i][j]);
            }
            println!();
        }
        println!();
    }

    #[test]
    fn gen_test() {
        let (w, h) = (4, 4);
        let mut b1 = generate_maze(w, h);
        // print_phase1(w, h, &b1);
        remove_some_walls(w, h, &mut b1);
        // print_phase1(w, h, &b1);
        remove_walls(w, h, &b1);
        // for i in 0..h {
        //     for j in 0..w {
        //         print!("{}", b2[i][j]);
        //     }
        //     println!();
        // }
        // println!();
    }
}
