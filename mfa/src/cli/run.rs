use super::{clock_in, list, log_in, log_out};
use anyhow::Result;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Mfa {
    #[structopt(subcommand)]
    sub_command: SubCommand,
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
    match mfa.sub_command {
        SubCommand::ClockIn => clock_in(),
        SubCommand::ClockOut => Ok(()),
        SubCommand::List => list(),
        SubCommand::LogIn => log_in(),
        SubCommand::LogOut => log_out(),
    }
}
