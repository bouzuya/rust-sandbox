use std::{
    collections::VecDeque,
    fs::{self, DirEntry},
    io,
    path::{Path, PathBuf},
    str::FromStr,
};

use chrono::{DateTime, FixedOffset, TimeZone, Utc};
use entity::BId;
use query::{Digit2, Digit4, OptionalDate, Query};
use use_case::TimeZoneOffset;

pub struct ListFiles {
    root_dir: PathBuf,
    query: Query,
    query_time_zone_offset: TimeZoneOffset,
    queue: VecDeque<PathBuf>,
}

impl ListFiles {
    pub fn new<P: AsRef<Path>>(
        path: P,
        query: Query,
        query_time_zone_offset: TimeZoneOffset,
    ) -> io::Result<Self> {
        Ok(ListFiles {
            root_dir: path.as_ref().to_path_buf(),
            query,
            query_time_zone_offset,
            queue: if path.as_ref().is_dir() {
                read_dir_sorted(path).map(|dir_entries| {
                    dir_entries
                        .into_iter()
                        .map(|dir_entry| dir_entry.path())
                        .collect::<VecDeque<_>>()
                })?
            } else {
                let mut queue = VecDeque::new();
                queue.push_back(path.as_ref().to_path_buf());
                queue
            },
        })
    }

    fn match_query<P: AsRef<Path>>(&self, path: P) -> bool {
        let (query_since, query_until) = self.query.naive_date_time_range();
        let query_since = DateTime::<Utc>::from(
            FixedOffset::from(self.query_time_zone_offset)
                .from_local_datetime(&query_since)
                .unwrap(),
        ); // TODO
        let query_until = DateTime::<Utc>::from(
            FixedOffset::from(self.query_time_zone_offset)
                .from_local_datetime(&query_until)
                .unwrap(),
        ); // TODO
        let is_empty = query_since > query_until;
        if is_empty {
            return false;
        }

        match self.parse_path(path) {
            Err(_) => return false,
            Ok((year, month, day_of_month, file)) => {
                let optional_date = match (year, month, day_of_month) {
                    (None, None, None) => return false,
                    (None, None, Some(_)) => unreachable!(),
                    (None, Some(_), None) => unreachable!(),
                    (None, Some(_), Some(_)) => unreachable!(),
                    (Some(yyyy), None, None) => OptionalDate::from_yyyy(yyyy),
                    (Some(_), None, Some(_)) => unreachable!(),
                    (Some(yyyy), Some(mm), None) => OptionalDate::from_yyyymm(yyyy, mm),
                    (Some(yyyy), Some(mm), Some(dd)) => OptionalDate::from_yyyymmdd(yyyy, mm, dd),
                };

                let (path_since, path_until) = optional_date.naive_date_time_range();
                let path_since = DateTime::<Utc>::from_utc(path_since, Utc);
                let path_until = DateTime::<Utc>::from_utc(path_until, Utc);
                if query_until < path_since || path_until < query_since {
                    return false;
                }

                // TODO
                if let Some(file) = file {
                    let s = file.rsplit_once('.').unwrap().0;
                    let bid = BId::from_str(s).unwrap();
                    return (query_since.timestamp()..=query_until.timestamp())
                        .contains(&bid.to_timestamp());
                }
            }
        }

        true
    }

    fn parse_path<P: AsRef<Path>>(
        &self,
        path: P,
    ) -> anyhow::Result<(
        Option<Digit4>,
        Option<Digit2>,
        Option<Digit2>,
        Option<String>,
    )> {
        let relative = path.as_ref().strip_prefix(self.root_dir.as_path())?;
        let mut components = relative.components();
        Ok((
            components
                .next()
                .and_then(|c| c.as_os_str().to_str())
                .map(|s| s.parse::<Digit4>())
                .transpose()?,
            components
                .next()
                .and_then(|c| c.as_os_str().to_str())
                .map(|s| s.parse::<Digit2>())
                .transpose()?,
            components
                .next()
                .and_then(|c| c.as_os_str().to_str())
                .map(|s| s.parse::<Digit2>())
                .transpose()?,
            components
                .next()
                .and_then(|c| c.as_os_str().to_str())
                .map(|s| s.to_owned()),
        ))
    }
}

impl Iterator for ListFiles {
    type Item = io::Result<PathBuf>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(entry_path) = self.queue.pop_front() {
            if !self.match_query(entry_path.as_path()) {
                continue;
            }

            if entry_path.is_dir() {
                self.queue.extend(match read_dir_sorted(entry_path) {
                    Ok(dir_entries) => dir_entries.into_iter().map(|dir_entry| dir_entry.path()),
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

    use tempfile::{tempdir, TempDir};

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
        let path_bufs =
            ListFiles::new(tempdir.path(), Query::default(), TimeZoneOffset::default())?
                .collect::<io::Result<Vec<PathBuf>>>()?;
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

    #[test]
    fn list_files_query_test() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let root_dir = temp_dir.path().join("data_dir").join("flow");
        fs::create_dir_all(root_dir.as_path())?;
        let d20210203 = root_dir.join("2021").join("02").join("03");
        fs::create_dir_all(d20210203.as_path())?;
        let f20210203 = d20210203.as_path().join("20210203T000000Z.json");
        fs::write(f20210203.as_path(), "{}")?;
        let d20210204 = root_dir.join("2021").join("02").join("04");
        fs::create_dir_all(d20210204.as_path())?;
        let f20210204 = d20210204.as_path().join("20210204T000000Z.json");
        fs::write(f20210204.as_path(), "{}")?;
        let path_bufs = ListFiles::new(
            root_dir.as_path(),
            "date:2021-02-03".parse()?,
            TimeZoneOffset::default(),
        )?
        .collect::<io::Result<Vec<PathBuf>>>()?;
        assert_eq!(path_bufs, vec![f20210203.clone()]);
        let path_bufs = ListFiles::new(
            root_dir.as_path(),
            "date:2021-02".parse()?,
            TimeZoneOffset::default(),
        )?
        .collect::<io::Result<Vec<PathBuf>>>()?;
        assert_eq!(path_bufs, vec![f20210203, f20210204]);
        Ok(())
    }
}
