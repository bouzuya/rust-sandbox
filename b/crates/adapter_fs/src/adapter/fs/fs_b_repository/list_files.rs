use std::{
    collections::VecDeque,
    convert::TryFrom,
    ffi::OsStr,
    fs::{self, DirEntry},
    io,
    path::{Path, PathBuf},
    str::FromStr,
};

use entity::BId;
use limited_date_time::{Instant, OffsetDateTime, TimeZoneOffset};
use query::{Digit2, Digit4, OptionalDate, Query};

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
        let (query_since, query_until) = {
            let (query_since, query_until) = self.query.naive_date_time_range();
            let is_empty = query_since > query_until;
            if is_empty {
                return false;
            }
            let query_since =
                OffsetDateTime::new(query_since, self.query_time_zone_offset).instant();
            let query_until =
                OffsetDateTime::new(query_until, self.query_time_zone_offset).instant();
            (query_since, query_until)
        };

        let out_of_range_dir = self
            .parse_path_date_range(path.as_ref())
            .map(|(path_since, path_until)| !(query_until < path_since || path_until < query_since))
            .unwrap_or(false);
        if !out_of_range_dir {
            return false;
        }

        let extension = path.as_ref().extension();
        if extension == Some(OsStr::new("json")) || extension == Some(OsStr::new("md")) {
            if let Some(s) = path.as_ref().file_stem().and_then(|s| s.to_str()) {
                let out_of_range_bid = !BId::from_str(s)
                    .map(|bid| {
                        (u64::from(query_since)..=u64::from(query_until)).contains(&u64::from(
                            // TODO: unwrap BId -> Instant
                            Instant::try_from(bid.to_timestamp() as u64).unwrap(),
                        ))
                    })
                    .unwrap_or(false);
                if out_of_range_bid {
                    return false;
                }
            }
        }

        true
    }

    fn parse_path_date_range<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<(Instant, Instant)> {
        let (year, month, day_of_month) = self.parse_path(path)?;
        let optional_date = match (year, month, day_of_month) {
            (None, None, None) => anyhow::bail!("root_directory"), // TODO
            (None, None, Some(_)) => unreachable!(),
            (None, Some(_), None) => unreachable!(),
            (None, Some(_), Some(_)) => unreachable!(),
            (Some(yyyy), None, None) => OptionalDate::from_yyyy(yyyy),
            (Some(_), None, Some(_)) => unreachable!(),
            (Some(yyyy), Some(mm), None) => OptionalDate::from_yyyymm(yyyy, mm),
            (Some(yyyy), Some(mm), Some(dd)) => OptionalDate::from_yyyymmdd(yyyy, mm, dd),
        };

        let (since, until) = optional_date.naive_date_time_range();
        let since = OffsetDateTime::new(since, self.query_time_zone_offset).instant();
        let until = OffsetDateTime::new(until, self.query_time_zone_offset).instant();
        Ok((since, until))
    }

    fn parse_path<P: AsRef<Path>>(
        &self,
        path: P,
    ) -> anyhow::Result<(Option<Digit4>, Option<Digit2>, Option<Digit2>)> {
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

    use tempfile::tempdir;

    #[test]
    fn read_dir_sorted_test() -> anyhow::Result<()> {
        let temp_dir = {
            let temp_dir = tempfile::tempdir()?;
            let root_dir = temp_dir.path();
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
            temp_dir
        };
        let dir_entries = read_dir_sorted(temp_dir.path())?;
        assert_eq!(
            dir_entries
                .into_iter()
                .map(|e| e.path())
                .collect::<Vec<PathBuf>>(),
            vec![temp_dir.path().join("dir1"), temp_dir.path().join("dir2")]
        );
        Ok(())
    }

    #[test]
    fn list_files_test() -> anyhow::Result<()> {
        let temp_dir = {
            let temp_dir = tempfile::tempdir()?;
            let root_dir = temp_dir.path().join("flow");
            let d20210203 = root_dir.join("2021").join("02").join("03");
            fs::create_dir_all(d20210203.as_path())?;
            let file1 = d20210203.join("20210203T000000Z.md");
            fs::write(file1, "file1 contents")?;
            temp_dir
        };
        let path_bufs = ListFiles::new(
            temp_dir.path().join("flow"),
            Query::default(),
            // TODO: unwrap TimeZoneOffset::system_default()
            TimeZoneOffset::from_str("+09:00").unwrap(),
        )?
        .collect::<io::Result<Vec<PathBuf>>>()?;
        assert_eq!(
            path_bufs,
            vec![temp_dir
                .path()
                .join("flow")
                .join("2021")
                .join("02")
                .join("03")
                .join("20210203T000000Z.md"),]
        );
        Ok(())
    }

    #[test]
    fn list_files_query_test() -> anyhow::Result<()> {
        // TODO: TimeZoneOffset::system_default()
        let time_zone_offset = TimeZoneOffset::from_str("+09:00")?;
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
            time_zone_offset,
        )?
        .collect::<io::Result<Vec<PathBuf>>>()?;
        assert_eq!(path_bufs, vec![f20210203.clone()]);
        let path_bufs = ListFiles::new(
            root_dir.as_path(),
            "date:2021-02".parse()?,
            time_zone_offset,
        )?
        .collect::<io::Result<Vec<PathBuf>>>()?;
        assert_eq!(path_bufs, vec![f20210203, f20210204]);
        Ok(())
    }
}
