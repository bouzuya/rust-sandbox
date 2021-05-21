use date_range::{date::Date, week_date::WeekDate};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "week", about = "Prints the week that contains the date")]
struct Opt {
    #[structopt(name = "DATE", help = "The date")]
    date: Date,
    #[structopt(long = "week-date", help = "Prints in week date format (YYYY-Www-D)")]
    week_date: bool,
    #[structopt(long = "week-year", help = "Prints in week year format (YYYY)")]
    week_year: bool,
    #[structopt(long = "week", help = "Prints in week format (YYYY-Www)")]
    week: bool,
}

fn main() {
    let opt = Opt::from_args();
    let wd = WeekDate::from(opt.date);
    let message = match (opt.week_year, opt.week, opt.week_date) {
        (false, false, false) => wd.year_week().to_string(),
        (true, false, false) => wd.year().to_string(),
        (false, true, false) => wd.year_week().to_string(),
        (false, false, true) => wd.to_string(),
        _ => panic!(),
    };
    println!("{}", message);
}
