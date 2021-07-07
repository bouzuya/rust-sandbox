use crate::bbn_repository::BbnRepository;
use anyhow::bail;
use date_range::date::Date;
use std::path::PathBuf;

pub fn view(data_dir: PathBuf, date: Date) -> anyhow::Result<()> {
    let repository = BbnRepository::new(data_dir);
    let entry_id = repository.find_id_by_date(date)?;
    let entry_meta = entry_id
        .as_ref()
        .and_then(|entry_id| repository.find_meta_by_id(entry_id).transpose())
        .transpose()?;
    match (entry_id, entry_meta) {
        (Some(entry_id), Some(entry_meta)) => {
            println!(
                "{}{} {} https://blog.bouzuya.net/{}/",
                entry_id.date(),
                entry_id
                    .id_title()
                    .map(|s| format!(" {}", s))
                    .unwrap_or_default(),
                entry_meta.title,
                entry_id.date().to_string().replace('-', "/")
            );
        }
        _ => bail!("not found"),
    }
    Ok(())
}
