use date_range::{self, DateRange, InputFormat, OutputFormat};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "date-range", about = "date range")]
struct Opt {
    #[structopt(long = "format")]
    format: Option<InputFormat>,
    #[structopt(long = "first")]
    first: bool,
    #[structopt(name = "input")]
    input: String,
    #[structopt(long = "last")]
    last: bool,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();
    let input_format = match opt.format {
        None => InputFormat::detect(&opt.input).unwrap(), // TODO: message
        Some(format) => format,
    };
    let output_format = match (opt.first, opt.last) {
        (false, false) => OutputFormat::Range,
        (false, true) => OutputFormat::Last,
        (true, false) => OutputFormat::First,
        (true, true) => OutputFormat::Range,
    };
    let r = DateRange::parse(&input_format, &opt.input).unwrap();
    let m = output_format.format(&r);
    println!("{}", m);
    Ok(())
}
