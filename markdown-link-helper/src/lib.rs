mod rule;

use std::path::Path;

use crate::rule::Rule;
use anyhow::anyhow;
use pulldown_cmark::{BrokenLink, Options, Parser};
use std::{collections::BTreeSet, convert::TryFrom, fs};

fn broken_links(content: &str) -> Vec<String> {
    let mut res = vec![];
    let mut callback = |broken_link: BrokenLink| {
        res.push(broken_link.reference.to_string());
        None
    };
    let parser =
        Parser::new_with_broken_link_callback(content, Options::empty(), Some(&mut callback));
    for _ in parser {}
    res
}

pub fn run(rules: &[Rule], content: &str) {
    let links = broken_links(content);
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

pub fn build_rules<P>(path: P) -> anyhow::Result<Vec<Rule>>
where
    P: AsRef<Path>,
{
    let content = fs::read_to_string(path)?;
    let json: Vec<(String, String)> = serde_json::from_str(content.as_str())?;
    json.into_iter()
        .map(|i| Rule::try_from((i.0.as_str(), i.1.as_str())))
        .collect::<Result<Vec<Rule>, rule::Error>>()
        .map_err(|e| anyhow!(e))
}
