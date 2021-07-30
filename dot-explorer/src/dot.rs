use anyhow::anyhow;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, char, multispace0, multispace1},
    combinator::{all_consuming, map, recognize},
    multi::{many0, separated_list0},
    sequence::{pair, tuple},
    IResult,
};

#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct Graph<'a> {
    pub nodes: Vec<&'a str>,
    pub edges: Vec<(&'a str, &'a str)>,
}

#[derive(Debug, Eq, PartialEq)]
enum Statement<'a> {
    Node(&'a str),
    Edge((&'a str, &'a str)),
}

pub fn parse(s: &str) -> anyhow::Result<Graph> {
    all_consuming(graph)(s)
        .map(|(_, v)| v)
        .map_err(|_| anyhow!("parse error"))
}

fn graph(s: &str) -> IResult<&str, Graph> {
    map(
        tuple((
            multispace0,
            tag("digraph"),
            multispace1,
            char('{'),
            stmt_list,
            char('}'),
            multispace0,
        )),
        |(_, _, _, _, s, _, _)| {
            s.into_iter().fold(Graph::default(), |mut g, x| {
                match x {
                    Statement::Node(s) => g.nodes.push(s),
                    Statement::Edge((l, r)) => g.edges.push((l, r)),
                }
                g
            })
        },
    )(s)
}

fn stmt_list(s: &str) -> IResult<&str, Vec<Statement>> {
    separated_list0(
        alt((
            map(tuple((multispace0, tag(";"), multispace0)), |(_, x, _)| x),
            multispace1,
        )),
        stmt,
    )(s)
}

fn stmt(s: &str) -> IResult<&str, Statement> {
    alt((
        map(edge_stmt, |(l, r): (&str, &str)| Statement::Edge((l, r))),
        map(node_stmt, |node: &str| Statement::Node(node)),
    ))(s)
}

fn node_stmt(s: &str) -> IResult<&str, &str> {
    node_id(s)
}

fn edge_stmt(s: &str) -> IResult<&str, (&str, &str)> {
    map(tuple((node_id, multispace1, edge_rhs)), |(l, _, r)| (l, r))(s)
}

fn edge_rhs(s: &str) -> IResult<&str, &str> {
    map(tuple((edgeop, multispace1, node_id)), |(_, _, r)| r)(s)
}

fn edgeop(s: &str) -> IResult<&str, &str> {
    alt((tag("->"), tag("--")))(s)
}

fn node_id(s: &str) -> IResult<&str, &str> {
    id(s)
}

fn id(s: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0(alt((alphanumeric1, tag("_")))),
    ))(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn graph_test() {
        assert_eq!(graph("digraph{}").is_err(), true);
        assert_eq!(graph("digraph {}"), Ok(("", Graph::default())));
        assert_eq!(
            graph("digraph {node}"),
            Ok((
                "",
                Graph {
                    nodes: vec!["node"],
                    edges: vec![]
                }
            ))
        );
        assert_eq!(
            graph("digraph {n1 n2 n3 -> n4}"),
            Ok((
                "",
                Graph {
                    nodes: vec!["n1", "n2"],
                    edges: vec![("n3", "n4")]
                }
            ))
        );
    }

    #[test]
    fn stmt_list_test() {
        assert_eq!(stmt_list(""), Ok(("", vec![])));
        assert_eq!(stmt_list("N1"), Ok(("", vec![Statement::Node("N1")])));
        assert_eq!(
            stmt_list("N1 -> N2"),
            Ok(("", vec![Statement::Edge(("N1", "N2"))]))
        );
        assert_eq!(
            stmt_list("N1 -- N2"),
            Ok(("", vec![Statement::Edge(("N1", "N2"))]))
        );
        assert_eq!(
            stmt_list("N1 N2"),
            Ok(("", vec![Statement::Node("N1"), Statement::Node("N2")],))
        );
        assert_eq!(
            stmt_list("N1 N2 -> N3 N4"),
            Ok((
                "",
                vec![
                    Statement::Node("N1"),
                    Statement::Edge(("N2", "N3")),
                    Statement::Node("N4"),
                ]
            ))
        );
        assert_eq!(
            stmt_list("N1;N2"),
            Ok(("", vec![Statement::Node("N1"), Statement::Node("N2")],))
        );
        assert_eq!(
            stmt_list("N1 ; N2"),
            Ok(("", vec![Statement::Node("N1"), Statement::Node("N2")],))
        );
    }

    #[test]
    fn stmt_test() {
        assert_eq!(stmt("N1"), Ok(("", Statement::Node("N1"))));
        assert_eq!(stmt("N1 -> N2"), Ok(("", Statement::Edge(("N1", "N2")))));
        assert_eq!(stmt("N1 -- N2"), Ok(("", Statement::Edge(("N1", "N2")))));
    }

    #[test]
    fn node_stmt_test() {
        assert_eq!(node_stmt("N1"), Ok(("", "N1")));
    }

    #[test]
    fn edge_stmt_test() {
        assert_eq!(edge_stmt("N1 -> N2"), Ok(("", ("N1", "N2"))));
        assert_eq!(edge_stmt("N1 -- N2"), Ok(("", ("N1", "N2"))));
    }

    #[test]
    fn node_id_test() {
        assert_eq!(node_id("N1"), Ok(("", "N1")));
    }

    #[test]
    fn id_test() {
        assert_eq!(id("node"), Ok(("", "node")));
        assert_eq!(id("NODE"), Ok(("", "NODE")));
        assert_eq!(id("_"), Ok(("", "_")));
        assert_eq!(id("N0123456789"), Ok(("", "N0123456789")));
        assert_eq!(id("0123456789").is_err(), true);
    }
}
