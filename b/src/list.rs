use crate::bid::BId;
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, TimeZone, Utc};
use std::{
    fs,
    io::{self, BufReader, Read},
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

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct BMeta {
    title: String,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize)]
struct B {
    md_path: PathBuf,
    title: String,
}

impl B {
    fn new(path_buf: PathBuf, title: String) -> Self {
        Self {
            md_path: path_buf,
            title,
        }
    }

    fn path(&self) -> &Path {
        self.md_path.as_path()
    }

    fn title(&self) -> &str {
        self.title.as_str()
    }
}

fn list_bids(data_dir: &Path, query: String) -> Vec<BId> {
    let mut bids = vec![];
    let date_time_range = utc_date_time_range(query.as_str());
    let timestamp_range =
        date_time_range.0.naive_utc().timestamp()..=date_time_range.1.naive_utc().timestamp();
    let dirs = dirs(data_dir, &date_time_range);
    for dir in dirs {
        if !dir.exists() {
            continue;
        }
        for dir_entry in dir.read_dir().unwrap() {
            let dir_entry = dir_entry.unwrap();
            let path = dir_entry.path();
            if let Ok(bid) = BId::from_meta_path(data_dir, path.as_path()) {
                if timestamp_range.contains(&bid.to_timestamp()) {
                    bids.push(bid);
                }
            }
        }
    }
    bids.sort();
    bids
}

fn list_bs(data_dir: PathBuf, query: String) -> Vec<B> {
    let mut files = vec![];
    let bids = list_bids(data_dir.as_path(), query);
    for bid in bids {
        let path = bid
            .to_meta_path_buf(data_dir.as_path())
            .with_extension("md");
        let file = fs::File::open(path.as_path()).unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut buf = [0; 512];
        let n = buf_reader.read(&mut buf).unwrap();
        let s = String::from_utf8_lossy(&buf[0..n]);
        let s = s
            .trim_end_matches('\u{FFFD}')
            .chars()
            .map(|c| if c == '\n' { ' ' } else { c })
            .take(80 - 27)
            .collect::<String>();
        let meta = BMeta { title: s };
        files.push(B::new(path, meta.title));
    }
    files
}

pub fn list(data_dir: PathBuf, json: bool, query: String, writer: &mut impl io::Write) {
    let bs = list_bs(data_dir, query);
    if json {
        serde_json::to_writer(writer, &bs).unwrap();
    } else {
        for b in bs {
            writeln!(writer, "{} {}", b.path().to_str().unwrap(), b.title()).unwrap();
        }
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
        for (i, f) in files.iter().enumerate() {
            fs::write(f.as_path(), format!("{}", i)).unwrap();
            fs::write(f.as_path().with_extension("json"), "").unwrap();
        }
        assert_eq!(
            list_bs(dir.path().to_path_buf(), "2021-02-03".to_string())
                .into_iter()
                .map(|p| p.path().to_path_buf())
                .collect::<Vec<PathBuf>>(),
            files[1..1 + 4]
        );
        assert_eq!(
            list_bs(dir.path().to_path_buf(), "2021-02-03".to_string())
                .into_iter()
                .map(|p| p.title().to_string())
                .collect::<Vec<String>>(),
            vec!["1", "2", "3", "4"]
        );
    }
}
