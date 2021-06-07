use std::path::PathBuf;

pub fn list(data_dir: PathBuf, query: String) {
    let ymd = query.split('-').collect::<Vec<&str>>();
    let (y, m, d) = match ymd[..] {
        [y, m, d] => (y, m, d),
        _ => return,
    };

    let dir = data_dir.join("flow").join(y).join(m).join(d);
    for dir_entry in dir.read_dir().unwrap() {
        let dir_entry = dir_entry.unwrap();
        let path = dir_entry.path();
        if path.extension().unwrap().to_str().unwrap() == "md" {
            println!("{}", dir.join(path).as_path().to_str().unwrap());
        }
    }
}
