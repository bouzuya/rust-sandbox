mod cursor;
mod direction;
mod map;
mod pipe;
mod point;
mod size;

use self::map::Map;
use self::pipe::Pipe;
use self::point::Point;

use clap::Parser;
use cursor::Cursor;
use size::Size;
use std::{
    io::{self, StdoutLock, Write},
    str::FromStr,
};
use termion::{input::TermRead, raw::IntoRawMode};

fn print(stdout: &mut StdoutLock, game: &Game) -> anyhow::Result<()> {
    let color_flow = termion::color::Fg(termion::color::LightBlue);
    let color_ng = termion::color::Fg(termion::color::Red);
    let (map, cursor, count) = (&game.map, game.cursor, game.count);
    let w = map.width();
    let h = map.height();
    let (ok, ng, flow) = map.test();
    write!(stdout, "{}", termion::cursor::Goto(1, 1))?;
    if ok {
        write!(
            stdout,
            " COUNT {} / q: quit / GAME OVER                       ",
            count
        )?;
    } else {
        write!(
            stdout,
            " COUNT {} / q: quit / ←↓↑→: move / space: rotate right",
            count
        )?;
    }
    write!(stdout, "{}", termion::cursor::Goto(1, 2))?;
    write!(
        stdout,
        "{}",
        (0..w * 2 + 2 + 1)
            .map(|x| if x == 0 {
                '╔'
            } else if x + 1 == w * 2 + 2 + 1 {
                '╗'
            } else {
                '═'
            })
            .collect::<String>()
    )?;
    for y in 0..h {
        write!(stdout, "{}", termion::cursor::Goto(1, 3 + u16::from(y)))?;
        write!(
            stdout,
            "{}",
            if y == 0 {
                format!(
                    "{}{}{}",
                    color_flow,
                    '━',
                    termion::color::Fg(termion::color::Reset)
                )
            } else {
                "║".to_string()
            }
        )?;

        let _ = stdout.write(
            if ok {
                " "
            } else if cursor.is_left_edge() && cursor.y() == y {
                "["
            } else {
                " "
            }
            .as_bytes(),
        )?;
        for x in 0..w {
            let p = map.pipe(Point::new(x, y));
            let c = if flow[usize::from(y) * usize::from(w) + usize::from(x)] {
                format!(
                    "{}{}{}",
                    if ng[usize::from(y) * usize::from(w) + usize::from(x)] {
                        color_ng.to_string()
                    } else {
                        color_flow.to_string()
                    },
                    match p {
                        Pipe::I(d) => match d {
                            0 => '┃',
                            1 => '━',
                            _ => unreachable!(),
                        },
                        Pipe::L(d) => match d {
                            0 => '┗',
                            1 => '┏',
                            2 => '┓',
                            3 => '┛',
                            _ => unreachable!(),
                        },
                        Pipe::T(d) => match d {
                            0 => '┳',
                            1 => '┫',
                            2 => '┻',
                            3 => '┣',
                            _ => unreachable!(),
                        },
                    },
                    termion::color::Fg(termion::color::Reset),
                )
            } else {
                format!("{}", p)
            };
            let _ = stdout.write(
                format!(
                    "{}{}",
                    c,
                    if ok {
                        " "
                    } else if cursor.x() == x && cursor.y() == y {
                        "]"
                    } else if cursor.x() == x + 1 && cursor.y() == y {
                        "["
                    } else {
                        " "
                    }
                )
                .as_bytes(),
            )?;
        }
        write!(
            stdout,
            "{}",
            if y + 1 == h {
                format!(
                    "{}{}{}",
                    if ok {
                        color_flow.to_string()
                    } else {
                        "".to_string()
                    },
                    '━',
                    termion::color::Fg(termion::color::Reset)
                )
            } else {
                "║".to_string()
            }
        )?;
    }
    write!(stdout, "{}", termion::cursor::Goto(1, 3 + u16::from(h)))?;
    write!(
        stdout,
        "{}",
        (0..w * 2 + 2 + 1)
            .map(|x| if x == 0 {
                '╚'
            } else if x + 1 == w * 2 + 2 + 1 {
                '╝'
            } else {
                '═'
            })
            .collect::<String>()
    )?;
    Ok(())
}

struct Game {
    map: Map,
    count: usize,
    cursor: Cursor,
    result: (bool, Vec<bool>, Vec<bool>),
}

impl Game {
    fn new(map: Map) -> anyhow::Result<Self> {
        let count = 0_usize;
        let cursor = Cursor::new(Size::new(map.width(), map.height())?, 0, 0);
        let result = map.test();
        Ok(Self {
            map,
            count,
            cursor,
            result,
        })
    }

    fn is_over(&self) -> bool {
        self.result.0
    }

    fn rotate(&mut self) {
        if self.is_over() {
            return;
        }
        self.map.rotate(self.cursor.into());
        self.count += 1;
        self.result = self.map.test();
    }

    fn left(&mut self) {
        if self.is_over() {
            return;
        }
        self.cursor.move_left()
    }

    fn down(&mut self) {
        if self.is_over() {
            return;
        }
        self.cursor.move_down()
    }

    fn up(&mut self) {
        if self.is_over() {
            return;
        }
        self.cursor.move_up()
    }

    fn right(&mut self) {
        if self.is_over() {
            return;
        }
        self.cursor.move_right()
    }
}

#[derive(Parser)]
#[clap(version)]
struct Opt {
    #[clap(long)]
    map: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let opt: Opt = Opt::parse();
    let map = opt
        .map
        .map(|s| Map::from_str(s.as_str()))
        .unwrap_or_else(|| {
            let size = Size::new(16, 16).map_err(map::Error::from)?;
            Map::new(
                size,
                (0..u16::from(size.width()) * u16::from(size.height()))
                    .into_iter()
                    .map(|_| Pipe::I(1))
                    .collect::<Vec<Pipe>>(),
            )
        })?;

    let stdout = io::stdout().lock();
    let stdin = io::stdin().lock();
    let mut stdout = stdout.into_raw_mode().unwrap();

    write!(stdout, "{}", termion::clear::All)?;
    write!(stdout, "{}", termion::cursor::Hide)?;
    write!(stdout, "{}", termion::cursor::Goto(1, 1))?;

    let mut game = Game::new(map)?;

    print(&mut stdout, &game)?;
    stdout.flush()?;

    for key in stdin.keys() {
        use termion::event::Key::*;
        match key? {
            Char(' ') | Char('\n') => game.rotate(),
            Char('h') | Left => game.left(),
            Char('j') | Down => game.down(),
            Char('k') | Up => game.up(),
            Char('l') | Right => game.right(),
            Char('q') | Esc => break,
            _ => {}
        }
        print(&mut stdout, &game)?;
        stdout.flush()?;
    }

    write!(stdout, "{}", termion::cursor::Show)?;
    Ok(())
}
