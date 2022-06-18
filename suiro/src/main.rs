mod area;
mod cursor;
mod direction;
mod pipe;
mod point;

use self::area::Area;
use self::pipe::Pipe;
use self::point::Point;

use cursor::Cursor;
use std::io::{self, StdoutLock, Write};
use termion::{input::TermRead, raw::IntoRawMode};

fn print(stdout: &mut StdoutLock, area: &Area, cursor: Cursor, count: usize) -> anyhow::Result<()> {
    let w = area.width();
    let h = area.height();
    let (ng, flow) = area.test();
    write!(stdout, "{}", termion::cursor::Goto(1, 1))?;
    write!(stdout, " count: {}", count)?;
    for y in 0..h {
        write!(stdout, "{}", termion::cursor::Goto(1, 2 + u16::from(y)))?;

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
                    "{}",
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
                    }
                )
            } else {
                format!("{}", p)
            };
            let _ = stdout.write(
                format!(
                    "{}{}{}{}",
                    if ng[usize::from(y) * usize::from(w) + usize::from(x)] {
                        termion::color::Fg(termion::color::Rgb(255, 0, 0)).to_string()
                    } else {
                        String::default()
                    },
                    c,
                    if ng[usize::from(y) * usize::from(w) + usize::from(x)] {
                        termion::color::Fg(termion::color::Reset).to_string()
                    } else {
                        String::default()
                    },
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
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let stdout = io::stdout().lock();
    let stdin = io::stdin().lock();
    let mut stdout = stdout.into_raw_mode().unwrap();

    let mut count = 0_usize;
    let mut cursor = Cursor::new(0, 0);
    // let mut area = Area::new(2, 2, vec![Pipe::I(1), Pipe::L(0), Pipe::T(0), Pipe::L(0)])?;
    let mut area = Area::new(
        24,
        24,
        (0..24 * 24)
            .into_iter()
            .map(|_| Pipe::I(1))
            .collect::<Vec<Pipe>>(),
    )?;

    write!(stdout, "{}", termion::clear::All)?;
    write!(stdout, "{}", termion::cursor::Hide)?;
    write!(stdout, "{}", termion::cursor::Goto(1, 1))?;

    print(&mut stdout, &area, cursor, count)?;
    stdout.flush()?;

    let mut keys = stdin.keys();
    loop {
        let b = keys.next().unwrap().unwrap();
        use termion::event::Key::*;
        match b {
            Char(' ') => {
                area.rotate(cursor.into());
                count += 1;
            }
            Char('h') => {
                if cursor.x > 0 {
                    cursor.x -= 1
                }
            }
            Char('j') => {
                if cursor.y < area.height() - 1 {
                    cursor.y += 1
                }
            }
            Char('k') => {
                if cursor.y > 0 {
                    cursor.y -= 1
                }
            }
            Char('l') => {
                if cursor.x < area.width() - 1 {
                    cursor.x += 1
                }
            }
            Char('q') => break,
            _ => {}
        }
        print(&mut stdout, &area, cursor, count)?;
        stdout.flush()?;
    }

    write!(stdout, "{}", termion::cursor::Show)?;
    Ok(())
}
