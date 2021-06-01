mod bbn_date_range;
mod post;
mod query;

use bbn_date_range::bbn_date_range;
use date_range::date::Date;
use post::list_posts;
use query::Query;
use std::{convert::TryFrom, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, StructOpt)]
enum Subcommand {
    #[structopt(name = "date-range", about = "Prints the date range")]
    DateRange {
        #[structopt(name = "input", help = "input")]
        month: String,
        #[structopt(long = "week-date", help = "Prints the date range as week date")]
        week_date: bool,
    },
    #[structopt(name = "list", about = "Lists the blog posts")]
    List {
        #[structopt(long = "data-dir", help = "the data dir")]
        data_dir: PathBuf,
        #[structopt(long = "json", help = "json")]
        json: bool,
        #[structopt(name = "query", help = "query")]
        query: String,
    },
    #[structopt(name = "view", about = "Views the blog post")]
    View {
        #[structopt(long = "data-dir", help = "the data dir")]
        data_dir: PathBuf,
        #[structopt(name = "date", help = "the date")]
        date: Date,
    },
}

fn main() {
    let opt = Opt::from_args();
    match opt.subcommand {
        Subcommand::DateRange { month, week_date } => bbn_date_range(month, week_date).unwrap(),
        Subcommand::List {
            data_dir,
            json,
            query,
        } => {
            let query = Query::try_from(query.as_str()).unwrap();
            let mut posts = list_posts(data_dir.as_path(), &query).unwrap();
            posts.sort();
            posts.reverse();
            let mut output = vec![];
            for post in posts {
                if json {
                    output.push(format!(
                        r#"{{"date":"{}","title":"{}"}}"#,
                        post.date, post.title
                    ));
                } else {
                    output.push(format!("{} {}", post.date, post.title));
                }
            }
            let output = if json {
                let mut s = String::new();
                s.push('[');
                s.push_str(&output.join(","));
                s.push(']');
                s
            } else {
                output.join("\n")
            };
            println!("{}", output);
        }
        Subcommand::View { data_dir, date } => {
            let query_string = format!("date:{}", date);
            let query = Query::try_from(query_string.as_str()).unwrap();
            let posts = list_posts(data_dir.as_path(), &query).unwrap();
            let post = posts.first().unwrap();
            println!(
                "{} {} https://blog.bouzuya.net/{}/",
                post.date,
                post.title,
                date.to_string().replace('-', "/")
            );
        }
    }
}
