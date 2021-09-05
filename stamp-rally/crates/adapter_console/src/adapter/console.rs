use rustyline::error::ReadlineError;
use rustyline::Editor;
use use_case::{
    CreateStampRallyUseCase, CreateUserUseCase, HasCreateStampRallyUseCase, HasCreateUserUseCase,
};

enum Command {
    ShowHelp,
    CreateStampRally,
    CreateUser,
    Unknown(String),
}

impl Command {
    fn execute<A>(&self, application: &A) -> anyhow::Result<()>
    where
        A: HasCreateStampRallyUseCase + HasCreateUserUseCase,
    {
        match self {
            Command::ShowHelp => show_help(),
            Command::CreateStampRally => create_stamp_rally(application),
            Command::CreateUser => create_user(application),
            Command::Unknown(ref line) => {
                println!("{} is unknown command", line);
                Ok(())
            }
        }
    }
}

impl From<String> for Command {
    fn from(s: String) -> Self {
        if s.is_empty() || s == "help" {
            Command::ShowHelp
        } else if s == "create stamp-rally" {
            Command::CreateStampRally
        } else if s == "create user" {
            Command::CreateUser
        } else {
            Command::Unknown(s)
        }
    }
}

fn show_help() -> anyhow::Result<()> {
    println!("Commands:");
    println!("  create stamp-rally");
    println!("  create user");
    Ok(())
}

fn create_stamp_rally<A>(application: &A) -> anyhow::Result<()>
where
    A: HasCreateStampRallyUseCase,
{
    let stamp_rally_id = application.create_stamp_rally_use_case().handle()?;
    println!("StampRally created (ID: {})", stamp_rally_id);
    Ok(())
}

fn create_user<A>(application: &A) -> anyhow::Result<()>
where
    A: HasCreateUserUseCase,
{
    let use_case = application.create_user_use_case();
    let user_id = use_case.handle()?;
    println!("User created (ID: {})", user_id);
    Ok(())
}

pub fn run<A>(application: A) -> anyhow::Result<()>
where
    A: HasCreateStampRallyUseCase + HasCreateUserUseCase,
{
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let command = Command::from(line);
                command.execute(&application)?;
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
