use anyhow::Result;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Mfa {
    #[structopt(subcommand)]
    sub_command: SubCommand
}

#[derive(Debug, StructOpt)]
enum SubCommand {
    ClockIn,
    ClockOut,
    List,
    LogIn,
    LogOut,
}

pub fn run() -> Result<()> {
    let mfa = Mfa::from_args();
    println!("{:?}", mfa);
    Ok(())
}
