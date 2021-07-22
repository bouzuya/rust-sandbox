mod bbn_date_range;
mod bbn_repository;
mod command;
mod config_repository;
mod data;
mod hatena_blog;
mod query;

pub use bbn_date_range::bbn_date_range;
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
        subcommand: HatenaBlogSubcommand,
    },
    #[structopt(name = "view", about = "Views the blog post")]
    View {
        #[structopt(long = "content", help = "Prints the contents of the entry")]
        content: bool,
        #[structopt(name = "date", help = "the date")]
        date: Date,
        #[structopt(long = "json", help = "Prints in the JSON format")]
        json: bool,
        #[structopt(long = "meta", help = "Prints the meta data of the entry")]
        meta: bool,
        #[structopt(long = "web", help = "Open the entry in the browser")]
        web: bool,
    },
}

#[derive(Debug, StructOpt)]
pub enum HatenaBlogSubcommand {
    #[structopt(name = "diff", about = "diff")]
    Diff {
        #[structopt(name = "DATE", help = "the entry id")]
        date: Option<String>,
    },
    #[structopt(name = "download", about = "Download to the hatena blog")]
    Download {
        #[structopt(long = "data-file-only")]
        data_file_only: bool,
        #[structopt(name = "DATE")]
        date: Option<Date>,
        #[structopt(long = "hatena-api-key", env = "HATENA_API_KEY")]
        hatena_api_key: String,
        #[structopt(long = "hatena-blog-id", env = "HATENA_BLOG_ID")]
        hatena_blog_id: String,
        #[structopt(long = "hatena-id", env = "HATENA_ID")]
        hatena_id: String,
    },
    #[structopt(name = "list")]
    List,
    #[structopt(name = "upload", about = "Upload to the hatena blog")]
    Upload {
        #[structopt(name = "DATE", help = "date")]
        date: Option<Date>,
        #[structopt(long = "draft")]
        draft: bool,
        #[structopt(long = "hatena-api-key", env = "HATENA_API_KEY")]
        hatena_api_key: String,
        #[structopt(long = "hatena-blog-id", env = "HATENA_BLOG_ID")]
        hatena_blog_id: String,
        #[structopt(long = "hatena-id", env = "HATENA_ID")]
        hatena_id: String,
        #[structopt(long = "interactive")]
        interactive: bool,
    },
    #[structopt(name = "view", about = "view")]
    View {
        #[structopt(long = "content")]
        content: bool,
        #[structopt(name = "DATE", help = "the entry id")]
        date: Date,
        #[structopt(long = "hatena-blog-id", env = "HATENA_BLOG_ID")]
        hatena_blog_id: String,
        #[structopt(long = "meta")]
        meta: bool,
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
        Subcommand::HatenaBlog { subcommand } => match subcommand {
            HatenaBlogSubcommand::Diff { date } => command::hatena_blog::diff(date).await,
            HatenaBlogSubcommand::Download {
                date,
                data_file_only,
                hatena_api_key,
                hatena_blog_id,
                hatena_id,
            } => {
                command::hatena_blog::download(
                    data_file_only,
                    date,
                    hatena_api_key,
                    hatena_blog_id,
                    hatena_id,
                )
                .await
            }
            HatenaBlogSubcommand::List => command::hatena_blog::list().await,
            HatenaBlogSubcommand::Upload {
                date,
                draft,
                hatena_api_key,
                hatena_blog_id,
                hatena_id,
                interactive,
            } => {
                command::hatena_blog::upload(
                    date,
                    draft,
                    hatena_api_key,
                    hatena_blog_id,
                    hatena_id,
                    interactive,
                )
                .await
            }
            HatenaBlogSubcommand::View {
                content,
                date,
                hatena_blog_id,
                meta,
                web,
            } => command::hatena_blog::view(content, date, hatena_blog_id, meta, web).await,
        },
        Subcommand::View {
            content,
            date,
            json,
            meta,
            web,
        } => command::view(date, content, json, meta, web),
    }
}
