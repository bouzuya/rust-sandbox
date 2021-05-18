use date_range::{self, DateRange, InputFormat, OutputFormat};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "date-range", about = "date range")]
struct Opt {
    #[structopt(long = "format", help = "Specifies the input format")]
    format: Option<InputFormat>,
    #[structopt(long = "first", help = "Prints the first day of the date range")]
    first: bool,
    #[structopt(name = "INPUT")]
    input: String,
    #[structopt(long = "last", help = "Prints the last day of the date range")]
    last: bool,
    #[structopt(long = "week-date", help = "Prints the week date instead of the date")]
    week_date: bool,
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
    let m = output_format.format(opt.week_date, &r);
    println!("{}", m);
    Ok(())
}
