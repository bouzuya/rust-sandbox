use crate::set::{ParseSetError, Set};
use serde::{Deserialize, Serialize};
use std::{fs, io, path::Path};
use thiserror::Error;

#[derive(Debug, Deserialize, Serialize)]
struct Line {
    key: String,
    value: f64,
}

#[derive(Debug, Error)]
pub enum ReadError {
    #[error("read error")]
    Read(#[source] io::Error),
    #[error("json parse error")]
    JsonParse(#[source] serde_json::Error),
    #[error("set convert error")]
    SetConvert(#[source] ParseSetError),
}

pub fn read_jsonl(path: &Path) -> Result<Vec<Set>, ReadError> {
    if !path.exists() {
        return Ok(vec![]);
    }
    let content = fs::read_to_string(path).map_err(|e| ReadError::Read(e))?;
    let mut jsonl = vec![];
    for line in content.split('\n') {
        if line.is_empty() {
            continue;
        }
        let set: Line = serde_json::from_str(line).map_err(|e| ReadError::JsonParse(e))?;
        let set = Set::new(set.key, set.value).map_err(|e| ReadError::SetConvert(e))?;
        jsonl.push(set);
    }
    Ok(jsonl)
}

#[derive(Debug, Error)]
pub enum WriteError {
    #[error("write error")]
    Write(#[source] io::Error),
    #[error("json convert error")]
    JsonConvert(#[source] serde_json::Error),
}

pub fn write_jsonl(path: &Path, events: &Vec<Set>) -> Result<(), WriteError> {
    let mut output = String::new();
    for set in events {
        let set = Line {
            key: set.key(),
            value: set.value(),
        };
        let line = serde_json::to_string(&set).map_err(|e| WriteError::JsonConvert(e))?;

        output.push_str(line.as_str());
        output.push('\n');
    }
    fs::write(path, output).map_err(|e| WriteError::Write(e))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn read_test() {
        let dir = tempdir().unwrap();
        let jsonl = dir.path().join("weight.jsonl");

        // not exists
        assert_eq!(read_jsonl(jsonl.as_path()).unwrap(), vec![]);

        // not file
        fs::create_dir(jsonl.as_path()).unwrap();
        assert_eq!(
            matches!(read_jsonl(jsonl.as_path()).err(), Some(ReadError::Read(_))),
            true
        );
        fs::remove_dir(jsonl.as_path()).unwrap();

        // broken json
        fs::write(jsonl.as_path(), concat!(r#"{]"#, "\n",)).unwrap();
        assert_eq!(
            matches!(
                read_jsonl(jsonl.as_path()).err(),
                Some(ReadError::JsonParse(_))
            ),
            true
        );

        // convert error (can't test)

        // OK
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
        assert_eq!(
            read_jsonl(jsonl.as_path()).unwrap(),
            vec![
                Set::new("2021-02-03".to_string(), 50.1).unwrap(),
                Set::new("2021-03-04".to_string(), 51.2).unwrap(),
            ]
        );
    }

    #[test]
    fn write_test() {
        let dir = tempdir().unwrap();
        let jsonl = dir.path().join("weight.jsonl");
        let events = vec![
            Set::new("2021-02-03".to_string(), 50.1).unwrap(),
            Set::new("2021-03-04".to_string(), 51.2).unwrap(),
        ];

        // json convert error (can't test)

        // not file
        fs::create_dir(jsonl.as_path()).unwrap();
        assert_eq!(
            matches!(
                write_jsonl(jsonl.as_path(), &events).err(),
                Some(WriteError::Write(_))
            ),
            true
        );
        fs::remove_dir(jsonl.as_path()).unwrap();

        // OK
        assert_eq!(write_jsonl(jsonl.as_path(), &events).is_ok(), true);
        assert_eq!(
            fs::read_to_string(jsonl.as_path()).unwrap(),
            concat!(
                r#"{"key":"2021-02-03","value":50.1}"#,
                "\n",
                r#"{"key":"2021-03-04","value":51.2}"#,
                "\n",
            )
        );
    }
}
