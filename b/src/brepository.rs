use crate::bid::BId;
use crate::bmeta::BMeta;
use anyhow::Context;
use chrono::{DateTime, Local, NaiveDate, TimeZone, Utc};
use std::{
    fs,
    io::{BufReader, Read},
    path::PathBuf,
    str::FromStr,
};

type DateTimeRange = (DateTime<Utc>, DateTime<Utc>);

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct BMetaJson {
    tags: Option<Vec<String>>,
    title: Option<String>,
}

pub struct BRepository {
    data_dir: PathBuf,
}

fn utc_date_time_range(date: &str) -> anyhow::Result<DateTimeRange> {
    let date = NaiveDate::from_str(date)?;
    let start = date.and_hms(0, 0, 0);
    let end = date.and_hms(23, 59, 59);
    let start = Local
        .from_local_datetime(&start)
        .single()
        .with_context(|| "invalid local datetime")?;
    let end = Local
        .from_local_datetime(&end)
        .single()
        .with_context(|| "invalid local datetime")?;
    let start = DateTime::<Utc>::from(start);
    let end = DateTime::<Utc>::from(end);
    Ok((start, end))
}

impl BRepository {
    pub fn new(data_dir: PathBuf) -> Self {
        Self { data_dir }
    }

    pub fn find_ids(&self, date: &str) -> anyhow::Result<Vec<BId>> {
        let mut bids = vec![];
        let date_time_range = utc_date_time_range(date)?;
        let timestamp_range =
            date_time_range.0.naive_utc().timestamp()..=date_time_range.1.naive_utc().timestamp();
        let dirs = self.dirs(&date_time_range);
        for dir in dirs {
            if !dir.exists() {
                continue;
            }
            for dir_entry in dir.read_dir()? {
                let dir_entry = dir_entry?;
                let path = dir_entry.path();
                if let Ok(bid) = BId::from_meta_path(self.data_dir.as_path(), path.as_path()) {
                    if timestamp_range.contains(&bid.to_timestamp()) {
                        bids.push(bid);
                    }
                }
            }
        }
        bids.sort();
        Ok(bids)
    }

    pub fn find_meta(&self, id: BId) -> anyhow::Result<Option<BMeta>> {
        let meta_path_buf = id.to_meta_path_buf(self.data_dir.as_path());
        let content_path_buf = id.to_content_path_buf(self.data_dir.as_path());

        if !meta_path_buf.exists() {
            return Ok(None);
        }

        let json_string = fs::read_to_string(meta_path_buf.as_path())?;
        let json = serde_json::from_str::<BMetaJson>(json_string.as_str())?;
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
        Ok(Some(BMeta {
            id,
            tags: json.tags.unwrap_or_default(),
            title,
        }))
    }

    fn dirs(&self, date_time_range: &DateTimeRange) -> Vec<PathBuf> {
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
                self.data_dir.join("flow").join(y).join(m).join(d)
            })
            .collect::<Vec<PathBuf>>()
    }
}
