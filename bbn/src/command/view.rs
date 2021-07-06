use crate::bbn_repository::BbnRepository;
use anyhow::bail;
use date_range::date::Date;
use std::path::PathBuf;

pub fn view(data_dir: PathBuf, date: Date) -> anyhow::Result<()> {
    let repository = BbnRepository::new(data_dir);
    match repository.find_by_date(date)? {
        Some(entry) => {
            println!(
                "{}{} {} https://blog.bouzuya.net/{}/",
                entry.entry_id.date(),
                entry
                    .entry_id
                    .id_title()
                    .map(|s| format!(" {}", s))
                    .unwrap_or_default(),
                entry.title,
                date.to_string().replace('-', "/")
            );
        }
        None => bail!("not found"),
    }
    Ok(())
}
