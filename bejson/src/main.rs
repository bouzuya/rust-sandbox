use bejson::JsonValue;
use fs::File;
use std::{
    error::Error,
    fs,
    io::{self, Read},
    path::{Path, PathBuf},
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    file: PathBuf,
    #[structopt(long = "trim-end")]
    trim_end: bool,
}

fn read_to_string_from_handle(handle: &mut impl Read) -> io::Result<String> {
    let mut content = String::new();
    handle.read_to_string(&mut content)?;
    Ok(content)
}

fn read_file_or_stdin(path: &Path) -> io::Result<String> {
    if path == Path::new("-") {
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        read_to_string_from_handle(&mut handle)
    } else {
        read_to_string_from_handle(&mut File::open(path)?)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    let content = read_file_or_stdin(opt.file.as_path())?;
    let bejson = content.parse::<JsonValue>()?;
    let json = bejson.eval(opt.trim_end)?;
    println!("{}", json);
    Ok(())
}
