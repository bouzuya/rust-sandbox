use std::{fs, io};

fn read_dir(path: &str) -> io::Result<Vec<fs::DirEntry>> {
    let mut entries = vec![];
    for entry in fs::read_dir(path)? {
        entries.push(entry?);
    }
    Ok(entries)
}

fn main() -> io::Result<()> {
    let entries = read_dir(".").expect("read_dir failed");

    let mut file_names = vec![];
    for entry in entries {
        let file_name = entry
            .file_name()
            .into_string()
            .expect("file_name is not string");
        file_names.push(file_name);
    }

    file_names.sort();

    for file_name in file_names {
        println!("{}", file_name);
    }
    Ok(())
}
