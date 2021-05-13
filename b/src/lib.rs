use std::collections::BTreeMap;

pub fn render(template: &str, data: &BTreeMap<String, String>) -> String {
    let mut t = String::new();
    let mut expr: Option<String> = None;
    for c in template.chars() {
        match expr {
            Some(mut s) => match c {
                '{' => panic!(),
                '}' => {
                    let v = data.get(&s).unwrap();
                    t.push_str(v.as_str());
                    expr = None;
                }
                c => {
                    if !c.is_ascii_lowercase() {
                        panic!();
                    }
                    s.push(c);
                    expr = Some(s);
                }
            },
            None => match c {
                '{' => expr = Some(String::new()),
                c => t.push(c),
            },
        }
    }
    t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut map = BTreeMap::new();
        map.insert("x".to_string(), "y".to_string());
        assert_eq!(render("{x}", &map), "y".to_string());
    }
}
