use std::{fs, io, path::Path};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "b-ls", about = "bouzuya's ls: list directory contents")]
struct Opt {
    #[structopt(
        short = "R",
        long = "recursive",
        help = "List subdirectories recursively"
    )]
    recursive: bool,
}

fn read_dir(dir: &Path) -> io::Result<Vec<fs::DirEntry>> {
    let mut entries = vec![];
    for entry in dir.read_dir()? {
        entries.push(entry?);
    }
    Ok(entries)
}

fn list(opt: &Opt, dir: &Path, file_names: &mut Vec<String>) {
    let entries = read_dir(dir).expect("read_dir failed");

    for entry in entries {
        let file_name = entry
            .path()
            .strip_prefix(".")
            .expect("strip_prefix failed")
            .to_str()
            .expect("file_name is not string")
            .to_string();
        let file_type = entry.file_type().expect("file_type failed");
        let is_dir = file_type.is_dir();
        file_names.push(format!("{}{}", file_name, if is_dir { "/" } else { "" }));
        if is_dir && opt.recursive {
            list(opt, entry.path().as_path(), file_names);
        }
    }
}

fn main() -> io::Result<()> {
    let opt = Opt::from_args();

    let mut file_names = vec![];

    list(&opt, &Path::new("."), &mut file_names);

    file_names.sort();

    for file_name in file_names {
        println!("{}", file_name);
    }
    Ok(())
}
