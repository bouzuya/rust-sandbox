use anyhow::anyhow;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, tag_no_case, take_until},
    character::complete::{alpha1, alphanumeric1, anychar, char, multispace0},
    combinator::{all_consuming, map, opt, recognize},
    error::ParseError,
    multi::{fold_many0, many0},
    sequence::{delimited, pair, tuple},
    IResult,
};

use crate::graph::{Either, Graph, Statement};

pub fn parse(s: &str) -> anyhow::Result<Graph> {
    all_consuming(graph)(s)
        .map(|(_, v)| v)
        .map_err(|_| anyhow!("parse error"))
}

fn graph(s: &str) -> IResult<&str, Graph> {
    // graph : [ strict ] (graph | digraph) [ ID ] '{' stmt_list '}'
    map(
        tuple((
            opt(ws(tag_no_case("strict"))),
            alt((ws(tag_no_case("graph")), ws(tag_no_case("digraph")))),
            opt(ws(id)),
            ws(char('{')),
            ws(stmt_list),
            ws(char('}')),
        )),
        |(_strict, _graph, name, _, statements, _)| Graph::new(name, statements),
    )(s)
}

fn stmt_list(s: &str) -> IResult<&str, Vec<Statement>> {
    // stmt_list : [ stmt [ ';' ] stmt_list ]
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
    // stmt : node_stmt | edge_stmt | attr_stmt | ID '=' ID | subgraph
    alt((
        edge_stmt,
        map(subgraph, Statement::Subgraph),
        map(tuple((ws(id), ws(char('=')), ws(id))), |(id1, _, id2)| {
            Statement::IDeqID(id1, id2)
        }),
        attr_stmt,
        node_stmt,
    ))(s)
}

fn attr_stmt(s: &str) -> IResult<&str, Statement> {
    // attr_stmt : (graph | node | edge) attr_list
    map(
        tuple((
            alt((
                ws(tag_no_case("graph")),
                ws(tag_no_case("node")),
                ws(tag_no_case("edge")),
            )),
            ws(attr_list),
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
            opt(alt((ws(char(';')), ws(char(','))))),
            opt(a_list),
        )),
        |(name, _, value, _, xs)| {
            let mut attrs = vec![(name, value)];
            match xs {
                None => attrs,
                Some(mut xs) => {
                    attrs.append(&mut xs);
                    attrs
                }
            }
        },
    )(s)
}

fn edge_stmt(s: &str) -> IResult<&str, Statement> {
    // edge_stmt : (node_id | subgraph) edge_rhs [ attr_list ]
    map(
        tuple((
            alt((
                map(ws(subgraph), Either::Right),
                map(ws(node_id), Either::Left),
            )),
            ws(edge_rhs),
            opt(attr_list),
        )),
        |(l, r, a)| Statement::Edge(l, r, a.unwrap_or_default()),
    )(s)
}

fn edge_rhs(s: &str) -> IResult<&str, Either<String, Graph>> {
    // edge_rhs : edgeop (node_id | subgraph) [ edge_rhs ]
    map(
        tuple((
            alt((ws(tag("->")), ws(tag("--")))),
            alt((
                map(ws(subgraph), Either::Right),
                map(ws(node_id), Either::Left),
            )),
        )),
        |(_, r)| r,
    )(s)
}

fn node_stmt(s: &str) -> IResult<&str, Statement> {
    // node_stmt : node_id [ attr_list ]
    map(tuple((ws(node_id), opt(attr_list))), |(n, a)| {
        Statement::Node(n, a.unwrap_or_default())
    })(s)
}

fn node_id(s: &str) -> IResult<&str, String> {
    // node_id : ID [ port ]
    // NOTE: port is not supported
    id(s)
}

fn subgraph(s: &str) -> IResult<&str, Graph> {
    // subgraph : [ subgraph [ ID ] ] '{' stmt_list '}'
    map(
        tuple((
            opt(tuple((tag_no_case("subgraph"), opt(ws(id))))),
            ws(char('{')),
            ws(stmt_list),
            ws(char('}')),
        )),
        |(subgraph, _, stmt_list, _)| {
            let name = match subgraph {
                None | Some((_, None)) => None,
                Some((_, Some(id))) => Some(id),
            };
            Graph::new(name, stmt_list)
        },
    )(s)
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

fn comment(s: &str) -> IResult<&str, ()> {
    alt((map(block_comment, |_| ()), line_comment))(s)
}

fn block_comment(s: &str) -> IResult<&str, &str> {
    delimited(tag("/*"), take_until("*/"), tag("*/"))(s)
}

fn line_comment(s: &str) -> IResult<&str, ()> {
    map(
        tuple((
            tag("//"),
            many0(is_not("\r\n")),
            opt(char('\r')),
            opt(char('\n')),
        )),
        |_| (),
    )(s)
}

fn ws<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    F: Fn(&'a str) -> IResult<&'a str, O>,
{
    map(
        tuple((
            multispace0,
            many0(ws1(comment)), // tuple((multispace0, opt(comment), multispace0))),
            multispace0,
            inner,
            multispace0,
            many0(ws1(comment)), // tuple((multispace0, opt(comment), multispace0))),
            multispace0,
        )),
        |(_, _, _, x, _, _, _)| x, // many0(tuple((multispace0, opt(comment), multispace0))),
    )
}
// <https://docs.rs/nom/6.2.1/nom/recipes/index.html#wrapper-combinators-that-eat-whitespace-before-and-after-a-parser>
fn ws1<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

#[cfg(test)]
mod tests {
    use crate::graph::AttrList;

    use super::*;

    fn ns(name: &str) -> Statement {
        Statement::Node(name.to_string(), vec![])
    }

    fn nswa(name: &str, attr_list: AttrList) -> Statement {
        Statement::Node(name.to_string(), attr_list)
    }

    fn es(l: &str, r: &str) -> Statement {
        Statement::Edge(
            Either::Left(l.to_string()),
            Either::Left(r.to_string()),
            vec![],
        )
    }

    fn eswa(l: &str, r: &str, attr_list: AttrList) -> Statement {
        Statement::Edge(
            Either::Left(l.to_string()),
            Either::Left(r.to_string()),
            attr_list,
        )
    }

    fn al(a: &[(&str, &str)]) -> AttrList {
        a.iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    #[test]
    fn graph_test() {
        assert_eq!(
            graph("// comment\ngraph {}"),
            Ok(("", Graph::new(None, vec![])))
        );
        assert_eq!(
            graph("/* comment */graph {}"),
            Ok(("", Graph::new(None, vec![])))
        );
        assert_eq!(graph("strict graph {}"), Ok(("", Graph::new(None, vec![]))));
        assert_eq!(graph("graph {}"), Ok(("", Graph::new(None, vec![]))));
        assert_eq!(graph("graph{}"), Ok(("", Graph::new(None, vec![]))));
        assert_eq!(
            graph("graph {n}"),
            Ok((
                "",
                Graph::new(None, vec![Statement::Node("n".to_string(), vec![])])
            ))
        );
        assert_eq!(
            graph("digraph {n1 n2 n3 -> n4}"),
            Ok((
                "",
                Graph::new(
                    None,
                    vec![
                        Statement::Node("n1".to_string(), vec![]),
                        Statement::Node("n2".to_string(), vec![]),
                        Statement::Edge(
                            Either::Left("n3".to_string()),
                            Either::Left("n4".to_string()),
                            vec![]
                        ),
                    ]
                )
            ))
        );
        assert_eq!(
            graph("digraph example {}"),
            Ok(("", Graph::new(Some("example".to_string()), vec![])))
        );
        assert_eq!(
            graph(r#"digraph "example graph" {}"#),
            Ok(("", Graph::new(Some("example graph".to_string()), vec![])))
        );
        assert_eq!(
            graph(r#"digraph{node[N1=V1]}"#),
            Ok((
                "",
                Graph::new(
                    None,
                    vec![Statement::Attr(
                        "node".to_string(),
                        vec![("N1".to_string(), "V1".to_string())]
                    )]
                )
            ))
        );
        assert_eq!(
            graph(r#"digraph{graph[N1=V1]}"#),
            Ok((
                "",
                Graph::new(
                    None,
                    vec![Statement::Attr(
                        "graph".to_string(),
                        vec![("N1".to_string(), "V1".to_string())]
                    )]
                )
            ))
        );
        assert_eq!(
            graph(r#"digraph{edge[N1=V1]}"#),
            Ok((
                "",
                Graph::new(
                    None,
                    vec![Statement::Attr(
                        "edge".to_string(),
                        vec![("N1".to_string(), "V1".to_string())]
                    )]
                )
            ))
        );
        assert_eq!(
            graph(r#"digraph{N1[K1=V1] N1 -> N2[K2=V2] }"#),
            Ok((
                "",
                Graph::new(
                    None,
                    vec![
                        Statement::Node(
                            "N1".to_string(),
                            vec![("K1".to_string(), "V1".to_string())]
                        ),
                        Statement::Edge(
                            Either::Left("N1".to_string()),
                            Either::Left("N2".to_string()),
                            vec![("K2".to_string(), "V2".to_string())]
                        )
                    ]
                )
            ))
        );
        assert_eq!(
            graph(r#"graph { layout="patchwork" }"#),
            Ok((
                "",
                Graph::new(
                    None,
                    vec![Statement::IDeqID(
                        "layout".to_string(),
                        "patchwork".to_string()
                    )]
                )
            ))
        );
        assert_eq!(
            graph(r#"digraph { A -> {B C} }"#),
            Ok((
                "",
                Graph::new(
                    None,
                    vec![Statement::Edge(
                        Either::Left("A".to_string()),
                        Either::Right(Graph::new(
                            None,
                            vec![
                                Statement::Node("B".to_string(), vec![]),
                                Statement::Node("C".to_string(), vec![])
                            ]
                        )),
                        vec![]
                    ),]
                )
            ))
        );
        assert_eq!(
            graph(r#"digraph { A -> subgraph {B C} }"#),
            Ok((
                "",
                Graph::new(
                    None,
                    vec![Statement::Edge(
                        Either::Left("A".to_string()),
                        Either::Right(Graph::new(
                            None,
                            vec![
                                Statement::Node("B".to_string(), vec![]),
                                Statement::Node("C".to_string(), vec![]),
                            ]
                        )),
                        vec![]
                    )]
                )
            ))
        );
        assert_eq!(
            graph(r#"digraph { { N1 -> N2 } -> { N3 -> N4 } }"#),
            Ok((
                "",
                Graph::new(
                    None,
                    vec![Statement::Edge(
                        Either::Right(Graph::new(
                            None,
                            vec![Statement::Edge(
                                Either::Left("N1".to_string()),
                                Either::Left("N2".to_string()),
                                vec![]
                            )]
                        )),
                        Either::Right(Graph::new(
                            None,
                            vec![Statement::Edge(
                                Either::Left("N3".to_string()),
                                Either::Left("N4".to_string()),
                                vec![]
                            )]
                        )),
                        vec![]
                    )]
                )
            ))
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
        assert_eq!(
            stmt("subgraph subgraph1 { subgraph subgraph2 {} }"),
            Ok((
                "",
                Statement::Subgraph(Graph::new(
                    Some("subgraph1".to_string()),
                    vec![Statement::Subgraph(Graph::new(
                        Some("subgraph2".to_string()),
                        vec![]
                    ))]
                ))
            ))
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
    fn subgraph_test() {
        assert_eq!(
            subgraph("subgraph id1 { node_id1 }"),
            Ok((
                "",
                Graph::new(
                    Some("id1".to_string()),
                    vec![Statement::Node("node_id1".to_string(), vec![])]
                )
            ))
        );
        assert_eq!(
            subgraph("subgraph { node_id1 }"),
            Ok((
                "",
                Graph::new(None, vec![Statement::Node("node_id1".to_string(), vec![])])
            ))
        );
        assert_eq!(
            subgraph("{ node_id1 }"),
            Ok((
                "",
                Graph::new(None, vec![Statement::Node("node_id1".to_string(), vec![])])
            ))
        );
        assert_eq!(subgraph("{}"), Ok(("", Graph::new(None, vec![]))));
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

    #[test]
    fn comment_test() {
        let f = comment;
        assert_eq!(f("/*abc*/"), Ok(("", ())));
        assert_eq!(f("//def"), Ok(("", ())));
    }

    #[test]
    fn block_comment_test() {
        let f = block_comment;
        assert_eq!(f("/**/"), Ok(("", "")));
        assert_eq!(f("/*a*/"), Ok(("", "a")));
        assert_eq!(f("/*a\nb*/"), Ok(("", "a\nb")));
        assert_eq!(f("/*a\n*/b*/"), Ok(("b*/", "a\n")));
    }

    #[test]
    fn line_comment_test() {
        let f = line_comment;
        assert_eq!(f("//"), Ok(("", ())));
        assert_eq!(f("//\n"), Ok(("", ())));
        assert_eq!(f("//\naaa"), Ok(("aaa", ())));
        assert_eq!(f("//\r"), Ok(("", ())));
        assert_eq!(f("//\raaa"), Ok(("aaa", ())));
        assert_eq!(f("//\r\n"), Ok(("", ())));
        assert_eq!(f("//\r\naaa"), Ok(("aaa", ())));
        assert_eq!(f("// foo"), Ok(("", ())));
        assert_eq!(f("// foo\nbbb"), Ok(("bbb", ())));
    }

    #[test]
    fn test() {
        let f = line_comment;
        assert_eq!(f("//"), Ok(("", ())));
        assert_eq!(f("//\n"), Ok(("", ())));
        assert_eq!(f("//\naaa"), Ok(("aaa", ())));
        assert_eq!(f("//\r"), Ok(("", ())));
        assert_eq!(f("//\raaa"), Ok(("aaa", ())));
        assert_eq!(f("//\r\n"), Ok(("", ())));
        assert_eq!(f("//\r\naaa"), Ok(("aaa", ())));
        assert_eq!(f("// foo"), Ok(("", ())));
        assert_eq!(f("// foo\nbbb"), Ok(("bbb", ())));
    }
}
