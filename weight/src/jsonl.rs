use crate::set::Set;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Debug, Deserialize, Serialize)]
struct Line {
    key: String,
    value: f64,
}

pub fn read_jsonl(path: &Path) -> Vec<Set> {
    if !path.exists() {
        return vec![];
    }
    let content = fs::read_to_string(path).unwrap();
    let mut jsonl = vec![];
    for line in content.split('\n') {
        if line.is_empty() {
            continue;
        }
        let set: Line = serde_json::from_str(line).unwrap();
        let set = Set::new(set.key, set.value).unwrap();
        jsonl.push(set);
    }
    jsonl
}

pub fn write_jsonl(path: &Path, jsonl: Vec<Set>) {
    let mut output = String::new();
    for set in jsonl {
        let set = Line {
            key: set.key(),
            value: set.value(),
        };
        let line = serde_json::to_string(&set).unwrap();
        output.push_str(line.as_str());
        output.push('\n');
    }
    fs::write(path, output).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test() {
        let dir = tempdir().unwrap();
        let jsonl = dir.path().join("weight.jsonl");
        fs::write(
            jsonl.as_path(),
            concat!(
                r#"{"key":"2021-02-03","value":50.1}"#,
                "\n",
                r#"{"key":"2021-03-04","value":51.2}"#,
                "\n",
            ),
        )
        .unwrap();

        let set = read_jsonl(jsonl.as_path());
        assert_eq!(
            set,
            vec![
                Set::new("2021-02-03".to_string(), 50.1).unwrap(),
                Set::new("2021-03-04".to_string(), 51.2).unwrap(),
            ]
        );
    }
}
