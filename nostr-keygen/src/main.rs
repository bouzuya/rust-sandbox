use nostr::{prelude::ToBech32, Keys};

#[derive(Debug, clap::Parser)]
struct Arg {
    prefix: String,
}

fn main() -> anyhow::Result<()> {
    let arg = <Arg as clap::Parser>::parse();
    for c in "1bio".chars() {
        if arg.prefix.contains(c) {
            println!("prefix {} contains invalid char '{}'", arg.prefix, c);
            return Ok(());
        }
    }
    let prefix = format!("npub1{}", arg.prefix);
    for count in 1_usize.. {
        let keys = Keys::generate();
        let public_key = keys.public_key().to_bech32()?;
        if public_key.starts_with(prefix.as_str()) {
            println!("public_key  = {}", public_key);
            println!("private_key = {}", keys.secret_key()?.to_bech32()?);
            break;
        }
        if count % 10000 == 0 {
            println!("{count}");
        }
    }
    Ok(())
}
