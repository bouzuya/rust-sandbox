use chrono::{DateTime, NaiveDateTime, Utc};
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    str::FromStr,
};

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct BId(i64);

impl std::fmt::Display for BId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_naive_date_time().format("%Y%m%dT%H%M%SZ"))
    }
}

impl FromStr for BId {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.ends_with('Z') {
            let ndt =
                NaiveDateTime::parse_from_str(s, "%Y%m%dT%H%M%SZ").map_err(|_| "invalid str")?;
            Ok(BId(ndt.timestamp()))
        } else {
            let dt =
                DateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%:z").map_err(|_| "invalid str")?;
            Ok(BId(dt.timestamp()))
        }
    }
}

impl BId {
    pub fn now() -> Self {
        let utc = Utc::now();
        Self(utc.timestamp())
    }

    pub fn from_content_path(data_dir: &Path, path: &Path) -> Result<Self, &'static str> {
        Self::from_meta_path(data_dir, path.with_extension("json").as_path())
    }

    pub fn from_meta_path(data_dir: &Path, path: &Path) -> Result<Self, &'static str> {
        let p = path.strip_prefix(data_dir).map_err(|_| "invalid path")?;
        if p.extension() != Some(OsStr::new("json")) {
            return Err("invalid extension");
        }
        let s = p
            .file_stem()
            .ok_or("invalid file_stem")?
            .to_str()
            .ok_or("invalid str (file_stem)")?;
        let bid = Self::from_str(s).map_err(|_| "invalid format")?;
        let components = p
            .components()
            .map(|c| c.as_os_str().to_str().ok_or("invalid component"))
            .collect::<Result<Vec<&str>, &'static str>>()?;
        if components
            .iter()
            .take(components.len().saturating_sub(1))
            .zip(bid.to_dir_components())
            .all(|(&c1, c2)| c1 == c2.as_str())
        {
            Ok(bid)
        } else {
            Err("invalid dir components")
        }
    }

    pub fn from_timestamp(i: i64) -> Self {
        Self::from_timestamp_opt(i).expect("invalid timestamp")
    }

    pub fn from_timestamp_opt(i: i64) -> Option<Self> {
        if (0..=253402300799).contains(&i) {
            Some(Self(i))
        } else {
            None
        }
    }

    pub fn to_content_path_buf(&self, data_dir: &Path) -> PathBuf {
        self.to_meta_path_buf(data_dir).with_extension("md")
    }

    pub fn to_timestamp(&self) -> i64 {
        self.0
    }

    pub fn to_meta_path_buf(&self, data_dir: &Path) -> PathBuf {
        let components = self.to_dir_components();
        components
            .into_iter()
            .fold(data_dir.to_path_buf(), |acc, x| acc.join(x))
            .join(self.to_string())
            .with_extension("json")
    }

    fn to_naive_date_time(&self) -> NaiveDateTime {
        NaiveDateTime::from_timestamp(self.0, 0)
    }

    fn to_dir_components(&self) -> Vec<String> {
        let ndt = self.to_naive_date_time();
        let yyyy = ndt.format("%Y").to_string();
        let mm = ndt.format("%m").to_string();
        let dd = ndt.format("%d").to_string();
        vec!["flow".to_string(), yyyy, mm, dd]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDateTime;
    use std::str::FromStr;

    #[test]
    fn timestamp_convert_test() {
        let now = BId::now();
        let now1 = Utc::now().timestamp();
        assert_eq!(now.to_timestamp(), now1);
        assert_eq!(BId::from_timestamp(now1).to_timestamp(), now1);

        let min_timestamp = 0;
        let max_timestamp = 253402300799;
        assert_eq!(
            min_timestamp,
            NaiveDateTime::from_str("1970-01-01T00:00:00")
                .unwrap()
                .timestamp()
        );
        assert_eq!(
            max_timestamp,
            NaiveDateTime::from_str("9999-12-31T23:59:59")
                .unwrap()
                .timestamp()
        );

        assert_eq!(BId::from_timestamp_opt(min_timestamp - 1).is_some(), false);
        assert_eq!(BId::from_timestamp_opt(min_timestamp).is_some(), true);
        assert_eq!(BId::from_timestamp_opt(max_timestamp).is_some(), true);
        assert_eq!(BId::from_timestamp_opt(max_timestamp + 1).is_some(), false);
    }

    #[test]
    fn string_convert_test() {
        let s = "20210203T161718Z";
        assert_eq!(BId::from_str(s).unwrap().to_string(), s.to_string());
        assert_eq!(
            BId::from_str("2021-02-03T16:17:18+09:00")
                .unwrap()
                .to_string(),
            "20210203T071718Z".to_string()
        );
    }

    #[test]
    fn path_buf_convert_test() {
        let data_dir = PathBuf::from("/");
        let content_path_buf = PathBuf::from("/flow/2021/02/03/20210203T000000Z.md");
        let bid = BId::from_content_path(data_dir.as_path(), content_path_buf.as_path()).unwrap();
        assert_eq!(
            bid.to_content_path_buf(data_dir.as_path()),
            content_path_buf
        );
        assert_eq!(bid.to_string(), "20210203T000000Z");

        let data_dir = PathBuf::from("/");
        let meta_path_buf = PathBuf::from("/flow/2021/02/03/20210203T000000Z.json");
        let bid = BId::from_meta_path(data_dir.as_path(), meta_path_buf.as_path()).unwrap();
        assert_eq!(bid.to_meta_path_buf(data_dir.as_path()), meta_path_buf);
        assert_eq!(bid.to_string(), "20210203T000000Z");

        let data_dir = PathBuf::from("/data_dir");
        let meta_path_buf = PathBuf::from("/data_dir/flow/2021/02/03/20210203T000000Z.json");
        let bid = BId::from_meta_path(data_dir.as_path(), meta_path_buf.as_path()).unwrap();
        assert_eq!(bid.to_meta_path_buf(data_dir.as_path()), meta_path_buf);
        assert_eq!(bid.to_string(), "20210203T000000Z");
    }
}
