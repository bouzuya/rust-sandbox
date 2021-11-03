use std::{
    fs, io,
    path::{Path, PathBuf},
};
use thiserror::Error;

#[derive(Debug, Eq, PartialEq)]
pub enum Entry {
    Dir { name: String },
    File { content: String, name: String },
}

#[derive(Debug, Error)]
pub enum ListEntriesError {
    #[error("file_name is not string")]
    FileName,
    #[error("read_dir or next")]
    ReadDir(#[from] io::Error),
    #[error("canonicalize")]
    Canonicalize(#[source] io::Error),
}

fn list_entries_impl(first: bool, path: &Path) -> Result<Vec<Entry>, ListEntriesError> {
    let name = path.to_str().ok_or(ListEntriesError::FileName)?.to_string();
    let mut entries = vec![];
    if path.is_dir() {
        if !first {
            entries.push(Entry::Dir { name });
        }
        for dir_entry in path.read_dir()? {
            let path_buf = path.join(dir_entry?.path());
            let dir_file_names = list_entries_impl(false, path_buf.as_path())?;
            entries.extend(dir_file_names);
        }
    } else {
        let content = fs::read_to_string(path)?;
        entries.push(Entry::File { content, name })
    }
    Ok(entries)
}

pub fn list_entries(path: &Path) -> Result<Vec<Entry>, ListEntriesError> {
    let path = path
        .canonicalize()
        .map_err(ListEntriesError::Canonicalize)?;
    let strip_prefix = |s: String| -> Result<String, ListEntriesError> {
        Ok(PathBuf::from(s)
            .strip_prefix(path.as_path())
            .expect("internal error")
            .to_str()
            .ok_or(ListEntriesError::FileName)?
            .to_string())
    };
    list_entries_impl(true, path.as_path())?
        .into_iter()
        .map(|e| {
            Ok(match e {
                Entry::Dir { name } => Entry::Dir {
                    name: strip_prefix(name)?,
                },
                Entry::File { content, name } => Entry::File {
                    content,
                    name: strip_prefix(name)?,
                },
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn create_test_data() -> PathBuf {
        // tmp_dir/
        //   tmpl/
        //     {{foo}}/
        //       {{bar}}
        let dir = tempdir().unwrap();
        let tmpl = dir.path().join("tmpl");
        fs::create_dir_all(tmpl.as_path()).unwrap();
        let tmpl_foo = tmpl.join("{{foo}}");
        fs::create_dir_all(tmpl_foo.as_path()).unwrap();
        let tmpl_foo_bar = tmpl_foo.join("{{bar}}");
        fs::write(tmpl_foo_bar.as_path(), "{{baz}}").unwrap();
        dir.into_path()
    }

    #[test]
    fn list_entries_test() {
        let dir = create_test_data();
        let tmpl = dir.join("tmpl");
        assert_eq!(
            list_entries(tmpl.as_path()).unwrap(),
            vec![
                Entry::Dir {
                    name: "{{foo}}".to_string()
                },
                Entry::File {
                    name: "{{foo}}/{{bar}}".to_string(),
                    content: "{{baz}}".to_string(),
                }
            ]
        );
    }
}
