mod data;
mod entry;
mod parse;
mod template;
mod template_entry;

use self::data::build_data;
use self::entry::list_entries;
use self::template_entry::TemplateEntry;
use std::{convert::TryFrom, env, fs::File, io, path::PathBuf};

pub fn new(data_file: PathBuf, template: PathBuf) -> anyhow::Result<()> {
    let data = if data_file == PathBuf::from("-").as_path() {
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        build_data(&mut handle)
    } else {
        build_data(&mut File::open(&data_file).unwrap())
    }
    .unwrap();
    let entries = list_entries(template.as_path()).unwrap();
    let templates = entries
        .iter()
        .map(TemplateEntry::try_from)
        .collect::<Result<Vec<TemplateEntry>, _>>()
        .unwrap();
    let root_dir = env::current_dir().unwrap();
    for template in templates {
        template.render(root_dir.as_path(), &data)?;
    }
    Ok(())
}
