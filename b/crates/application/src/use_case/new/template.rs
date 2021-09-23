use super::parse::{parse, Token};
use anyhow::Context;
use std::collections::{BTreeMap, HashSet};
use thiserror::Error;

#[derive(Debug, Eq, PartialEq)]
pub struct Template<'a> {
    pub tokens: Vec<Token<'a>>,
    vars: HashSet<&'a str>,
}

#[derive(Debug, Error)]
pub enum ParseTemplateError {
    #[error("parse")]
    Parse,
}

impl<'a> Template<'a> {
    pub fn render(&self, data: &BTreeMap<String, String>) -> anyhow::Result<String> {
        let mut t = String::new();
        for token in self.tokens.iter() {
            match token {
                Token::Str(s) => t.push_str(s),
                Token::Var(v) => {
                    let v = data.get(&v.to_string()).context("no var error")?;
                    t.push_str(v.as_str());
                }
            }
        }
        Ok(t)
    }
}

impl<'a> std::convert::TryFrom<&'a str> for Template<'a> {
    type Error = ParseTemplateError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let tokens: Vec<Token<'a>> = parse(value).map_err(|_| ParseTemplateError::Parse)?;
        let mut vars: HashSet<&'a str> = HashSet::new();
        for token in tokens.iter() {
            match token {
                Token::Str(_) => {}
                Token::Var(s) => {
                    vars.insert(s);
                }
            }
        }
        Ok(Self { tokens, vars })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn render_test() {
        let tmpl = Template::try_from("foo{{bar}}baz").unwrap();
        let mut map = BTreeMap::new();
        map.insert("bar".to_string(), ",".to_string());
        assert_eq!(tmpl.render(&map).unwrap(), "foo,baz".to_string());
    }

    #[test]
    fn try_from_test() {
        let mut vars = HashSet::new();
        vars.insert("bar");
        assert_eq!(
            Template::try_from("foo{{bar}}baz").unwrap(),
            Template {
                tokens: vec![Token::Str("foo"), Token::Var("bar"), Token::Str("baz")],
                vars
            }
        );
    }
}
