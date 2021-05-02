use std::{fs, io, path::Path};

fn read_dir(dir: &Path) -> io::Result<Vec<fs::DirEntry>> {
    let mut entries = vec![];
    for entry in dir.read_dir()? {
        entries.push(entry?);
    }
    Ok(entries)
}

pub fn list(recursive: bool) -> Vec<String> {
    let mut file_names = vec![];
    list2(&mut file_names, &Path::new("."), recursive);
    file_names
}

fn list2(file_names: &mut Vec<String>, dir: &Path, recursive: bool) {
    let entries = read_dir(dir).expect("read_dir failed");
    for entry in entries {
        let path_buf = entry.path();
        let file_name = path_buf
            .strip_prefix(".")
            .expect("strip_prefix failed")
            .to_str()
            .expect("file_name is not string")
            .to_string();
        let file_type = entry.file_type().expect("file_type failed");
        let is_dir = file_type.is_dir();
        file_names.push(format!("{}{}", file_name, if is_dir { "/" } else { "" }));
        if is_dir && recursive {
            list2(file_names, path_buf.as_path(), recursive);
        }
    }
}
