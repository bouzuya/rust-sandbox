use anyhow::anyhow;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{alpha1, alphanumeric1, anychar, char, multispace0, multispace1},
    combinator::{all_consuming, map, opt, recognize},
    multi::{fold_many0, many0},
    sequence::{delimited, pair, tuple},
    IResult,
};

#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct Graph {
    pub nodes: Vec<String>,
    pub edges: Vec<(String, String)>,
}

#[derive(Debug, Eq, PartialEq)]
enum Statement {
    Node(String),
    Edge((String, String)),
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
            alt((tag("graph"), tag("digraph"))),
            opt(map(tuple((multispace1, id)), |(_, x)| x)),
            multispace0,
            char('{'),
            multispace0,
            stmt_list,
            multispace0,
            char('}'),
            multispace0,
        )),
        |(_, _graph, _id, _, _, _, s, _, _, _)| {
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
    map(
        opt(tuple((
            multispace0,
            stmt,
            multispace0,
            opt(tuple((tag(";"), multispace0))),
            stmt_list,
        ))),
        |r| match r {
            None => vec![],
            Some((_, x, _, _, mut xs)) => {
                let mut ys = vec![x];
                ys.append(&mut xs);
                ys
            }
        },
    )(s)
}

fn stmt(s: &str) -> IResult<&str, Statement> {
    alt((edge_stmt, node_stmt))(s)
}

fn node_stmt(s: &str) -> IResult<&str, Statement> {
    map(node_id, Statement::Node)(s)
}

fn edge_stmt(s: &str) -> IResult<&str, Statement> {
    map(tuple((node_id, multispace1, edge_rhs)), |(l, _, r)| {
        Statement::Edge((l, r))
    })(s)
}

fn edge_rhs(s: &str) -> IResult<&str, String> {
    map(tuple((edgeop, multispace1, node_id)), |(_, _, r)| r)(s)
}

fn edgeop(s: &str) -> IResult<&str, &str> {
    alt((tag("->"), tag("--")))(s)
}

fn node_id(s: &str) -> IResult<&str, String> {
    id(s)
}

fn id(s: &str) -> IResult<&str, String> {
    alt((map(id_string, |s| s.to_string()), id_double_quoted_string))(s)
}

fn id_string(s: &str) -> IResult<&str, &str> {
    // TODO: \x80-\xFF
    recognize(pair(
        alt((alpha1, underscore)),
        many0(alt((alphanumeric1, underscore))),
    ))(s)
}

fn underscore(s: &str) -> IResult<&str, &str> {
    tag("_")(s)
}

enum StringFragment<'a> {
    Literal(&'a str),
    EscapedChar(char),
}

fn id_double_quoted_string(s: &str) -> IResult<&str, String> {
    delimited(
        char('"'),
        fold_many0(
            alt((
                map(is_not(r#""\"#), StringFragment::Literal),
                map(tuple((char('\\'), anychar)), |(_, c)| {
                    StringFragment::EscapedChar(c)
                }),
            )),
            String::new(),
            |mut s, f| {
                match f {
                    StringFragment::Literal(t) => s.push_str(t),
                    StringFragment::EscapedChar(c) => {
                        if c != '"' {
                            s.push('\\');
                        }
                        s.push(c);
                    }
                }
                s
            },
        ),
        char('"'),
    )(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(name: &str) -> Statement {
        Statement::Node(name.to_string())
    }

    fn edge(l: &str, r: &str) -> Statement {
        Statement::Edge((l.to_string(), r.to_string()))
    }

    #[test]
    fn graph_test() {
        assert_eq!(graph("digraph{}"), Ok(("", Graph::default())));
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
        assert_eq!(
            graph("digraph {n1 n2 n3 -> n4}"),
            Ok((
                "",
                Graph {
                    nodes: vec!["n1".to_string(), "n2".to_string()],
                    edges: vec![("n3".to_string(), "n4".to_string())]
                }
            ))
        );
        assert_eq!(graph("graph {}"), Ok(("", Graph::default())));
        assert_eq!(graph("digraph example {}"), Ok(("", Graph::default())));
        assert_eq!(
            graph(r#"digraph "example graph" {}"#),
            Ok(("", Graph::default()))
        );
    }

    #[test]
    fn stmt_list_test() {
        // [ stmt [';'] stmt_list ]
        let f = |s| all_consuming(stmt_list)(s);
        assert_eq!(f(""), Ok(("", vec![])));
        assert_eq!(f("N1"), Ok(("", vec![node("N1")])));
        assert_eq!(f("N1 -> N2"), Ok(("", vec![edge("N1", "N2")])));
        assert_eq!(f("N1 -- N2"), Ok(("", vec![edge("N1", "N2")])));
        assert_eq!(f("N1 N2"), Ok(("", vec![node("N1"), node("N2")],)));
        assert_eq!(f("N1;N2"), Ok(("", vec![node("N1"), node("N2")],)));
        assert_eq!(f("N1 ; N2"), Ok(("", vec![node("N1"), node("N2")],)));
        assert_eq!(f("N1;N2;"), Ok(("", vec![node("N1"), node("N2")],)));
        assert_eq!(f("N1 ; N2 ; "), Ok(("", vec![node("N1"), node("N2")],)));
        assert_eq!(
            f("N1 N2 -> N3 N4"),
            Ok(("", vec![node("N1"), edge("N2", "N3"), node("N4"),]))
        );
        assert_eq!(f(";").is_err(), true);
    }

    #[test]
    fn stmt_test() {
        assert_eq!(stmt("N1"), Ok(("", node("N1"))));
        assert_eq!(stmt("N1 -> N2"), Ok(("", edge("N1", "N2"))));
        assert_eq!(stmt("N1 -- N2"), Ok(("", edge("N1", "N2"))));
    }

    #[test]
    fn node_stmt_test() {
        assert_eq!(node_stmt("N1"), Ok(("", node("N1"))));
    }

    #[test]
    fn edge_stmt_test() {
        assert_eq!(edge_stmt("N1 -> N2"), Ok(("", edge("N1", "N2"))));
        assert_eq!(edge_stmt("N1 -- N2"), Ok(("", edge("N1", "N2"))));
    }

    #[test]
    fn node_id_test() {
        assert_eq!(node_id("N1"), Ok(("", "N1".to_string())));
    }

    #[test]
    fn id_test() {
        let ok = |s: &str| Ok(("", s.to_string()));
        assert_eq!(id("node"), ok("node"));
        assert_eq!(id("NODE"), ok("NODE"));
        assert_eq!(id("_"), ok("_"));
        assert_eq!(id("N0123456789"), ok("N0123456789"));
        assert_eq!(id("0123456789").is_err(), true);
    }

    #[test]
    fn id_double_quoted_string_test() {
        let f = id_double_quoted_string;
        assert_eq!(f(r#""""#), Ok(("", r#""#.to_string())));
        assert_eq!(f(r#""abc""#), Ok(("", r#"abc"#.to_string())));
        assert_eq!(f(r#""abc def""#), Ok(("", r#"abc def"#.to_string())));
        assert_eq!(f(r#""abc\"def""#), Ok(("", r#"abc"def"#.to_string())));
        assert_eq!(f(r#""abc\\def""#), Ok(("", r#"abc\\def"#.to_string())));
        assert_eq!(f(r#""abc"def""#), Ok(("def\"", r#"abc"#.to_string())));
    }
}
