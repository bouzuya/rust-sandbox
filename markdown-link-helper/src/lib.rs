mod rule;

use crate::rule::Rule;
use pulldown_cmark::{BrokenLink, Options, Parser};
use std::{collections::BTreeSet, convert::TryFrom};

fn broken_links(content: &str) -> Vec<String> {
    let mut res = vec![];
    let mut callback = |broken_link: BrokenLink| {
        res.push(broken_link.reference.to_owned());
        None
    };
    let mut parser =
        Parser::new_with_broken_link_callback(&content, Options::empty(), Some(&mut callback));
    while let Some(_) = parser.next() {}
    res
}

pub fn run(rules: &Vec<Rule>, content: &str) {
    let links = broken_links(&content);
    let links = links.into_iter().collect::<BTreeSet<String>>();
    for link in links {
        let mut m = None;
        for rule in rules.iter() {
            if let Some(replaced) = rule.apply(&link) {
                m = Some(replaced);
                break;
            }
        }
        match m {
            None => eprintln!("'{}' is a broken link", link),
            Some(replaced) => println!("{}", replaced),
        }
    }
}

pub fn build_rules() -> Vec<Rule> {
    vec![
        (
            r"^(\d{4})-(\d{2})-(\d{2})$",
            "[$1-$2-$3]: https://blog.bouzuya.net/$1/$2/$3/",
        ),
        (
            r"^github:([0-9A-Za-z]+(?:-?[0-9A-Za-z])*)/((?:..[-.0-9A-Z_a-z]+)|(?:.[-0-9A-Z_a-z][-.0-9A-Z_a-z]*)|(?:[-0-9A-Z_a-z][-.0-9A-Z_a-z]*))$",
            "[$0]: https://github.com/$1/$2",
        ),
        (
            r"^([0-9A-Za-z]+(?:-?[0-9A-Za-z])*)/((?:..[-.0-9A-Z_a-z]+)|(?:.[-0-9A-Z_a-z][-.0-9A-Z_a-z]*)|(?:[-0-9A-Z_a-z][-.0-9A-Z_a-z]*))$",
            "[$0]: https://github.com/$1/$2",
        ),
    ]
    .iter()
    .map(|&rule| Rule::try_from(rule).expect("re is not valid"))
    .collect::<Vec<Rule>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_broken_links() {
        assert_eq!(
            broken_links(
                &vec![
                    "[a][]",
                    "",
                    "[][b]",
                    "",
                    "[c]",
                    "",
                    "[d] [e]",
                    "",
                    "[ok1](http://example.com)",
                    "",
                    "[ok2][]",
                    "",
                    "[][ok3]",
                    "",
                    "[ok2]: http://example.com",
                    "[ok3]: http://example.com"
                ]
                .join("\n")
            ),
            vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string(),
                "e".to_string(),
            ]
        );
        assert_eq!(
            broken_links(&vec!["[a]", "[a]",].join("\n")),
            vec!["a".to_string(), "a".to_string(),]
        );
    }

    #[test]
    fn test_rules() {
        let rules = build_rules();
        assert_eq!(
            rules
                .iter()
                .map(|rule| rule.apply("2021-04-29"))
                .find(|r| r.is_some()),
            Some(Some(
                "[2021-04-29]: https://blog.bouzuya.net/2021/04/29/".to_owned()
            ))
        );
        assert_eq!(
            rules
                .iter()
                .map(|rule| rule.apply("bouzuya/blog.bouzuya.net"))
                .find(|r| r.is_some()),
            Some(Some(
                "[bouzuya/blog.bouzuya.net]: https://github.com/bouzuya/blog.bouzuya.net"
                    .to_owned()
            ))
        );
        assert_eq!(
            rules
                .iter()
                .map(|rule| rule.apply("github:bouzuya/blog.bouzuya.net"))
                .find(|r| r.is_some()),
            Some(Some(
                "[github:bouzuya/blog.bouzuya.net]: https://github.com/bouzuya/blog.bouzuya.net"
                    .to_owned()
            ))
        );
    }

    #[test]
    fn test_bbn_rule() {
        let rules = build_rules();
        let f = |s| {
            rules
                .iter()
                .map(|rule| rule.apply(s))
                .find(|r| r.is_some())
                .is_some()
        };
        assert_eq!(f("2021-04-29"), true);
    }

    #[test]
    fn test_github_rule() {
        let rules = build_rules();
        let f = |s| {
            rules
                .iter()
                .map(|rule| rule.apply(s))
                .find(|r| r.is_some())
                .is_some()
        };
        assert_eq!(f("-/repo"), false);
        assert_eq!(f("0/repo"), true);
        assert_eq!(f("A/repo"), true);
        assert_eq!(f("a/repo"), true);
        assert_eq!(f("-0/repo"), false);
        assert_eq!(f("0-/repo"), false);
        assert_eq!(f("0-0/repo"), true);
        assert_eq!(f("0-A/repo"), true);
        assert_eq!(f("0-a/repo"), true);
        assert_eq!(f("0--0/repo"), false);
        assert_eq!(f("owner/-"), true);
        assert_eq!(f("owner/."), false);
        assert_eq!(f("owner/0"), true);
        assert_eq!(f("owner/A"), true);
        assert_eq!(f("owner/_"), true);
        assert_eq!(f("owner/a"), true);
        assert_eq!(f("owner/.-"), true);
        assert_eq!(f("owner/.."), false);
        assert_eq!(f("owner/..-"), true);
        assert_eq!(f("owner/..."), true);
    }
}
