mod parse;

use parse::{parse, Token};

use std::collections::BTreeMap;

pub fn render(template: &str, data: &BTreeMap<String, String>) -> String {
    let mut t = String::new();
    let tokens = parse(template).unwrap();
    for token in tokens {
        match token {
            Token::Str(s) => t.push_str(s),
            Token::Var(v) => {
                let v = data.get(v).unwrap();
                t.push_str(v.as_str());
            }
        }
    }
    t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_test() {
        let mut map = BTreeMap::new();
        map.insert("x".to_string(), "y".to_string());
        assert_eq!(render("{{x}}", &map), "y".to_string());
    }
}
