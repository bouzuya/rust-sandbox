use b_ls::list;
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

fn main() {
    let opt = Opt::from_args();

    let mut file_names = list(opt.recursive);

    file_names.sort();

    for file_name in file_names {
        println!("{}", file_name);
    }
}
