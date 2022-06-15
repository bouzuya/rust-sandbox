mod area;
mod direction;
mod pipe;
mod point;

use self::area::Area;
use self::pipe::Pipe;
use self::point::Point;

fn print(area: &Area) {
    let w = area.width();
    let h = area.height();
    let (_, flow) = area.test();
    for y in 0..h {
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
            print!("{}", c);
        }
        println!();
    }
    println!();
}

fn main() -> anyhow::Result<()> {
    let mut area = Area::new(2, 2, vec![Pipe::I(1), Pipe::L(0), Pipe::T(0), Pipe::L(0)])?;
    print(&area);

    let operations = vec![Point::new(1, 0), Point::new(1, 0)];
    for o in operations.iter().copied() {
        area.rotate(o);
        print(&area);
    }
    Ok(())
}
