use anyhow::anyhow;
use entity::BMeta;
use limited_date_time::{Instant, OffsetDateTime, TimeZoneOffset};
use std::{convert::TryFrom, io, path::PathBuf, str::FromStr};
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
    fn from(bmeta: BMeta, repository: &impl BRepository) -> Self {
        BOutput {
            content_path: repository.to_content_path_buf(&bmeta.id),
            // TODO: unwrap OffsetDateTime::from_instant
            created_at: OffsetDateTime::from_instant(
                // TODO: unwrap BId -> Instant
                Instant::try_from(bmeta.id.to_timestamp() as u64).unwrap(),
                // TODO: unwrap TimeZoneOffset::system_default()
                TimeZoneOffset::from_str("+09:00").unwrap(),
            )
            .unwrap()
            .to_string(),
            id: bmeta.id.to_string(),
            meta_path: repository.to_meta_path_buf(&bmeta.id),
            tags: bmeta.tags,
            title: bmeta.title,
        }
    }
}

pub fn list<T: HasListUseCase + HasBRepository>(
    app: &T,
    json: bool,
    query: String,
    writer: &mut impl io::Write,
) -> anyhow::Result<()> {
    let query = Query::from_str(query.as_str()).map_err(|_| anyhow!("invalid query"))?;
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
