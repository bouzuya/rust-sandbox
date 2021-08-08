use std::collections::BTreeSet;

pub type AttrList = Vec<(String, String)>;

#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct Graph {
    name: Option<String>,
    directed: Option<bool>,
    statements: Vec<Statement>,
    nodes: BTreeSet<String>,
    edges: Vec<(String, String, AttrList)>,
}

impl Graph {
    pub fn directed(name: Option<String>, statements: Vec<Statement>) -> Self {
        Self::new(Some(true), name, statements)
    }

    pub fn new(directed: Option<bool>, name: Option<String>, statements: Vec<Statement>) -> Self {
        let mut nodes = BTreeSet::new();
        let mut edges = vec![];
        for x in statements.clone() {
            match x {
                Statement::Node(s, _) => {
                    nodes.insert(s);
                }
                Statement::Edge(l, r, a) => match (l, r) {
                    (Either::Left(l), Either::Left(r)) => {
                        nodes.insert(l.clone());
                        nodes.insert(r.clone());
                        edges.push((l, r, a));
                    }
                    (Either::Left(l), Either::Right(rg)) => {
                        nodes.insert(l.clone());
                        for n in rg.nodes() {
                            nodes.insert(n.clone());
                            edges.push((l.clone(), n.clone(), vec![])); // TODO
                        }
                        for e in rg.edges() {
                            edges.push(e);
                        }
                    }
                    (Either::Right(lg), Either::Left(r)) => {
                        nodes.insert(r.clone());
                        for n in lg.nodes() {
                            nodes.insert(n.clone());
                            edges.push((n.clone(), r.clone(), vec![])); // TODO
                        }
                        for e in lg.edges() {
                            edges.push(e);
                        }
                    }
                    (Either::Right(lg), Either::Right(rg)) => {
                        for l in lg.nodes() {
                            nodes.insert(l.clone());
                            for r in rg.nodes() {
                                nodes.insert(r.clone());
                                edges.push((l.clone(), r.clone(), vec![])); // TODO
                            }
                        }
                        for e in lg.edges() {
                            edges.push(e);
                        }
                        for e in rg.edges() {
                            edges.push(e);
                        }
                    }
                },
                Statement::Attr(_, _) => {}
                Statement::IDeqID(_, _) => {}
                Statement::Subgraph(g) => {
                    for n in g.nodes() {
                        nodes.insert(n);
                    }
                    for e in g.edges() {
                        edges.push(e);
                    }
                }
            }
        }
        Self {
            name,
            directed,
            statements,
            nodes,
            edges,
        }
    }

    pub fn subgraph(name: Option<String>, statements: Vec<Statement>) -> Self {
        Self::new(None, name, statements)
    }

    pub fn undirected(name: Option<String>, statements: Vec<Statement>) -> Self {
        Self::new(Some(false), name, statements)
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn nodes(&self) -> BTreeSet<String> {
        self.nodes.clone()
    }

    pub fn edges(&self) -> Vec<(String, String, AttrList)> {
        self.edges.clone()
    }

    pub fn statements(&self) -> Vec<Statement> {
        self.statements.clone()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Statement {
    Node(String, AttrList),
    Edge(Either<String, Graph>, Either<String, Graph>, AttrList),
    Attr(String, AttrList),
    IDeqID(String, String),
    Subgraph(Graph),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let g = Graph::default();
        assert_eq!(g.name(), None);
        assert_eq!(g.nodes(), BTreeSet::new());
        assert_eq!(g.edges(), vec![]);
        assert_eq!(g.statements(), vec![]);
    }

    #[test]
    fn test_new_flat() {
        assert_eq!(Graph::new(None, None, vec![]), Graph::default());
        let g = Graph::new(None, Some("name1".to_string()), vec![]);
        assert_eq!(g.name(), Some("name1"));
        let g = Graph::new(None, None, vec![Statement::Node("N".to_string(), vec![])]);
        assert_eq!(g.nodes(), {
            let mut set = BTreeSet::new();
            set.insert("N".to_string());
            set
        });
        let g = Graph::new(
            None,
            None,
            vec![
                Statement::Node("N1".to_string(), vec![]),
                Statement::Node("N2".to_string(), vec![]),
            ],
        );
        assert_eq!(g.nodes(), {
            let mut set = BTreeSet::new();
            set.insert("N1".to_string());
            set.insert("N2".to_string());
            set
        });
        let g = Graph::new(
            None,
            None,
            vec![Statement::Edge(
                Either::Left("N1".to_string()),
                Either::Left("N2".to_string()),
                vec![],
            )],
        );
        assert_eq!(g.nodes(), {
            let mut set = BTreeSet::new();
            set.insert("N1".to_string());
            set.insert("N2".to_string());
            set
        });
        assert_eq!(
            g.edges(),
            vec![("N1".to_string(), "N2".to_string(), vec![])]
        );
    }

    #[test]
    fn test_new_nested() {
        // graph {
        //   {
        //     N1
        //   }
        // }
        let g = Graph::undirected(
            None,
            vec![Statement::Subgraph(Graph::subgraph(
                None,
                vec![Statement::Node("N1".to_string(), vec![])],
            ))],
        );
        assert_eq!(g.nodes(), {
            let mut set = BTreeSet::new();
            set.insert("N1".to_string());
            set
        });

        // digraph {
        //   {
        //     N1 -> N2
        //   }
        // }
        // N1 -> N2
        let g = Graph::directed(
            None,
            vec![Statement::Subgraph(Graph::subgraph(
                None,
                vec![Statement::Edge(
                    Either::Left("N1".to_string()),
                    Either::Left("N2".to_string()),
                    vec![],
                )],
            ))],
        );
        assert_eq!(g.nodes(), {
            let mut set = BTreeSet::new();
            set.insert("N1".to_string());
            set.insert("N2".to_string());
            set
        });
        assert_eq!(
            g.edges(),
            vec![("N1".to_string(), "N2".to_string(), vec![])]
        );

        // digraph {
        //   N1 -> {
        //     N2
        //     N3
        //   }
        // }
        // N1 -> N2
        // N1 -> N3
        let g = Graph::directed(
            None,
            vec![Statement::Edge(
                Either::Left("N1".to_string()),
                Either::Right(Graph::subgraph(
                    None,
                    vec![
                        Statement::Node("N2".to_string(), vec![]),
                        Statement::Node("N3".to_string(), vec![]),
                    ],
                )),
                vec![],
            )],
        );
        assert_eq!(g.nodes(), {
            let mut set = BTreeSet::new();
            set.insert("N1".to_string());
            set.insert("N2".to_string());
            set.insert("N3".to_string());
            set
        });
        assert_eq!(
            g.edges(),
            vec![
                ("N1".to_string(), "N2".to_string(), vec![]),
                ("N1".to_string(), "N3".to_string(), vec![]),
            ]
        );

        // digraph {
        //   N1 -> {
        //     N2 -> N3
        //   }
        // }
        // N1 -> N2
        // N1 -> N3
        // N2 -> N3
        let g = Graph::directed(
            None,
            vec![Statement::Edge(
                Either::Left("N1".to_string()),
                Either::Right(Graph::subgraph(
                    None,
                    vec![Statement::Edge(
                        Either::Left("N2".to_string()),
                        Either::Left("N3".to_string()),
                        vec![],
                    )],
                )),
                vec![],
            )],
        );
        assert_eq!(g.nodes(), {
            let mut set = BTreeSet::new();
            set.insert("N1".to_string());
            set.insert("N2".to_string());
            set.insert("N3".to_string());
            set
        });
        assert_eq!(
            g.edges(),
            vec![
                ("N1".to_string(), "N2".to_string(), vec![]),
                ("N1".to_string(), "N3".to_string(), vec![]),
                ("N2".to_string(), "N3".to_string(), vec![])
            ]
        );

        // digraph {
        //   N1 -> {
        //     N2 -> {
        //       N3
        //     }
        //   }
        // }
        // N1 -> N2
        // N1 -> N3
        // N2 -> N3
        let g = Graph::directed(
            None,
            vec![Statement::Edge(
                Either::Left("N1".to_string()),
                Either::Right(Graph::new(
                    None,
                    None,
                    vec![Statement::Edge(
                        Either::Left("N2".to_string()),
                        Either::Right(Graph::new(
                            None,
                            None,
                            vec![Statement::Node("N3".to_string(), vec![])],
                        )),
                        vec![],
                    )],
                )),
                vec![],
            )],
        );
        assert_eq!(g.nodes(), {
            let mut set = BTreeSet::new();
            set.insert("N1".to_string());
            set.insert("N2".to_string());
            set.insert("N3".to_string());
            set
        });
        assert_eq!(
            g.edges(),
            vec![
                ("N1".to_string(), "N2".to_string(), vec![]),
                ("N1".to_string(), "N3".to_string(), vec![]),
                ("N2".to_string(), "N3".to_string(), vec![])
            ]
        );

        // digraph {
        //   {
        //     N1
        //     N2
        //   } -> N3
        // }
        // N1 -> N3
        // N2 -> N3
        let g = Graph::directed(
            None,
            vec![Statement::Edge(
                Either::Right(Graph::subgraph(
                    None,
                    vec![
                        Statement::Node("N1".to_string(), vec![]),
                        Statement::Node("N2".to_string(), vec![]),
                    ],
                )),
                Either::Left("N3".to_string()),
                vec![],
            )],
        );
        assert_eq!(g.nodes(), {
            let mut set = BTreeSet::new();
            set.insert("N1".to_string());
            set.insert("N2".to_string());
            set.insert("N3".to_string());
            set
        });
        assert_eq!(
            g.edges(),
            vec![
                ("N1".to_string(), "N3".to_string(), vec![]),
                ("N2".to_string(), "N3".to_string(), vec![]),
            ]
        );

        // digraph {
        //   {
        //     N1 -> N2
        //   } -> N3
        // }
        // N1 -> N2
        // N1 -> N3
        // N2 -> N3
        let g = Graph::directed(
            None,
            vec![Statement::Edge(
                Either::Right(Graph::subgraph(
                    None,
                    vec![Statement::Edge(
                        Either::Left("N1".to_string()),
                        Either::Left("N2".to_string()),
                        vec![],
                    )],
                )),
                Either::Left("N3".to_string()),
                vec![],
            )],
        );
        assert_eq!(g.nodes(), {
            let mut set = BTreeSet::new();
            set.insert("N1".to_string());
            set.insert("N2".to_string());
            set.insert("N3".to_string());
            set
        });
        assert_eq!(
            g.edges(),
            vec![
                ("N1".to_string(), "N3".to_string(), vec![]),
                ("N2".to_string(), "N3".to_string(), vec![]),
                ("N1".to_string(), "N2".to_string(), vec![]),
            ]
        );

        // digraph {
        //   {
        //     N1
        //     N2
        //   } -> {
        //     N3
        //     N4
        //   }
        // }
        // N1 -> N3
        // N1 -> N4
        // N2 -> N3
        // N2 -> N4
        let g = Graph::directed(
            None,
            vec![Statement::Edge(
                Either::Right(Graph::subgraph(
                    None,
                    vec![
                        Statement::Node("N1".to_string(), vec![]),
                        Statement::Node("N2".to_string(), vec![]),
                    ],
                )),
                Either::Right(Graph::subgraph(
                    None,
                    vec![
                        Statement::Node("N3".to_string(), vec![]),
                        Statement::Node("N4".to_string(), vec![]),
                    ],
                )),
                vec![],
            )],
        );
        assert_eq!(g.nodes(), {
            let mut set = BTreeSet::new();
            set.insert("N1".to_string());
            set.insert("N2".to_string());
            set.insert("N3".to_string());
            set.insert("N4".to_string());
            set
        });
        assert_eq!(
            g.edges(),
            vec![
                ("N1".to_string(), "N3".to_string(), vec![]),
                ("N1".to_string(), "N4".to_string(), vec![]),
                ("N2".to_string(), "N3".to_string(), vec![]),
                ("N2".to_string(), "N4".to_string(), vec![]),
            ]
        );

        // digraph {
        //   {
        //     N1 -> N2
        //   } -> {
        //     N3
        //     N4
        //   }
        // }
        // N1 -> N2
        // N1 -> N3
        // N1 -> N4
        // N2 -> N3
        // N2 -> N4
        let g = Graph::directed(
            None,
            vec![Statement::Edge(
                Either::Right(Graph::subgraph(
                    None,
                    vec![Statement::Edge(
                        Either::Left("N1".to_string()),
                        Either::Left("N2".to_string()),
                        vec![],
                    )],
                )),
                Either::Right(Graph::subgraph(
                    None,
                    vec![
                        Statement::Node("N3".to_string(), vec![]),
                        Statement::Node("N4".to_string(), vec![]),
                    ],
                )),
                vec![],
            )],
        );
        assert_eq!(g.nodes(), {
            let mut set = BTreeSet::new();
            set.insert("N1".to_string());
            set.insert("N2".to_string());
            set.insert("N3".to_string());
            set.insert("N4".to_string());
            set
        });
        assert_eq!(
            g.edges(),
            vec![
                ("N1".to_string(), "N3".to_string(), vec![]),
                ("N1".to_string(), "N4".to_string(), vec![]),
                ("N2".to_string(), "N3".to_string(), vec![]),
                ("N2".to_string(), "N4".to_string(), vec![]),
                ("N1".to_string(), "N2".to_string(), vec![]),
            ]
        );

        // digraph {
        //   {
        //     N1 -> N2
        //   } -> {
        //     N3 -> N4
        //   }
        // }
        // N1 -> N2
        // N1 -> N3
        // N1 -> N4
        // N2 -> N3
        // N2 -> N4
        // N3 -> N4
        let g = Graph::directed(
            None,
            vec![Statement::Edge(
                Either::Right(Graph::subgraph(
                    None,
                    vec![Statement::Edge(
                        Either::Left("N1".to_string()),
                        Either::Left("N2".to_string()),
                        vec![],
                    )],
                )),
                Either::Right(Graph::subgraph(
                    None,
                    vec![Statement::Edge(
                        Either::Left("N3".to_string()),
                        Either::Left("N4".to_string()),
                        vec![],
                    )],
                )),
                vec![],
            )],
        );
        assert_eq!(g.nodes(), {
            let mut set = BTreeSet::new();
            set.insert("N1".to_string());
            set.insert("N2".to_string());
            set.insert("N3".to_string());
            set.insert("N4".to_string());
            set
        });
        assert_eq!(
            g.edges(),
            vec![
                ("N1".to_string(), "N3".to_string(), vec![]),
                ("N1".to_string(), "N4".to_string(), vec![]),
                ("N2".to_string(), "N3".to_string(), vec![]),
                ("N2".to_string(), "N4".to_string(), vec![]),
                ("N1".to_string(), "N2".to_string(), vec![]),
                ("N3".to_string(), "N4".to_string(), vec![]),
            ]
        );
    }
}
