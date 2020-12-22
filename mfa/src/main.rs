mod cli;
mod http_client;

use anyhow::Result;
use cli::run;

fn main() -> Result<()> {
    run()
}
