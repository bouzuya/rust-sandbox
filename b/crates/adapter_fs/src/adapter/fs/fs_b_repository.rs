mod list_files;

use self::list_files::ListFiles;
use anyhow::{bail, Context};
use entity::{BId, BMeta};
use limited_date_time::{Date, DateTime, Instant, OffsetDateTime, Time, TimeZoneOffset};
use std::{
    convert::TryFrom,
    ffi::OsStr,
    fs::{self, File},
    io::{BufReader, Read},
    path::{Path, PathBuf},
    str::FromStr,
};
use use_case::BRepository;

type DateTimeRange = (Instant, Instant);

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct BMetaJson {
    tags: Option<Vec<String>>,
    title: Option<String>,
}

pub struct FsBRepository {
    data_dir: PathBuf,
    time_zone_offset: TimeZoneOffset,
}

fn to_dir_components(id: &BId) -> Vec<String> {
    // TODO: unwrap BId -> Instant
    let instant = Instant::try_from(id.to_timestamp() as u64).unwrap();
    // TODO: unwrap Instant -> OffsetDateTime
    let offset_date_time = OffsetDateTime::from_instant(instant, TimeZoneOffset::utc()).unwrap();
    let date = offset_date_time.date_time().date();
    let yyyy = date.year().to_string();
    let mm = date.month().to_string();
    let dd = date.day_of_month().to_string();
    vec!["flow".to_string(), yyyy, mm, dd]
}

impl BRepository for FsBRepository {
    // TODO: hide path ?
    fn find_by_content_path(&self, path: &Path) -> anyhow::Result<BId> {
        // TODO: using fs
        self.find_by_meta_path(path.with_extension("json").as_path())
    }

    // TODO: hide path ?
    fn find_by_meta_path(&self, path: &Path) -> anyhow::Result<BId> {
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
        let bid = BId::from_str(s)?;
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

    fn find_content(&self, id: BId) -> anyhow::Result<Option<String>> {
        let content_path_buf = self.to_content_path_buf(&id);
        if !content_path_buf.exists() {
            return Ok(None);
        }

        Ok(Some(fs::read_to_string(content_path_buf.as_path())?))
    }

    fn find_ids(&self, date: &str) -> anyhow::Result<Vec<BId>> {
        let mut bids = vec![];
        let date_time_range = self.utc_date_time_range(date)?;
        let timestamp_range = i64::from(date_time_range.0)..=i64::from(date_time_range.1);
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

    fn find_meta(&self, id: BId) -> anyhow::Result<Option<BMeta>> {
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
                let file = File::open(content_path_buf.as_path())?;
                let mut buf_reader = BufReader::new(file);
                let mut buf = [0; 512];
                let n = buf_reader.read(&mut buf)?;
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

    fn to_content_path_buf(&self, id: &BId) -> PathBuf {
        self.to_meta_path_buf(id).with_extension("md")
    }

    fn to_meta_path_buf(&self, id: &BId) -> PathBuf {
        let components = to_dir_components(id);
        components
            .into_iter()
            .fold(self.data_dir.to_path_buf(), |acc, x| acc.join(x))
            .join(id.to_string())
            .with_extension("json")
    }
}

impl FsBRepository {
    pub fn new(data_dir: PathBuf, time_zone_offset: TimeZoneOffset) -> Self {
        Self {
            data_dir,
            time_zone_offset,
        }
    }

    pub fn find_all_ids(&self) -> anyhow::Result<impl Iterator<Item = anyhow::Result<BId>>> {
        self.find_ids_by_query(query::Query::default())
    }

    pub fn find_ids_iter(
        &self,
        date: &str,
    ) -> anyhow::Result<impl Iterator<Item = anyhow::Result<BId>>> {
        let q = query::Query::from_str(&format!("date:{}", date))?;
        self.find_ids_by_query(q)
    }

    pub fn find_ids_by_query(
        &self,
        query: query::Query,
    ) -> anyhow::Result<impl Iterator<Item = anyhow::Result<BId>>> {
        let files = ListFiles::new(self.data_dir.join("flow"), query, self.time_zone_offset)?;
        Ok(files
            .map(|f| {
                let p = f?;
                if p.extension() != Some(OsStr::new("json")) {
                    return Ok(None);
                }

                let s = p
                    .file_stem()
                    .with_context(|| "invalid file_stem")?
                    .to_str()
                    .with_context(|| "invalid str (file_stem)")?;
                let bid = BId::from_str(s)?;
                Ok(Some(bid))
            })
            .filter_map(|x| match x {
                Ok(None) => None,
                Ok(Some(bid)) => Some(Ok(bid)),
                Err(err) => Some(Err(err)),
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
                // TODO: unwrap Instant -> OffsetDateTime
                let date_string = OffsetDateTime::from_instant(*date, TimeZoneOffset::utc())
                    .unwrap()
                    .date_time()
                    .date()
                    .to_string();
                let ymd = date_string.split('-').collect::<Vec<&str>>();
                let (y, m, d) = match ymd[..] {
                    [y, m, d] => (y, m, d),
                    _ => unreachable!(),
                };
                self.data_dir.join("flow").join(y).join(m).join(d)
            })
            .collect::<Vec<PathBuf>>()
    }

    fn utc_date_time_range(&self, date: &str) -> anyhow::Result<DateTimeRange> {
        let date = Date::from_str(date)?;
        let start = DateTime::from_date_time(date, Time::from_str("00:00:00")?);
        let end = DateTime::from_date_time(date, Time::from_str("23:59:59")?);
        let start = OffsetDateTime::new(start, self.time_zone_offset);
        let end = OffsetDateTime::new(end, self.time_zone_offset);
        let start = start.instant();
        let end = end.instant();
        Ok((start, end))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_buf_convert_test() {
        let time_zone_offset = TimeZoneOffset::from_str("+09:00").unwrap();
        let data_dir = PathBuf::from("/");
        let repository = FsBRepository::new(data_dir, time_zone_offset);
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
        let repository = FsBRepository::new(data_dir, time_zone_offset);
        let meta_path_buf = PathBuf::from("/data_dir/flow/2021/02/03/20210203T000000Z.json");
        let bid = repository
            .find_by_meta_path(meta_path_buf.as_path())
            .unwrap();
        assert_eq!(repository.to_meta_path_buf(&bid), meta_path_buf);
    }
}
