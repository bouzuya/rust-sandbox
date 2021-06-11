use crate::bid::BId;
use crate::query::Query;
use chrono::NaiveDateTime;
use chrono::{DateTime, Local, NaiveDate, TimeZone, Utc};
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

#[derive(Debug, Eq, PartialEq)]
struct BMeta {
    id: BId,
    tags: Vec<String>,
    title: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct BMetaJson {
    tags: Option<Vec<String>>,
    title: Option<String>,
}

#[derive(Debug, Eq, PartialEq, serde::Serialize)]
struct BOutput {
    content_path: PathBuf,
    created_at: String,
    meta_path: PathBuf,
    tags: Vec<String>,
    title: String,
}

impl BOutput {
    fn from(bmeta: BMeta, data_dir: &Path) -> Self {
        BOutput {
            content_path: bmeta.id.to_content_path_buf(data_dir),
            created_at: Local
                .from_utc_datetime(&NaiveDateTime::from_timestamp(bmeta.id.to_timestamp(), 0))
                .to_rfc3339(),
            meta_path: bmeta.id.to_meta_path_buf(data_dir),
            tags: bmeta.tags,
            title: bmeta.title,
        }
    }
}

fn list_bids(data_dir: &Path, query: &Query) -> Vec<BId> {
    let mut bids = vec![];
    let date_time_range = utc_date_time_range(query.date.as_str());
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

fn list_bmetas(data_dir: &Path, query: &Query) -> Vec<BMeta> {
    let mut bmetas = vec![];
    let bids = list_bids(data_dir, query);
    for bid in bids {
        let meta_path_buf = bid.to_meta_path_buf(data_dir);
        let content_path_buf = bid.to_content_path_buf(data_dir);

        let json_string = fs::read_to_string(meta_path_buf.as_path()).unwrap();
        let json = serde_json::from_str::<BMetaJson>(json_string.as_str()).unwrap();
        let title = match json.title {
            Some(title) => title,
            None => {
                let file = fs::File::open(content_path_buf.as_path()).unwrap();
                let mut buf_reader = BufReader::new(file);
                let mut buf = [0; 512];
                let n = buf_reader.read(&mut buf).unwrap();
                let s = String::from_utf8_lossy(&buf[0..n]);
                s.trim_end_matches('\u{FFFD}')
                    .chars()
                    .map(|c| if c == '\n' { ' ' } else { c })
                    .take(80 - 27)
                    .collect::<String>()
            }
        };
        let bmeta = BMeta {
            id: bid,
            tags: json.tags.unwrap_or_default(),
            title,
        };
        match &query.tags {
            Some(ref tags) => {
                if tags
                    .iter()
                    .all(|tag| bmeta.tags.iter().any(|s| s.as_str() == tag))
                {
                    bmetas.push(bmeta);
                }
            }
            None => bmetas.push(bmeta),
        }
    }
    bmetas
}

pub fn list(data_dir: PathBuf, json: bool, query: String, writer: &mut impl io::Write) {
    let query = Query::from_str(query.as_str()).unwrap();
    let bmetas = list_bmetas(data_dir.as_path(), &query);
    if json {
        serde_json::to_writer(
            writer,
            &bmetas
                .into_iter()
                .map(|bmeta| BOutput::from(bmeta, &data_dir))
                .collect::<Vec<BOutput>>(),
        )
        .unwrap();
    } else {
        for bmeta in bmetas {
            writeln!(
                writer,
                "{} {}",
                bmeta.id.to_content_path_buf(&data_dir).to_str().unwrap(),
                bmeta.title
            )
            .unwrap();
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
            fs::write(f.as_path().with_extension("json"), "{}").unwrap();
        }
        let query = Query::from_str("2021-02-03").unwrap();
        assert_eq!(
            list_bmetas(dir.path(), &query)
                .into_iter()
                .map(|p| p.id.to_content_path_buf(dir.path()))
                .collect::<Vec<PathBuf>>(),
            files[1..1 + 4]
        );
        assert_eq!(
            list_bmetas(dir.path(), &query)
                .into_iter()
                .map(|p| p.title)
                .collect::<Vec<String>>(),
            vec!["1", "2", "3", "4"]
        );
    }
}
