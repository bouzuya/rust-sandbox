mod area;
mod pipe;

use self::area::Area;
use self::pipe::Pipe;

fn main() -> anyhow::Result<()> {
    let mut area = Area::new(2, 2, vec![Pipe::I(1), Pipe::L(0), Pipe::T(0), Pipe::L(0)])?;
    area.print();
    println!();

    let operations = vec![(0, 1), (0, 1)];
    for o in operations.iter().copied() {
        area.rotate(o.1, o.0);
        area.print();
        println!();
    }
    Ok(())
}
