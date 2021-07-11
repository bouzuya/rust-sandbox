mod diff;
mod download;
mod upload;
mod view;

use self::diff::diff;
use self::download::download_from_hatena_blog;
use self::upload::post_to_hatena_blog;
use self::view::view;
use date_range::date::Date;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum HatenaBlogSubcommand {
    #[structopt(name = "diff", about = "diff")]
    Diff {
        #[structopt(name = "DATE", help = "the entry id")]
        date: Option<String>,
    },
    #[structopt(name = "download", about = "Download to the hatena blog")]
    Download {
        #[structopt(long = "hatena-api-key", env = "HATENA_API_KEY")]
        hatena_api_key: String,
        #[structopt(long = "hatena-blog-id", env = "HATENA_BLOG_ID")]
        hatena_blog_id: String,
        #[structopt(long = "hatena-id", env = "HATENA_ID")]
        hatena_id: String,
    },
    #[structopt(name = "upload", about = "Upload to the hatena blog")]
    Upload {
        #[structopt(name = "DATE", help = "date")]
        date: String,
        #[structopt(long = "draft")]
        draft: bool,
        #[structopt(long = "hatena-api-key", env = "HATENA_API_KEY")]
        hatena_api_key: String,
        #[structopt(long = "hatena-blog-id", env = "HATENA_BLOG_ID")]
        hatena_blog_id: String,
        #[structopt(long = "hatena-id", env = "HATENA_ID")]
        hatena_id: String,
    },
    #[structopt(name = "view", about = "view")]
    View {
        #[structopt(name = "DATE", help = "the entry id")]
        date: Date,
        #[structopt(long = "hatena-blog-id", env = "HATENA_BLOG_ID")]
        hatena_blog_id: String,
        #[structopt(long = "web", help = "Open the entry in the browser")]
        web: bool,
    },
}

pub async fn hatena_blog(subcommand: HatenaBlogSubcommand) -> anyhow::Result<()> {
    match subcommand {
        HatenaBlogSubcommand::Diff { date } => diff(date).await,
        HatenaBlogSubcommand::Download {
            hatena_api_key,
            hatena_blog_id,
            hatena_id,
        } => download_from_hatena_blog(hatena_api_key, hatena_blog_id, hatena_id).await,
        HatenaBlogSubcommand::Upload {
            date,
            draft,
            hatena_api_key,
            hatena_blog_id,
            hatena_id,
        } => post_to_hatena_blog(date, draft, hatena_api_key, hatena_blog_id, hatena_id).await,
        HatenaBlogSubcommand::View {
            date,
            hatena_blog_id,
            web,
        } => view(date, hatena_blog_id, web).await,
    }
}
