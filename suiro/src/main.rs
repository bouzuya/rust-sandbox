mod area;
mod cursor;
mod direction;
mod pipe;
mod point;
mod size;

use self::area::Area;
use self::pipe::Pipe;
use self::point::Point;

use cursor::Cursor;
use size::Size;
use std::io::{self, StdoutLock, Write};
use termion::{input::TermRead, raw::IntoRawMode};

fn print(stdout: &mut StdoutLock, game: &Game) -> anyhow::Result<()> {
    let (area, cursor, count) = (&game.area, game.cursor, game.count);
    let w = area.width();
    let h = area.height();
    let (ng, flow) = area.test();
    write!(stdout, "{}", termion::cursor::Goto(1, 1))?;
    write!(stdout, " count: {}, space: rotate, q: quit", count)?;
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
        write!(stdout, "{}", if y == 0 { '━' } else { '║' })?;
        let _ = stdout.write(
            if cursor.x == 0 && cursor.y == y {
                "["
            } else {
                " "
            }
            .as_bytes(),
        )?;
        for x in 0..w {
            let p = area.pipe(Point::new(x, y));
            let c = if flow[usize::from(y) * usize::from(w) + usize::from(x)] {
                format!(
                    "{}{}{}",
                    if ng[usize::from(y) * usize::from(w) + usize::from(x)] {
                        termion::color::Fg(termion::color::Red).to_string()
                    } else {
                        termion::color::Fg(termion::color::LightBlue).to_string()
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
                    if cursor.x == x && cursor.y == y {
                        "]"
                    } else if cursor.x == x + 1 && cursor.y == y {
                        "["
                    } else {
                        " "
                    }
                )
                .as_bytes(),
            )?;
        }
        write!(stdout, "{}", if y + 1 == h { '━' } else { '║' })?;
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
    area: Area,
    count: usize,
    cursor: Cursor,
}

impl Game {
    fn new() -> anyhow::Result<Self> {
        let count = 0_usize;
        let size = Size::new(16, 16)?;
        let cursor = Cursor::new(size, 0, 0);
        // let area = Area::new(2, 2, vec![Pipe::I(1), Pipe::L(0), Pipe::T(0), Pipe::L(0)])?;
        let area = Area::new(
            16,
            16,
            (0..16 * 16)
                .into_iter()
                .map(|_| Pipe::I(1))
                .collect::<Vec<Pipe>>(),
        )?;
        Ok(Self {
            area,
            count,
            cursor,
        })
    }

    fn rotate(&mut self) {
        self.area.rotate(self.cursor.into());
        self.count += 1;
    }

    fn left(&mut self) {
        self.cursor.left()
    }

    fn down(&mut self) {
        self.cursor.down()
    }

    fn up(&mut self) {
        self.cursor.up()
    }

    fn right(&mut self) {
        self.cursor.right()
    }
}

fn main() -> anyhow::Result<()> {
    let stdout = io::stdout().lock();
    let stdin = io::stdin().lock();
    let mut stdout = stdout.into_raw_mode().unwrap();

    write!(stdout, "{}", termion::clear::All)?;
    write!(stdout, "{}", termion::cursor::Hide)?;
    write!(stdout, "{}", termion::cursor::Goto(1, 1))?;

    let mut game = Game::new()?;

    print(&mut stdout, &game)?;
    stdout.flush()?;

    let mut keys = stdin.keys();
    loop {
        let b = keys.next().unwrap().unwrap();
        use termion::event::Key::*;
        match b {
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
