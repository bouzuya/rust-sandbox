use std::str::FromStr;

use entity::{PlayerId, StampRallyId, UserId};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use use_case::{
    CreateStampRallyUseCase, CreateUserUseCase, HasCreateStampRallyUseCase, HasCreateUserUseCase,
    HasIssueStampCardUseCase, HasJoinStampRallyUseCase, IssueStampCardUseCase,
    JoinStampRallyUseCase,
};

enum Command {
    CreateStampRally,
    CreateUser,
    IssueStampCard(String, String),
    JoinStampRally(String, String),
    ShowHelp,
    Unknown(String),
}

impl Command {
    fn execute<A>(&self, application: &A) -> anyhow::Result<()>
    where
        A: HasCreateStampRallyUseCase
            + HasCreateUserUseCase
            + HasIssueStampCardUseCase
            + HasJoinStampRallyUseCase,
    {
        match self {
            Command::CreateStampRally => create_stamp_rally(application),
            Command::CreateUser => create_user(application),
            Command::IssueStampCard(stamp_rally_id, player_id) => {
                issue_stamp_card(application, stamp_rally_id, player_id)
            }
            Command::JoinStampRally(stamp_rally_id, user_id) => {
                join_stamp_rally(application, stamp_rally_id, user_id)
            }
            Command::ShowHelp => show_help(),
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
        } else if s.starts_with("issue stamp-card ") {
            let ids = s
                .trim_start_matches("issue stamp-card ")
                .split(' ')
                .collect::<Vec<&str>>();
            if ids.len() != 2 {
                Command::Unknown(s)
            } else {
                let stamp_rally_id = ids[0];
                let player_id = ids[1];
                Command::IssueStampCard(stamp_rally_id.to_string(), player_id.to_string())
            }
        } else if s.starts_with("join stamp-rally ") {
            let ids = s
                .trim_start_matches("join stamp-rally ")
                .split(' ')
                .collect::<Vec<&str>>();
            if ids.len() != 2 {
                Command::Unknown(s)
            } else {
                let stamp_rally_id = ids[0];
                let user_id = ids[1];
                Command::JoinStampRally(stamp_rally_id.to_string(), user_id.to_string())
            }
        } else {
            Command::Unknown(s)
        }
    }
}

fn show_help() -> anyhow::Result<()> {
    println!("Commands:");
    println!("  create stamp-rally");
    println!("  create user");
    println!("  issue stamp-card <stamp_rally_id> <player_id>");
    println!("  join stamp-rally <stamp_rally_id> <user_id>");
    Ok(())
}

fn create_stamp_rally<A>(application: &A) -> anyhow::Result<()>
where
    A: HasCreateStampRallyUseCase,
{
    let use_case = application.create_stamp_rally_use_case();
    let stamp_rally_id = CreateStampRallyUseCase::handle(use_case)?;
    println!("StampRally created (ID: {})", stamp_rally_id);
    Ok(())
}

fn create_user<A>(application: &A) -> anyhow::Result<()>
where
    A: HasCreateUserUseCase,
{
    let use_case = application.create_user_use_case();
    let user_id = CreateUserUseCase::handle(use_case)?;
    println!("User created (ID: {})", user_id);
    Ok(())
}

fn issue_stamp_card<A>(application: &A, stamp_rally_id: &str, player_id: &str) -> anyhow::Result<()>
where
    A: HasIssueStampCardUseCase,
{
    let use_case = application.issue_stamp_card_use_case();
    let stamp_rally_id = StampRallyId::from_str(stamp_rally_id)?;
    let player_id = PlayerId::from_str(player_id)?;
    let stamp_card_id = IssueStampCardUseCase::handle(use_case, stamp_rally_id, player_id)?;
    println!(
        "StampCard created (ID: {}, StampRally ID: {}, Player ID: {})",
        stamp_card_id, stamp_rally_id, player_id
    );
    Ok(())
}

fn join_stamp_rally<A>(application: &A, stamp_rally_id: &str, user_id: &str) -> anyhow::Result<()>
where
    A: HasJoinStampRallyUseCase,
{
    let use_case = application.join_stamp_rally_use_case();
    let stamp_rally_id = StampRallyId::from_str(stamp_rally_id)?;
    let user_id = UserId::from_str(user_id)?;
    let player_id = JoinStampRallyUseCase::handle(use_case, stamp_rally_id, user_id)?;
    println!(
        "Player created (ID: {}, StampRally ID: {}, User ID: {})",
        player_id, stamp_rally_id, user_id
    );
    Ok(())
}

pub fn run<A>(application: A) -> anyhow::Result<()>
where
    A: HasCreateStampRallyUseCase
        + HasCreateUserUseCase
        + HasIssueStampCardUseCase
        + HasJoinStampRallyUseCase,
{
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let command = Command::from(line);
                match command.execute(&application) {
                    Ok(_) => {}
                    Err(err) => {
                        println!("Error: {:?}", err);
                    }
                }
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
