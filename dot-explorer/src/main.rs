use std::env;

fn main() {
    let mut args = env::args();
    let initial_node_label = args.nth(1).unwrap();

    let nodes = vec!["a", "b", "c"];
    let edges = vec![(0, 1), (0, 2), (1, 2)];
    let initial_node_id = nodes.iter().position(|n| n == &initial_node_label).unwrap();

    let to = {
        let mut e = vec![vec![]; nodes.len()];
        for (u, v) in edges.iter().copied() {
            e[u].push(v);
        }
        e
    };
    let from = {
        let mut e = vec![vec![]; nodes.len()];
        for (u, v) in edges.iter().copied() {
            e[v].push(u);
        }
        e
    };

    println!("[{}] {}", initial_node_id, nodes[initial_node_id]);
    for node_id in from[initial_node_id].iter().copied() {
        println!(
            "[{}] {} -> [{}] {}",
            node_id, nodes[node_id], initial_node_id, nodes[initial_node_id]
        );
    }
    for node_id in to[initial_node_id].iter().copied() {
        println!(
            "[{}] {} -> [{}] {}",
            initial_node_id, nodes[initial_node_id], node_id, nodes[node_id]
        );
    }
}
