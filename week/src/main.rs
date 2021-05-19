use date_range::{date::Date, week_date::WeekDate};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "week", about = "Prints the week that contains the date")]
struct Opt {
    #[structopt(name = "DATE", help = "The date")]
    date: Date,
}

fn main() {
    let opt = Opt::from_args();
    println!("{}", WeekDate::from(opt.date).year_week());
}
