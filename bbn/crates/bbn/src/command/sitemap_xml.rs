use std::{fs::File, io::BufWriter, path::PathBuf};

use anyhow::Context;
use bbn_repository::{BbnRepository, Query};
use sitemap_xml::writer::{SitemapWriter, Url};

use crate::config_repository::ConfigRepository;

pub fn run(out_dir: PathBuf) -> anyhow::Result<()> {
    let config_repository = ConfigRepository::new()?;
    let config = config_repository
        .load()
        .context("The configuration file does not found. Use `bbn config` command.")?;
    let data_dir = config.data_dir().to_path_buf();

    let bbn_repository = BbnRepository::new(data_dir);
    let query = Query::try_from("date:1970-01-01/9999-12-31")?;
    let entry_ids = bbn_repository.find_ids_by_query(query)?;

    let path = out_dir.join("sitemap.xml");
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    let mut writer = SitemapWriter::start(writer)?;

    for entry_id in entry_ids {
        let date = entry_id.date();
        let yyyy = date.year().to_string();
        let mm = date.month().to_string();
        let dd = date.day_of_month().to_string();
        // TODO: lastmod
        writer.write(Url::loc(
            format!("https://blog.bouzuya.net/{yyyy}/{mm}/{dd}/").as_str(),
        )?)?;
    }

    writer.end()?;

    Ok(())
}
