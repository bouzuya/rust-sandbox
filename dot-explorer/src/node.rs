#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Node<'a> {
    id: usize,
    label: &'a str,
}

impl<'a> Node<'a> {
    pub fn new(id: usize, label: &'a str) -> Self {
        Node { id, label }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

impl<'a> std::fmt::Display for Node<'a> {
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
        assert_eq!(node.id(), 1);
        assert_eq!(node.to_string(), "[1] Hello");
    }
}
