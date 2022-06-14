mod area;
mod block;

use self::area::Area;
use self::block::Block;

fn main() -> anyhow::Result<()> {
    let mut area = Area::new(
        2,
        2,
        vec![Block::I(1), Block::L(0), Block::T(0), Block::L(0)],
    )?;
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
