#[derive(Debug, clap::Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, clap::Subcommand)]
enum Subcommand {
    CreateUserRequest,
    SendUserRequest,
    UpdateQueryUser,
    UpdateUser,
}

fn main() {
    use Subcommand::*;

    let args = <Args as clap::Parser>::parse();
    match args.subcommand {
        CreateUserRequest => todo!(),
        SendUserRequest => todo!(),
        UpdateQueryUser => todo!(),
        UpdateUser => todo!(),
    }
}
