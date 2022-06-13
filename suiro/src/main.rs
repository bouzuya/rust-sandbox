mod block;

use block::Block;

fn print_board(width: usize, height: usize, blocks: &[Block]) {
    for i in 0..height {
        for j in 0..width {
            print!("{}", blocks[i * width + j]);
        }
        println!();
    }
}

fn main() {
    let width = 2;
    let height = 2;
    let mut blocks = vec![Block::I(1), Block::L(0), Block::T(0), Block::L(0)];
    print_board(width, height, &blocks);
    println!();

    let operations = vec![(0, 1), (0, 1)];
    for o in operations.iter().copied() {
        let p = o.0 * width + o.1;
        blocks[p] = blocks[p].rotate();
        print_board(width, height, &blocks);
        println!();
    }
}
