#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Node {
    id: usize,
    label: &'static str,
}

impl Node {
    pub fn new(id: usize, label: &'static str) -> Self {
        Node { id, label }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.id, self.label)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let node = Node::new(1, "Hello");
        assert_eq!(node, Node::new(1, "Hello"));
        assert_eq!(node.to_string(), "[1] Hello");
    }
}
