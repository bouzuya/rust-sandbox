mod area;
mod pipe;
mod point;

use self::area::Area;
use self::pipe::Pipe;
use self::point::Point;

fn main() -> anyhow::Result<()> {
    let mut area = Area::new(2, 2, vec![Pipe::I(1), Pipe::L(0), Pipe::T(0), Pipe::L(0)])?;
    area.print();
    println!();

    let operations = vec![Point::new(1, 0), Point::new(1, 0)];
    for o in operations.iter().copied() {
        area.rotate(o);
        area.print();
        println!();
    }
    Ok(())
}
