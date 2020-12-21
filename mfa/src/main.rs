mod http_client;

use anyhow::Result;
use http_client::HttpClient;

fn main() -> Result<()> {
    let client = HttpClient::new()?;
    let response = client.get("http://bouzuya.net")?;
    println!("{:?}", response);

    Ok(())
}
