mod bbn_date_range;
mod bbn_hatena_blog;
mod bbn_repository;
mod command;
mod config_repository;
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
    #[structopt(name = "config", about = "Updates the configuration file")]
    Config {
        #[structopt(long = "data-dir", name = "DATA_DIR", help = "the data dir")]
        data_dir: PathBuf,
        #[structopt(
            long = "hatena-blog-data-file",
            name = "HATENA_BLOG_DATA_FILE",
            help = "the hatena-blog data file"
        )]
        hatena_blog_data_file: PathBuf,
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
        Subcommand::Config {
            data_dir,
            hatena_blog_data_file,
        } => command::config(data_dir, hatena_blog_data_file),
        Subcommand::DateRange { month, week_date } => command::date_range(month, week_date),
        Subcommand::List { json, query } => command::list(json, query),
        Subcommand::HatenaBlog { subcommand } => command::hatena_blog(subcommand).await,
        Subcommand::View { date, web } => command::view(date, web),
    }
}
