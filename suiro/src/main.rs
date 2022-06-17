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
        |stdout: &mut RawTerminal<StdoutLock>, area: &Area, cursor: Cursor| -> anyhow::Result<()> {
            write!(
                stdout,
                "{}",
                termion::cursor::Goto(1 + u16::from(cursor.x) * 2, 1 + u16::from(cursor.y))
            )?;
            let _ = stdout.write(format!(" {} ", area.pipe(cursor.into())).as_bytes())?;
            Ok(())
        };

    let redraw_cursor =
        |stdout: &mut RawTerminal<StdoutLock>, area: &Area, cursor: Cursor| -> anyhow::Result<()> {
            write!(
                stdout,
                "{}",
                termion::cursor::Goto(1 + u16::from(cursor.x) * 2, 1 + u16::from(cursor.y))
            )?;
            let _ = stdout.write(format!("[{}]", area.pipe(cursor.into())).as_bytes())?;
            Ok(())
        };

    let stdout = io::stdout().lock();
    let stdin = io::stdin().lock();
    let mut stdout = stdout.into_raw_mode().unwrap();

    let mut cursor = Cursor::new(0, 0);

    write!(stdout, "{}", termion::clear::All)?;

    write!(stdout, "{}", termion::cursor::Hide)?;
    write!(stdout, "{}", termion::cursor::Goto(1, 1))?;
    let mut area = Area::new(2, 2, vec![Pipe::I(1), Pipe::L(0), Pipe::T(0), Pipe::L(0)])?;
    print(&mut stdout, &area)?;
    clear_cursor(&mut stdout, &area, cursor)?;
    redraw_cursor(&mut stdout, &area, cursor)?;

    stdout.flush()?;

    let mut keys = stdin.keys();
    loop {
        let b = keys.next().unwrap().unwrap();
        use termion::event::Key::*;
        match b {
            Char(' ') => {
                clear_cursor(&mut stdout, &area, cursor)?;
                area.rotate(cursor.into());
                redraw_cursor(&mut stdout, &area, cursor)?;
            }
            Char('h') => {
                clear_cursor(&mut stdout, &area, cursor)?;
                if cursor.x > 0 {
                    cursor.x -= 1
                }
                redraw_cursor(&mut stdout, &area, cursor)?;
            }
            Char('j') => {
                clear_cursor(&mut stdout, &area, cursor)?;
                if cursor.y < area.height() - 1 {
                    cursor.y += 1
                }
                redraw_cursor(&mut stdout, &area, cursor)?;
            }
            Char('k') => {
                clear_cursor(&mut stdout, &area, cursor)?;
                if cursor.y > 0 {
                    cursor.y -= 1
                }
                redraw_cursor(&mut stdout, &area, cursor)?;
            }
            Char('l') => {
                clear_cursor(&mut stdout, &area, cursor)?;
                if cursor.x < area.width() - 1 {
                    cursor.x += 1
                }
                redraw_cursor(&mut stdout, &area, cursor)?;
            }
            Char('q') => break,
            _ => {}
        }
        stdout.flush()?;
    }

    write!(stdout, "{}", termion::cursor::Show)?;
    Ok(())
}
