use crate::bmeta::BMeta;
use crate::brepository::BRepository;
use crate::query::Query;
use chrono::NaiveDateTime;
use chrono::{Local, TimeZone};
use std::{io, path::PathBuf, str::FromStr};

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
    fn from(bmeta: BMeta, repository: &BRepository) -> Self {
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

fn list_bmetas(repository: &BRepository, query: &Query) -> Vec<BMeta> {
    let mut bmetas = vec![];
    let bids = repository.find_ids(query.date.as_str()).unwrap();
    for bid in bids {
        let bmeta = repository.find_meta(bid).unwrap().unwrap();
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
    let repository = BRepository::new(data_dir);
    let bmetas = list_bmetas(&repository, &query);
    if json {
        serde_json::to_writer(
            writer,
            &bmetas
                .into_iter()
                .map(|bmeta| BOutput::from(bmeta, &repository))
                .collect::<Vec<BOutput>>(),
        )
        .unwrap();
    } else {
        for bmeta in bmetas {
            writeln!(
                writer,
                "{} {}",
                repository.to_content_path_buf(&bmeta.id).to_str().unwrap(),
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
        let repository = BRepository::new(dir.path().to_path_buf());
        assert_eq!(
            list_bmetas(&repository, &query)
                .into_iter()
                .map(|p| repository.to_content_path_buf(&p.id))
                .collect::<Vec<PathBuf>>(),
            files[1..1 + 4]
        );
        assert_eq!(
            list_bmetas(&repository, &query)
                .into_iter()
                .map(|p| p.title)
                .collect::<Vec<String>>(),
            vec!["1", "2", "3", "4"]
        );
    }
}
