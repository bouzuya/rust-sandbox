use crate::bbn_repository::BbnRepository;
use anyhow::bail;
use date_range::date::Date;
use std::path::PathBuf;

pub fn login() -> anyhow::Result<()> {
    let repository = BbnRepository::new(data_dir);
    let entry_id = repository.find_id_by_date(date)?;
    let entry = entry_id
        .as_ref()
        .and_then(|entry_id| repository.find_entry_by_id(entry_id).transpose())
        .transpose()?;
    match (entry_id, entry) {
        (Some(entry_id), Some((entry_meta, entry_content))) => {
            let url = format!(
                "https://blog.bouzuya.net/{}/",
                entry_id.date().to_string().replace('-', "/")
            );
            if web {
                open::that(url)?;
            } else {
                println!(
                    "{}{} {} {}",
                    entry_id.date(),
                    entry_id
                        .id_title()
                        .map(|s| format!(" {}", s))
                        .unwrap_or_default(),
                    entry_meta.title,
                    url
                );
                println!("{}", entry_content);
            }
        }
        _ => bail!("not found"),
    }
    Ok(())
}
