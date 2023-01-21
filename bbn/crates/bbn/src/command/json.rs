use anyhow::Context;
use bbn_repository::{BbnRepository, Query};
use std::convert::TryFrom;

use crate::config_repository::ConfigRepository;

pub fn run() -> anyhow::Result<()> {
    let config_repository = ConfigRepository::new()?;
    let config = config_repository
        .load()
        .context("The configuration file does not found. Use `bbn config` command.")?;
    let data_dir = config.data_dir().to_path_buf();

    let bbn_repository = BbnRepository::new(data_dir);
    let query = Query::try_from("date:1970-01-01/9999-12-31")?;
    let mut entry_ids = bbn_repository.find_ids_by_query(query)?;
    entry_ids.sort();
    for entry_id in entry_ids {
        // TODO
        println!("{}", entry_id);
    }
    Ok(())
}
