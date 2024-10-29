use crate::DateLike;
use anyhow::Context;
use bbn_repository::BbnRepository;
use date_range::date::Date;

use crate::config_repository::ConfigRepository;

pub struct Params {
    pub date_like: DateLike,
}

pub fn run(Params { date_like }: Params) -> anyhow::Result<()> {
    let config_repository = ConfigRepository::new()?;
    let config = config_repository.load()?;
    let data_dir = config.data_dir().to_path_buf();
    let path_buf = config
        .link_completion_rules_file()
        .context("link_completion_rules_file is null")?;
    let rules = markdown_link_helper::build_rules(&path_buf)?;

    let repository = BbnRepository::new(data_dir);
    let date = Date::from(date_like);
    let entry_id = repository.find_id_by_date(date)?;
    let entry = entry_id
        .as_ref()
        .and_then(|entry_id| repository.find_entry_by_id(entry_id).transpose())
        .transpose()?;
    let (_, _, content) = entry_id
        .and_then(|entry_id| {
            entry.map(|entry| (entry_id, entry.meta().clone(), entry.content().to_string()))
        })
        .context("not found")?;

    let results = markdown_link_helper::run(&rules, &content);
    for (link, replaced) in results {
        match replaced {
            None => eprintln!("'{}' is a broken link", link),
            Some(replaced) => println!("{}", replaced),
        }
    }

    Ok(())
}
