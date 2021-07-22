use anyhow::Context;

use crate::{
    bbn_repository::BbnRepository, config_repository::ConfigRepository, data::EntryId, query::Query,
};
use std::convert::TryFrom;

pub fn list(json: bool, query: String) -> anyhow::Result<()> {
    let config_repository = ConfigRepository::new();
    let config = config_repository
        .load()
        .context("The configuration file does not found. Use `bbn config` command.")?;
    let data_dir = config.data_dir().to_path_buf();

    let bbn_repository = BbnRepository::new(data_dir);
    let query = Query::try_from(query.as_str()).unwrap();
    let mut entry_ids = bbn_repository.find_ids_by_query(query)?;
    entry_ids.sort();
    entry_ids.reverse();
    let mut output = vec![];
    for entry_id in entry_ids {
        let entry_meta = bbn_repository
            .find_meta_by_id(&entry_id)?
            .context("meta not found")?;
        if json {
            output.push(format!(
                r#"{{"date":"{}","title":"{}","url":"{}"}}"#,
                entry_id.date(),
                entry_meta.title,
                entry_url(&entry_id),
            ));
        } else {
            output.push(format!(
                "{} {} <{}>",
                entry_id.date(),
                entry_meta.title,
                entry_url(&entry_id),
            ));
        }
    }
    let output = if json {
        let mut s = String::new();
        s.push('[');
        s.push_str(&output.join(","));
        s.push(']');
        s
    } else {
        output.join("\n")
    };
    println!("{}", output);
    Ok(())
}

fn entry_url(entry_id: &EntryId) -> String {
    format!(
        "https://blog.bouzuya.net/{}/",
        entry_id.date().to_string().replace('-', "/")
    )
}
