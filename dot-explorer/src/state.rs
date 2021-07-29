use crate::node::Node;
use anyhow::bail;

#[derive(Debug)]
pub struct State<'a> {
    selected_node_id: usize,
    edges: Vec<(usize, usize)>,
    nodes: Vec<&'a str>,
    to: Vec<Vec<usize>>,
    from: Vec<Vec<usize>>,
}

impl<'a> State<'a> {
    pub fn new(edges: Vec<(usize, usize)>, nodes: Vec<&'a str>, selected_node_id: usize) -> Self {
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

    pub fn select(&mut self, id: usize) -> anyhow::Result<()> {
        if id >= self.nodes.len() {
            bail!("ouf of range");
        }
        self.selected_node_id = id;
        Ok(())
    }

    pub fn selected(&self) -> Node {
        Node::new(self.selected_node_id, self.nodes[self.selected_node_id])
    }

    pub fn from(&self) -> Vec<Node> {
        self.from[self.selected_node_id]
            .iter()
            .copied()
            .map(|id| Node::new(id, self.nodes[id]))
            .collect()
    }

    pub fn to(&self) -> Vec<Node> {
        self.to[self.selected_node_id]
            .iter()
            .copied()
            .map(|id| Node::new(id, self.nodes[id]))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut state = State::new(vec![(0, 1), (0, 2), (1, 2)], vec!["N0", "N1", "N2"], 1);

        assert_eq!(state.selected(), Node::new(1, "N1"));
        assert_eq!(state.from(), vec![Node::new(0, "N0")]);
        assert_eq!(state.to(), vec![Node::new(2, "N2")]);

        assert_eq!(state.select(0).is_ok(), true);
        assert_eq!(state.selected(), Node::new(0, "N0"));
        assert_eq!(state.from(), vec![]);
        assert_eq!(state.to(), vec![Node::new(1, "N1"), Node::new(2, "N2")]);
    }
}
