mod post;
mod query;

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
    #[structopt(name = "list", about = "Lists the blog posts")]
    List {
        #[structopt(long = "data-dir", help = "the data dir")]
        data_dir: PathBuf,
        #[structopt(name = "query", help = "query")]
        query: String,
    },
}

fn main() {
    let opt = Opt::from_args();
    match opt.subcommand {
        Subcommand::List { data_dir, query } => {
            let query = Query::try_from(query.as_str()).unwrap();
            let mut posts = list_posts(data_dir.as_path(), &query).unwrap();
            posts.sort();
            posts.reverse();
            for post in posts {
                println!("{} {}", post.date, post.title);
            }
        }
    }
}
