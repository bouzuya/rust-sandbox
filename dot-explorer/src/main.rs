mod node;
mod state;

use crate::state::State;
use std::env;

fn main() {
    let mut args = env::args();
    let initial_node_label = args.nth(1).unwrap();

    let state = {
        let nodes = vec!["a", "b", "c"];
        let edges = vec![(0, 1), (0, 2), (1, 2)];
        let initial_node_id = nodes.iter().position(|n| n == &initial_node_label).unwrap();
        State::new(edges, nodes, initial_node_id)
    };

    render(&state);
}

fn render(state: &State) {
    println!("{}", state.selected());
    for node in state.from() {
        println!("{} -> {}", state.selected(), node);
    }
    for node in state.to() {
        println!("{} -> {}", node, state.selected());
    }
}
