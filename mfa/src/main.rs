mod cli;
mod http_client;

use anyhow::Result;
use cli::run;
use http_client::{HttpClient, HttpMethod};

fn main() -> Result<()> {
    let client = HttpClient::new()?;
    let request = client.request(HttpMethod::GET, "http://bouzuya.net")?;
    println!("{:?}", request);
    let response = client.execute(request)?;
    println!("{:?}", response);

    run()
}
