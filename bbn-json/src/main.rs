use std::{fs, path::PathBuf};

fn list_paths() -> anyhow::Result<Vec<PathBuf>> {
    let mut paths = vec![];
    let read_dir =
        fs::read_dir("/Users/bouzuya/ghq/github.com/bouzuya/tmp-kraken-test/bbn-data-20230119")?;
    for root_dir_entry in read_dir {
        let root_dir_entry = root_dir_entry?;
        let year_path = root_dir_entry.path();
        for year_dir_entry in fs::read_dir(year_path.as_path())? {
            let year_dir_entry = year_dir_entry?;
            let month_path = year_dir_entry.path();
            for month_dir_entry in fs::read_dir(month_path.as_path())? {
                let month_dir_entry = month_dir_entry?;
                let date_path = month_dir_entry.path();
                paths.push(date_path);
            }
        }
    }
    paths.sort();
    Ok(paths)
}

fn main() -> anyhow::Result<()> {
    let paths = list_paths()?;
    for path in paths {
        println!("{:?}", path);
    }
    Ok(())
}
