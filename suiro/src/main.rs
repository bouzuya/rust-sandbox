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

fn print(stdout: &mut StdoutLock, area: &Area, cursor: Cursor) -> anyhow::Result<()> {
    let w = area.width();
    let h = area.height();
    let (ng, flow) = area.test();
    for y in 0..h {
        write!(stdout, "{}", termion::cursor::Goto(1, 1 + u16::from(y)))?;

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
            if ng[usize::from(y) * usize::from(w) + usize::from(x)] {
                let _ = write!(
                    stdout,
                    "{}",
                    termion::color::Fg(termion::color::Rgb(255, 0, 0))
                )?;
            }
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
            if ng[usize::from(y) * usize::from(w) + usize::from(x)] {
                let _ = write!(stdout, "{}", termion::color::Fg(termion::color::Reset))?;
            }
        }
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let stdout = io::stdout().lock();
    let stdin = io::stdin().lock();
    let mut stdout = stdout.into_raw_mode().unwrap();

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

    print(&mut stdout, &area, cursor)?;
    stdout.flush()?;

    let mut keys = stdin.keys();
    loop {
        let b = keys.next().unwrap().unwrap();
        use termion::event::Key::*;
        match b {
            Char(' ') => {
                area.rotate(cursor.into());
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
        print(&mut stdout, &area, cursor)?;
        stdout.flush()?;
    }

    write!(stdout, "{}", termion::cursor::Show)?;
    Ok(())
}
