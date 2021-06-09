use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, TimeZone, Utc};
use std::{
    io,
    path::{Path, PathBuf},
    str::FromStr,
};

type DateTimeRange = (DateTime<Utc>, DateTime<Utc>);

fn utc_date_time_range(date: &str) -> DateTimeRange {
    let date = NaiveDate::from_str(date).unwrap();
    let start = date.and_hms(0, 0, 0);
    let end = date.and_hms(23, 59, 59);
    let start = Local.from_local_datetime(&start).unwrap();
    let end = Local.from_local_datetime(&end).unwrap();
    let start = DateTime::<Utc>::from(start);
    let end = DateTime::<Utc>::from(end);
    (start, end)
}

fn in_date_time_range(date_time_range: &DateTimeRange, date: &str) -> bool {
    let dt = NaiveDateTime::parse_from_str(&date[0..date.len() - 1], "%Y%m%dT%H%M%S").unwrap();
    let dt = Utc.from_utc_datetime(&dt);
    (date_time_range.0..=date_time_range.1).contains(&dt)
}

fn dirs(data_dir: &Path, date_time_range: &DateTimeRange) -> Vec<PathBuf> {
    let (start, end) = date_time_range;
    let dates = if start == end {
        vec![start]
    } else {
        vec![start, end]
    };

    dates
        .into_iter()
        .map(|date| {
            let date_string = date.naive_utc().date().to_string();
            let ymd = date_string.split('-').collect::<Vec<&str>>();
            let (y, m, d) = match ymd[..] {
                [y, m, d] => (y, m, d),
                _ => unreachable!(),
            };
            data_dir.join("flow").join(y).join(m).join(d)
        })
        .collect::<Vec<PathBuf>>()
}

fn list_files(data_dir: PathBuf, query: String) -> Vec<PathBuf> {
    let mut files = vec![];
    let date_time_range = utc_date_time_range(query.as_str());
    let dirs = dirs(data_dir.as_path(), &date_time_range);
    for dir in dirs {
        for dir_entry in dir.read_dir().unwrap() {
            let dir_entry = dir_entry.unwrap();
            let path = dir_entry.path();
            if path.extension().unwrap().to_str().unwrap() == "md"
                && in_date_time_range(
                    &date_time_range,
                    path.file_stem().unwrap().to_str().unwrap(),
                )
            {
                files.push(dir.join(path));
            }
        }
    }
    files.sort();
    files
}

pub fn list(data_dir: PathBuf, query: String, writer: &mut impl io::Write) {
    let files = list_files(data_dir, query);
    for file in files {
        writeln!(writer, "{}", file.as_path().to_str().unwrap()).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test() {
        let dir = tempdir().unwrap();
        let dir20210202 = dir.path().join("flow").join("2021").join("02").join("02");
        let dir20210203 = dir.path().join("flow").join("2021").join("02").join("03");
        fs::create_dir_all(dir20210202.as_path()).unwrap();
        fs::create_dir_all(dir20210203.as_path()).unwrap();
        let files = vec![
            dir20210202.join("20210202T145959Z.md"),
            dir20210202.join("20210202T150000Z.md"),
            dir20210202.join("20210202T235959Z.md"),
            dir20210203.join("20210203T000000Z.md"),
            dir20210203.join("20210203T145959Z.md"),
            dir20210203.join("20210203T150000Z.md"),
        ];
        for f in files.iter() {
            fs::write(f.as_path(), "").unwrap();
        }
        assert_eq!(
            list_files(dir.path().to_path_buf(), "2021-02-03".to_string()),
            files[1..1 + 4]
        );
    }
}
