mod bbn_date_range;
mod bbn_hatena_blog;
mod bbn_repository;
mod command;
mod entry_id;
mod entry_meta;
mod query;
mod timestamp;

use bbn_date_range::bbn_date_range;
use date_range::date::Date;
use std::{io, path::PathBuf};
use structopt::{clap::Shell, StructOpt};

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, StructOpt)]
enum Subcommand {
    #[structopt(name = "completion", about = "Prints the shell's completion script")]
    Completion {
        #[structopt(name = "SHELL", help = "the shell", possible_values = &Shell::variants())]
        shell: Shell,
    },
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
    #[structopt(name = "hatena-blog", about = "hatena-blog")]
    HatenaBlog {
        #[structopt(subcommand)]
        subcommand: command::HatenaBlogSubcommand,
    },
    #[structopt(name = "view", about = "Views the blog post")]
    View {
        #[structopt(long = "data-dir", help = "the data dir")]
        data_dir: PathBuf,
        #[structopt(name = "date", help = "the date")]
        date: Date,
        #[structopt(long = "web", help = "Open the entry in the browser")]
        web: bool,
    },
}

fn completion(shell: Shell) -> anyhow::Result<()> {
    Opt::clap().gen_completions_to("bbn", shell, &mut io::stdout());
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();
    match opt.subcommand {
        Subcommand::Completion { shell } => completion(shell),
        Subcommand::DateRange { month, week_date } => command::date_range(month, week_date),
        Subcommand::List {
            data_dir,
            json,
            query,
        } => command::list(data_dir, json, query),
        Subcommand::HatenaBlog { subcommand } => command::hatena_blog(subcommand).await,
        Subcommand::View {
            data_dir,
            date,
            web,
        } => command::view(data_dir, date, web),
    }
}
