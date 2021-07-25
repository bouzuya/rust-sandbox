use std::env;

#[derive(Debug)]
struct State {
    selected_node_id: usize,
    edges: Vec<(usize, usize)>,
    nodes: Vec<&'static str>,
    to: Vec<Vec<usize>>,
    from: Vec<Vec<usize>>,
}

impl State {
    fn new(edges: Vec<(usize, usize)>, nodes: Vec<&'static str>, selected_node_id: usize) -> Self {
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
        State {
            selected_node_id,
            edges,
            nodes,
            to,
            from,
        }
    }

    fn render(&self) {
        let nodes = &self.nodes;
        let selected_node_id = self.selected_node_id;

        println!("[{}] {}", selected_node_id, nodes[selected_node_id]);
        for node_id in self.from[selected_node_id].iter().copied() {
            println!(
                "[{}] {} -> [{}] {}",
                node_id, nodes[node_id], selected_node_id, nodes[selected_node_id]
            );
        }
        for node_id in self.to[selected_node_id].iter().copied() {
            println!(
                "[{}] {} -> [{}] {}",
                selected_node_id, nodes[selected_node_id], node_id, nodes[node_id]
            );
        }
    }
}

fn main() {
    let mut args = env::args();
    let initial_node_label = args.nth(1).unwrap();

    let state = {
        let nodes = vec!["a", "b", "c"];
        let edges = vec![(0, 1), (0, 2), (1, 2)];
        let initial_node_id = nodes.iter().position(|n| n == &initial_node_label).unwrap();
        State::new(edges, nodes, initial_node_id)
    };

    state.render();
}
