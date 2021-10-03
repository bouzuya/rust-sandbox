use std::{
    fs::{self, DirEntry},
    io,
    path::{Path, PathBuf},
};

pub struct ListFiles(Vec<PathBuf>);

impl ListFiles {
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        Ok(ListFiles(if path.as_ref().is_dir() {
            read_dir_sorted(path).map(|dir_entries| {
                dir_entries
                    .into_iter()
                    .rev()
                    .map(|dir_entry| dir_entry.path())
                    .collect()
            })?
        } else {
            vec![path.as_ref().to_path_buf()]
        }))
    }
}

impl Iterator for ListFiles {
    type Item = io::Result<PathBuf>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(entry_path) = self.0.pop() {
            if entry_path.is_dir() {
                self.0.extend(match read_dir_sorted(entry_path) {
                    Ok(dir_entries) => dir_entries
                        .into_iter()
                        .rev()
                        .map(|dir_entry| dir_entry.path()),
                    Err(err) => return Some(Err(err)),
                });
                continue;
            }

            if entry_path.is_file() {
                return Some(Ok(entry_path));
            }
        }
        None
    }
}

fn read_dir_sorted<P: AsRef<Path>>(path: P) -> io::Result<Vec<DirEntry>> {
    let mut dir_entries = fs::read_dir(path)?
        .into_iter()
        .collect::<io::Result<Vec<DirEntry>>>()?;
    dir_entries.sort_by_key(|dir_entry| dir_entry.file_name());
    Ok(dir_entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;

    use tempfile::TempDir;

    fn setup() -> anyhow::Result<TempDir> {
        let tempdir = tempfile::tempdir()?;
        let root_dir = tempdir.path();
        let dir1 = root_dir.join("dir1");
        fs::create_dir_all(dir1.as_path())?;
        let dir2 = root_dir.join("dir2");
        fs::create_dir_all(dir2.as_path())?;
        let file1 = dir1.join("file1");
        fs::write(file1, "file1 contents")?;
        let file2 = dir1.join("file2");
        fs::write(file2, "file2 contents")?;
        let file3 = dir2.join("file1");
        fs::write(file3, "file1 contents")?;
        let file4 = dir2.join("file2");
        fs::write(file4, "file2 contents")?;
        Ok(tempdir)
    }

    #[test]
    fn read_dir_sorted_test() -> anyhow::Result<()> {
        let tempdir = setup()?;
        let dir_entries = read_dir_sorted(tempdir.path())?;
        assert_eq!(
            dir_entries
                .into_iter()
                .map(|e| e.path())
                .collect::<Vec<PathBuf>>(),
            vec![tempdir.path().join("dir1"), tempdir.path().join("dir2")]
        );
        Ok(())
    }

    #[test]
    fn list_files_test() -> anyhow::Result<()> {
        let tempdir = setup()?;
        let path_bufs = ListFiles::new(tempdir.path())?.collect::<io::Result<Vec<PathBuf>>>()?;
        assert_eq!(
            path_bufs,
            vec![
                tempdir.path().join("dir1").join("file1"),
                tempdir.path().join("dir1").join("file2"),
                tempdir.path().join("dir2").join("file1"),
                tempdir.path().join("dir2").join("file2"),
            ]
        );
        Ok(())
    }
}
