use crate::brepository::BRepositoryImpl;
use crate::TimeZoneOffset;
use anyhow::anyhow;
use chrono::{Local, NaiveDateTime, TimeZone};
use entity::BMeta;
use std::{io, path::PathBuf, str::FromStr};
use use_case::{BRepository, HasBRepository, HasListUseCase, ListUseCase, Query};

#[derive(Debug, Eq, PartialEq, serde::Serialize)]
struct BOutput {
    content_path: PathBuf,
    created_at: String,
    id: String,
    meta_path: PathBuf,
    tags: Vec<String>,
    title: String,
}

impl BOutput {
    fn from(bmeta: BMeta, repository: &BRepositoryImpl) -> Self {
        BOutput {
            content_path: repository.to_content_path_buf(&bmeta.id),
            created_at: Local
                .from_utc_datetime(&NaiveDateTime::from_timestamp(bmeta.id.to_timestamp(), 0))
                .to_rfc3339(),
            id: bmeta.id.to_string(),
            meta_path: repository.to_meta_path_buf(&bmeta.id),
            tags: bmeta.tags,
            title: bmeta.title,
        }
    }
}

// FIXME:
struct App {
    brepository: BRepositoryImpl,
}

impl HasBRepository for App {
    type BRepository = BRepositoryImpl;

    fn b_repository(&self) -> &Self::BRepository {
        &self.brepository
    }
}

impl HasListUseCase for App {
    type ListUseCase = App;

    fn list_use_case(&self) -> &Self::ListUseCase {
        self
    }
}

pub fn list(
    data_dir: PathBuf,
    json: bool,
    query: String,
    time_zone_offset: Option<String>,
    writer: &mut impl io::Write,
) -> anyhow::Result<()> {
    let query = Query::from_str(query.as_str()).map_err(|_| anyhow!("invalid query"))?;
    let time_zone_offset = match time_zone_offset {
        Some(s) => {
            TimeZoneOffset::from_str(s.as_str()).map_err(|_| anyhow!("invalid time_zone_offset"))?
        }
        None => TimeZoneOffset::default(),
    };
    let app = App {
        brepository: BRepositoryImpl::new(data_dir, time_zone_offset),
    };
    let bmetas = app.list_use_case().handle(&query)?;
    if json {
        serde_json::to_writer(
            writer,
            &bmetas
                .into_iter()
                .map(|bmeta| BOutput::from(bmeta, app.b_repository()))
                .collect::<Vec<BOutput>>(),
        )?;
        Ok(())
    } else {
        for bmeta in bmetas {
            writeln!(
                writer,
                "{} {}",
                app.b_repository()
                    .to_content_path_buf(&bmeta.id)
                    .to_str()
                    .unwrap(),
                bmeta.title
            )?;
        }
        Ok(())
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
        let repository = BRepositoryImpl::new(
            dir.path().to_path_buf(),
            TimeZoneOffset::from_str("+09:00").unwrap(),
        );
        let app = App {
            brepository: repository,
        };
        let use_case = app.list_use_case();
        assert_eq!(
            use_case
                .handle(&query)
                .unwrap()
                .into_iter()
                .map(|p| app.b_repository().to_content_path_buf(&p.id))
                .collect::<Vec<PathBuf>>(),
            files[1..1 + 4]
        );
        assert_eq!(
            use_case
                .handle(&query)
                .unwrap()
                .into_iter()
                .map(|p| p.title)
                .collect::<Vec<String>>(),
            vec!["1", "2", "3", "4"]
        );
    }
}
