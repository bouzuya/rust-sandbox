use anyhow::Context;
use once_cell::sync::Lazy;
use regex::Regex;
use url::Url;

static ASIN_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"\A[0-9A-Z]{10}\z"#).expect("invalid pattern"));

#[derive(clap::Parser)]
struct Args {
    url: String,
}

fn main() -> anyhow::Result<()> {
    let args = <Args as clap::Parser>::parse();
    let url = Url::parse(&args.url).context("invalid url")?;
    let mut path_segments = url.path_segments().context("no path")?;
    let asin = path_segments
        .find(|path_segment| ASIN_PATTERN.is_match(path_segment))
        .context("no asin")?;
    println!("https://www.amazon.co.jp/dp/{asin}");
    Ok(())
}
