use anyhow::anyhow;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, tag_no_case},
    character::complete::{alpha1, alphanumeric1, anychar, char, multispace0, one_of},
    combinator::{all_consuming, map, opt, recognize},
    error::ParseError,
    multi::{fold_many0, many0},
    sequence::{delimited, pair, tuple},
    IResult,
};

type AttrList = Vec<(String, String)>;

#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct Graph {
    pub nodes: Vec<(String, AttrList)>,
    pub edges: Vec<(String, String, AttrList)>,
}

#[derive(Debug, Eq, PartialEq)]
enum Statement {
    Node(String, AttrList),
    Edge(String, String, AttrList),
    Attr(String, AttrList),
    IDeqID(String, String),
}

pub fn parse(s: &str) -> anyhow::Result<Graph> {
    all_consuming(graph)(s)
        .map(|(_, v)| v)
        .map_err(|_| anyhow!("parse error"))
}

fn graph(s: &str) -> IResult<&str, Graph> {
    map(
        tuple((
            alt((ws(tag_no_case("graph")), ws(tag_no_case("digraph")))),
            opt(ws(id)),
            ws(char('{')),
            stmt_list,
            ws(char('}')),
        )),
        |(_graph, _id, _, s, _)| {
            s.into_iter().fold(Graph::default(), |mut g, x| {
                match x {
                    Statement::Node(s, a) => g.nodes.push((s, a)),
                    Statement::Edge(l, r, a) => g.edges.push((l, r, a)),
                    Statement::Attr(_, _) => {}
                    Statement::IDeqID(_, _) => {}
                }
                g
            })
        },
    )(s)
}

fn stmt_list(s: &str) -> IResult<&str, Vec<Statement>> {
    map(
        opt(tuple((ws(stmt), opt(ws(char(';'))), stmt_list))),
        |r| match r {
            None => vec![],
            Some((x, _, mut xs)) => {
                let mut ys = vec![x];
                ys.append(&mut xs);
                ys
            }
        },
    )(s)
}

fn stmt(s: &str) -> IResult<&str, Statement> {
    alt((
        map(tuple((ws(id), ws(char('=')), ws(id))), |(id1, _, id2)| {
            Statement::IDeqID(id1, id2)
        }),
        attr_stmt,
        edge_stmt,
        node_stmt,
    ))(s)
}

fn attr_stmt(s: &str) -> IResult<&str, Statement> {
    // attr_stmt : (graph | node | edge) attr_list
    map(
        tuple((
            alt((
                tag_no_case("graph"),
                tag_no_case("node"),
                tag_no_case("edge"),
            )),
            attr_list,
        )),
        |(target, attr_list)| Statement::Attr(target.to_string(), attr_list),
    )(s)
}

fn attr_list(s: &str) -> IResult<&str, Vec<(String, String)>> {
    // attr_list : '[' [ a_list ] ']' [ attr_list ]
    map(
        tuple((ws(char('[')), opt(a_list), ws(char(']')), opt(attr_list))),
        |(_, a1, _, a2)| {
            let mut a1 = a1.unwrap_or_default();
            let mut a2 = a2.unwrap_or_default();
            a1.append(&mut a2);
            a1
        },
    )(s)
}

fn a_list(s: &str) -> IResult<&str, Vec<(String, String)>> {
    // a_list : ID '=' ID [ (';' | ',') ] [ a_list ]
    map(
        tuple((
            ws(id),
            ws(char('=')),
            ws(id),
            opt(ws(one_of(";,"))),
            opt(a_list),
        )),
        |(n, _, v, _, xs)| {
            let mut ys = vec![(n, v)];
            match xs {
                None => ys,
                Some(mut xs) => {
                    ys.append(&mut xs);
                    ys
                }
            }
        },
    )(s)
}

fn node_stmt(s: &str) -> IResult<&str, Statement> {
    // node_stmt : node_id [ attr_list ]
    map(tuple((node_id, opt(attr_list))), |(n, a)| {
        Statement::Node(n, a.unwrap_or_default())
    })(s)
}

fn edge_stmt(s: &str) -> IResult<&str, Statement> {
    map(
        tuple((ws(node_id), ws(edge_rhs), opt(attr_list))),
        |(l, r, a)| Statement::Edge(l, r, a.unwrap_or_default()),
    )(s)
}

fn edge_rhs(s: &str) -> IResult<&str, String> {
    map(tuple((ws(edgeop), ws(node_id))), |(_, r)| r)(s)
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

// <https://docs.rs/nom/6.2.1/nom/recipes/index.html#wrapper-combinators-that-eat-whitespace-before-and-after-a-parser>
fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn n(name: &str) -> (String, AttrList) {
        (name.to_string(), vec![])
    }

    fn nwa(name: &str, attr_list: AttrList) -> (String, AttrList) {
        (name.to_string(), attr_list)
    }

    fn ewa(l: &str, r: &str, attr_list: AttrList) -> (String, String, AttrList) {
        (l.to_string(), r.to_string(), attr_list)
    }

    fn ns(name: &str) -> Statement {
        Statement::Node(name.to_string(), vec![])
    }

    fn nswa(name: &str, attr_list: AttrList) -> Statement {
        Statement::Node(name.to_string(), attr_list)
    }

    fn es(l: &str, r: &str) -> Statement {
        Statement::Edge(l.to_string(), r.to_string(), vec![])
    }

    fn eswa(l: &str, r: &str, attr_list: AttrList) -> Statement {
        Statement::Edge(l.to_string(), r.to_string(), attr_list)
    }

    fn al(a: &[(&str, &str)]) -> AttrList {
        a.iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
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
                    nodes: vec![n("node")],
                    edges: vec![]
                }
            ))
        );
        assert_eq!(
            graph("digraph {n1 n2 n3 -> n4}"),
            Ok((
                "",
                Graph {
                    nodes: vec![n("n1"), n("n2")],
                    edges: vec![("n3".to_string(), "n4".to_string(), vec![])]
                }
            ))
        );
        assert_eq!(graph("graph {}"), Ok(("", Graph::default())));
        assert_eq!(graph("digraph example {}"), Ok(("", Graph::default())));
        assert_eq!(
            graph(r#"digraph "example graph" {}"#),
            Ok(("", Graph::default()))
        );
        assert_eq!(graph(r#"digraph{node[N1=V1]}"#), Ok(("", Graph::default())));
        assert_eq!(
            graph(r#"digraph{N1[K1=V1] N1 -> N2[K2=V2] }"#),
            Ok((
                "",
                Graph {
                    nodes: vec![nwa("N1", al(&[("K1", "V1")]))],
                    edges: vec![ewa("N1", "N2", al(&[("K2", "V2")]))],
                }
            ))
        );
        assert_eq!(
            graph(r#"graph { layout="patchwork" }"#),
            Ok(("", Graph::default()))
        );
    }

    #[test]
    fn stmt_list_test() {
        // [ stmt [';'] stmt_list ]
        let f = |s| all_consuming(stmt_list)(s);
        assert_eq!(f(""), Ok(("", vec![])));
        assert_eq!(f("N1"), Ok(("", vec![ns("N1")])));
        assert_eq!(f("N1 -> N2"), Ok(("", vec![es("N1", "N2")])));
        assert_eq!(f("N1 -- N2"), Ok(("", vec![es("N1", "N2")])));
        assert_eq!(f("N1 N2"), Ok(("", vec![ns("N1"), ns("N2")],)));
        assert_eq!(f("N1;N2"), Ok(("", vec![ns("N1"), ns("N2")],)));
        assert_eq!(f("N1 ; N2"), Ok(("", vec![ns("N1"), ns("N2")],)));
        assert_eq!(f("N1;N2;"), Ok(("", vec![ns("N1"), ns("N2")],)));
        assert_eq!(f("N1 ; N2 ; "), Ok(("", vec![ns("N1"), ns("N2")],)));
        assert_eq!(
            f("N1 N2 -> N3 N4"),
            Ok(("", vec![ns("N1"), es("N2", "N3"), ns("N4"),]))
        );
        assert_eq!(f(";").is_err(), true);
    }

    #[test]
    fn stmt_test() {
        assert_eq!(stmt("N1"), Ok(("", ns("N1"))));
        assert_eq!(stmt("N1 -> N2"), Ok(("", es("N1", "N2"))));
        assert_eq!(stmt("N1 -- N2"), Ok(("", es("N1", "N2"))));
        assert_eq!(
            stmt("node [N1=V1]"),
            Ok((
                "",
                Statement::Attr(
                    "node".to_string(),
                    vec![("N1".to_string(), "V1".to_string())]
                )
            ))
        );
        assert_eq!(
            stmt("ID1 = ID2"),
            Ok(("", Statement::IDeqID("ID1".to_string(), "ID2".to_string())))
        );
    }

    #[test]
    fn attr_list_test() {
        let attr = |n: &str, v: &str| (n.to_string(), v.to_string());
        assert_eq!(attr_list("[]"), Ok(("", vec![])));
        assert_eq!(attr_list("[][]"), Ok(("", vec![])));
        assert_eq!(attr_list("[N1=V1]"), Ok(("", vec![attr("N1", "V1")])));
        assert_eq!(
            attr_list("[N1=V1 N2=V2]"),
            Ok(("", vec![attr("N1", "V1"), attr("N2", "V2")]))
        );
        assert_eq!(
            attr_list("[N1=V1 N2=V2][N3=V3]"),
            Ok((
                "",
                vec![attr("N1", "V1"), attr("N2", "V2"), attr("N3", "V3")]
            ))
        );
    }

    #[test]
    fn a_list_test() {
        let attr = |n: &str, v: &str| (n.to_string(), v.to_string());
        assert_eq!(a_list("N1=V1"), Ok(("", vec![attr("N1", "V1")])));
        assert_eq!(a_list(" N1 = V1 "), Ok(("", vec![attr("N1", "V1")])));
        assert_eq!(
            a_list(" \"= \" = \"d e f\" "),
            Ok(("", vec![attr("= ", "d e f")]))
        );
        assert_eq!(
            a_list("N1=V1 N2=V2"),
            Ok(("", vec![attr("N1", "V1"), attr("N2", "V2")]))
        );

        assert_eq!(
            a_list("N1=V1;N2=V2"),
            Ok(("", vec![attr("N1", "V1"), attr("N2", "V2")]))
        );
        assert_eq!(
            a_list("N1=V1 ; N2=V2"),
            Ok(("", vec![attr("N1", "V1"), attr("N2", "V2")]))
        );

        assert_eq!(
            a_list("N1=V1,N2=V2"),
            Ok(("", vec![attr("N1", "V1"), attr("N2", "V2")]))
        );
        assert_eq!(
            a_list("N1=V1 , N2=V2"),
            Ok(("", vec![attr("N1", "V1"), attr("N2", "V2")]))
        );
    }

    #[test]
    fn node_stmt_test() {
        assert_eq!(node_stmt("N1"), Ok(("", ns("N1"))));
        assert_eq!(
            node_stmt("N1[K1=V1]"),
            Ok(("", nswa("N1", al(&[("K1", "V1")]))))
        );
    }

    #[test]
    fn edge_stmt_test() {
        assert_eq!(edge_stmt("N1 -> N2"), Ok(("", es("N1", "N2"))));
        assert_eq!(edge_stmt("N1 -- N2"), Ok(("", es("N1", "N2"))));
        assert_eq!(
            edge_stmt("N1 -- N2 [K1=V1]"),
            Ok(("", eswa("N1", "N2", al(&[("K1", "V1")]))))
        );
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
