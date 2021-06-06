use super::EventRepository;
use crate::{
    event::Event,
    remove::Remove,
    set::{ParseSetError, Set},
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::{
    fs, io,
    path::{Path, PathBuf},
};
use thiserror::Error;

pub struct JsonlEventRepository {
    data_file: PathBuf,
}

impl JsonlEventRepository {
    pub fn new(data_file: PathBuf) -> Self {
        Self { data_file }
    }
}

#[async_trait]
impl EventRepository for JsonlEventRepository {
    async fn find_all(&self) -> anyhow::Result<Vec<Event>> {
        Ok(read_jsonl(self.data_file.as_path())?)
    }

    async fn save(&self, events: &Vec<Event>) -> anyhow::Result<()> {
        Ok(write_jsonl(self.data_file.as_path(), events)?)
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
enum Line {
    #[serde(rename = "remove")]
    Remove { key: String },
    #[serde(rename = "set")]
    Set { key: String, value: f64 },
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

fn read_jsonl(path: &Path) -> Result<Vec<Event>, ReadError> {
    if !path.exists() {
        return Ok(vec![]);
    }
    let content = fs::read_to_string(path).map_err(|e| ReadError::Read(e))?;
    let mut jsonl = vec![];
    for line in content.split('\n') {
        if line.is_empty() {
            continue;
        }
        let line: Line = serde_json::from_str(line).map_err(|e| ReadError::JsonParse(e))?;
        let event = match line {
            Line::Remove { key } => Event::Remove(Remove::new(key)),
            Line::Set { key, value } => Set::new(key, value)
                .map(|set| Event::Set(set))
                .map_err(|e| ReadError::SetConvert(e))?,
        };
        jsonl.push(event);
    }
    Ok(jsonl)
}

#[derive(Debug, Error)]
enum WriteError {
    #[error("write error")]
    Write(#[source] io::Error),
    #[error("json convert error")]
    JsonConvert(#[source] serde_json::Error),
}

fn write_jsonl(path: &Path, events: &Vec<Event>) -> Result<(), WriteError> {
    let mut output = String::new();
    for event in events {
        let line = match event {
            Event::Remove(remove) => {
                let set = Line::Remove { key: remove.key() };
                serde_json::to_string(&set).map_err(|e| WriteError::JsonConvert(e))?
            }
            Event::Set(set) => {
                let set = Line::Set {
                    key: set.key(),
                    value: set.value(),
                };
                serde_json::to_string(&set).map_err(|e| WriteError::JsonConvert(e))?
            }
        };

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
                r#"{"type":"set","key":"2021-02-03","value":50.1}"#,
                "\n",
                r#"{"type":"set","key":"2021-03-04","value":51.2}"#,
                "\n",
                r#"{"type":"remove","key":"2021-03-04"}"#,
                "\n",
            ),
        )
        .unwrap();
        assert_eq!(
            read_jsonl(jsonl.as_path()).unwrap(),
            vec![
                Event::Set(Set::new("2021-02-03".to_string(), 50.1).unwrap()),
                Event::Set(Set::new("2021-03-04".to_string(), 51.2).unwrap()),
                Event::Remove(Remove::new("2021-03-04".to_string())),
            ]
        );
    }

    #[test]
    fn write_test() {
        let dir = tempdir().unwrap();
        let jsonl = dir.path().join("weight.jsonl");
        let events = vec![
            Event::Set(Set::new("2021-02-03".to_string(), 50.1).unwrap()),
            Event::Set(Set::new("2021-03-04".to_string(), 51.2).unwrap()),
            Event::Remove(Remove::new("2021-03-04".to_string())),
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
                r#"{"type":"set","key":"2021-02-03","value":50.1}"#,
                "\n",
                r#"{"type":"set","key":"2021-03-04","value":51.2}"#,
                "\n",
                r#"{"type":"remove","key":"2021-03-04"}"#,
                "\n",
            )
        );
    }
}
