use crate::bid::BId;
use crate::bmeta::BMeta;
use anyhow::{anyhow, bail, Context};
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, TimeZone, Utc};
use std::{
    ffi::OsStr,
    fs,
    io::{BufReader, Read},
    path::{Path, PathBuf},
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

fn to_dir_components(id: &BId) -> Vec<String> {
    let ndt = NaiveDateTime::from_timestamp(id.to_timestamp(), 0);
    let yyyy = ndt.format("%Y").to_string();
    let mm = ndt.format("%m").to_string();
    let dd = ndt.format("%d").to_string();
    vec!["flow".to_string(), yyyy, mm, dd]
}

impl BRepository {
    pub fn new(data_dir: PathBuf) -> Self {
        Self { data_dir }
    }

    // TODO: hide path ?
    pub fn find_by_content_path(&self, path: &Path) -> anyhow::Result<BId> {
        // TODO: using fs
        self.find_by_meta_path(path.with_extension("json").as_path())
    }

    // TODO: hide path ?
    pub fn find_by_meta_path(&self, path: &Path) -> anyhow::Result<BId> {
        // TODO: using fs
        let p = path
            .strip_prefix(self.data_dir.as_path())
            .with_context(|| "invalid path")?;
        if p.extension() != Some(OsStr::new("json")) {
            bail!("invalid extension");
        }
        let s = p
            .file_stem()
            .with_context(|| "invalid file_stem")?
            .to_str()
            .with_context(|| "invalid str (file_stem)")?;
        let bid = BId::from_str(s).map_err(|_| anyhow!("invalid format"))?;
        let components = p
            .components()
            .map(|c| c.as_os_str().to_str().with_context(|| "invalid component"))
            .collect::<anyhow::Result<Vec<&str>>>()?;
        if components
            .iter()
            .take(components.len().saturating_sub(1))
            .zip(to_dir_components(&bid))
            .all(|(&c1, c2)| c1 == c2.as_str())
        {
            Ok(bid)
        } else {
            bail!("invalid dir components")
        }
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
                if let Ok(bid) = self.find_by_meta_path(path.as_path()) {
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
        let meta_path_buf = self.to_meta_path_buf(&id);
        let content_path_buf = self.to_content_path_buf(&id);

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

    pub fn to_content_path_buf(&self, id: &BId) -> PathBuf {
        self.to_meta_path_buf(id).with_extension("md")
    }

    pub fn to_meta_path_buf(&self, id: &BId) -> PathBuf {
        let components = to_dir_components(&id);
        components
            .into_iter()
            .fold(self.data_dir.to_path_buf(), |acc, x| acc.join(x))
            .join(id.to_string())
            .with_extension("json")
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_buf_convert_test() {
        let data_dir = PathBuf::from("/");
        let repository = BRepository::new(data_dir);
        let content_path_buf = PathBuf::from("/flow/2021/02/03/20210203T000000Z.md");
        let bid = repository
            .find_by_content_path(content_path_buf.as_path())
            .unwrap();
        assert_eq!(repository.to_content_path_buf(&bid), content_path_buf);

        let meta_path_buf = PathBuf::from("/flow/2021/02/03/20210203T000000Z.json");
        let bid = repository
            .find_by_meta_path(meta_path_buf.as_path())
            .unwrap();
        assert_eq!(repository.to_meta_path_buf(&bid), meta_path_buf);

        let data_dir = PathBuf::from("/data_dir");
        let repository = BRepository::new(data_dir);
        let meta_path_buf = PathBuf::from("/data_dir/flow/2021/02/03/20210203T000000Z.json");
        let bid = repository
            .find_by_meta_path(meta_path_buf.as_path())
            .unwrap();
        assert_eq!(repository.to_meta_path_buf(&bid), meta_path_buf);
    }
}
