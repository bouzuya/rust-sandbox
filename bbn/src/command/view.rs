use crate::{bbn_repository::BbnRepository, config_repository::ConfigRepository};
use anyhow::{bail, Context};
use date_range::date::Date;

pub fn view(date: Date, web: bool) -> anyhow::Result<()> {
    let config_repository = ConfigRepository::new();
    let config = config_repository
        .load()
        .context("The configuration file does not found. Use `bbn config` command.")?;
    let data_dir = config.data_dir().to_path_buf();

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
