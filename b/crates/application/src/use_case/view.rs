use crate::brepository::BRepository;
use crate::{BId, TimeZoneOffset};
use anyhow::Context;
use std::{fs, io, path::PathBuf};

pub fn view(data_dir: PathBuf, id: BId, writer: &mut impl io::Write) -> anyhow::Result<()> {
    let time_zone_offset = TimeZoneOffset::default(); // TODO
    let repository = BRepository::new(data_dir, time_zone_offset);
    let meta = repository.find_meta(id)?;
    let meta = meta.with_context(|| "b not found")?;
    let content = fs::read_to_string(repository.to_content_path_buf(&meta.id).as_path())?;
    write!(writer, "{}", content)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, str::FromStr};
    use tempfile::tempdir;

    #[test]
    fn test() {
        let dir = tempdir().unwrap();
        let dir20210203 = dir.path().join("flow").join("2021").join("02").join("03");
        fs::create_dir_all(dir20210203.as_path()).unwrap();
        let meta = dir20210203.join("20210203T000000Z.json");
        fs::write(meta.as_path(), "{}").unwrap();
        let content = meta.with_extension("md");
        fs::write(content.as_path(), "Hello, world!").unwrap();
        let bid = BId::from_str("20210203T000000Z").unwrap();
        let mut output = vec![];
        view(dir.path().to_path_buf(), bid, &mut output).unwrap();
        assert_eq!(output, b"Hello, world!");
    }
}
