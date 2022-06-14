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
    for i in 0..h {
        for j in 0..w {
            print!("{}", area.pipe(Point::new(j, i)));
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
