use anyhow::anyhow;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, char, multispace0, multispace1, satisfy},
    combinator::{all_consuming, map, recognize},
    multi::many0,
    sequence::{pair, tuple},
    IResult,
};

#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct Graph {
    pub nodes: Vec<String>,
    pub edges: Vec<(usize, usize)>,
}

pub fn parse(s: &str) -> anyhow::Result<Graph> {
    all_consuming(graph)(s)
        .map(|(_, v)| v)
        .map_err(|_| anyhow!("parse error"))
}

fn graph(s: &str) -> IResult<&str, Graph> {
    map(
        tuple((tag("digraph"), multispace1, char('{'), stmt_list, char('}'))),
        |(_, _, _, g, _)| g,
    )(s)
}

fn stmt_list(s: &str) -> IResult<&str, Graph> {
    alt((
        map(node_stmt, |node: &str| Graph {
            nodes: vec![node.to_string()],
            edges: vec![],
        }),
        map(multispace0, |_| Graph {
            nodes: vec![],
            edges: vec![],
        }),
    ))(s)
}

fn node_stmt(s: &str) -> IResult<&str, &str> {
    node_id(s)
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
                    nodes: vec!["node".to_string()],
                    edges: vec![]
                }
            ))
        );
    }

    #[test]
    fn node_stmt_test() {
        assert_eq!(id("N1"), Ok(("", "N1")));
    }

    #[test]
    fn node_id_test() {
        assert_eq!(id("N1"), Ok(("", "N1")));
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
