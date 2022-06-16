mod area;
mod direction;
mod pipe;
mod point;

use self::area::Area;
use self::pipe::Pipe;
use self::point::Point;

use std::{
    hash::Hasher,
    io::{self, Stdout, StdoutLock, Write},
    thread,
    time::Duration,
};
use termion::{
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
};

fn print(stdout: &mut StdoutLock, area: &Area) -> anyhow::Result<()> {
    let w = area.width();
    let h = area.height();
    let (_, flow) = area.test();
    for y in 0..h {
        write!(stdout, "{}", termion::cursor::Goto(1, 1 + u16::from(y)))?;
        let _ = stdout.write(" ".as_bytes())?;
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
            let _ = stdout.write(format!("{} ", c).as_bytes())?;
        }
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let clear_cursor =
        |stdout: &mut RawTerminal<StdoutLock>, area: &Area, x: u8, y: u8| -> anyhow::Result<()> {
            write!(
                stdout,
                "{}",
                termion::cursor::Goto(1 + u16::from(x) * 2, 1 + u16::from(y))
            )?;
            let _ = stdout.write(format!(" {} ", area.pipe(Point::new(x, y))).as_bytes())?;
            Ok(())
        };

    let redraw_cursor =
        |stdout: &mut RawTerminal<StdoutLock>, area: &Area, x: u8, y: u8| -> anyhow::Result<()> {
            write!(
                stdout,
                "{}",
                termion::cursor::Goto(1 + u16::from(x) * 2, 1 + u16::from(y))
            )?;
            let _ = stdout.write(format!("[{}]", area.pipe(Point::new(x, y))).as_bytes())?;
            Ok(())
        };

    let stdout = io::stdout().lock();
    let stdin = io::stdin().lock();
    let mut stdout = stdout.into_raw_mode().unwrap();

    let mut x = 0_u8;
    let mut y = 0_u8;

    write!(stdout, "{}", termion::clear::All)?;

    write!(stdout, "{}", termion::cursor::Hide)?;
    write!(stdout, "{}", termion::cursor::Goto(1, 1))?;
    let mut area = Area::new(2, 2, vec![Pipe::I(1), Pipe::L(0), Pipe::T(0), Pipe::L(0)])?;
    print(&mut stdout, &area)?;
    clear_cursor(&mut stdout, &area, x, y)?;
    redraw_cursor(&mut stdout, &area, x, y)?;

    stdout.flush()?;

    let mut keys = stdin.keys();
    loop {
        let b = keys.next().unwrap().unwrap();
        use termion::event::Key::*;
        match b {
            Char(' ') => {
                clear_cursor(&mut stdout, &area, x, y)?;
                area.rotate(Point::new(x, y));
                redraw_cursor(&mut stdout, &area, x, y)?;
            }
            Char('h') => {
                clear_cursor(&mut stdout, &area, x, y)?;
                if x > 0 {
                    x -= 1
                }
                redraw_cursor(&mut stdout, &area, x, y)?;
            }
            Char('j') => {
                clear_cursor(&mut stdout, &area, x, y)?;
                if y < area.height() - 1 {
                    y += 1
                }
                redraw_cursor(&mut stdout, &area, x, y)?;
            }
            Char('k') => {
                clear_cursor(&mut stdout, &area, x, y)?;
                if y > 0 {
                    y -= 1
                }
                redraw_cursor(&mut stdout, &area, x, y)?;
            }
            Char('l') => {
                clear_cursor(&mut stdout, &area, x, y)?;
                if x < area.width() - 1 {
                    x += 1
                }
                redraw_cursor(&mut stdout, &area, x, y)?;
            }
            Char('q') => break,
            _ => {}
        }
        stdout.flush()?;
    }

    Ok(())
}
