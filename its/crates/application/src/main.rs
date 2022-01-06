use use_case::{issue_management_context_use_case, IssueManagementContextCommand};

use crate::use_case::{create_issue_use_case, CreateIssue};
use entity::IssueTitle;

mod use_case;

#[argopt::subcmd(name = "issue-create")]
fn issue_create(#[opt(long = "title")] title: Option<String>) -> anyhow::Result<()> {
    let issue_title = IssueTitle::try_from(title.unwrap_or_default()).unwrap();
    let command = IssueManagementContextCommand::CreateIssue(CreateIssue { issue_title });
    let event = issue_management_context_use_case(command)?;
    println!("issue created : {:?}", event);
    Ok(())
}

#[argopt::cmd_group(commands = [issue_create])]
fn main() -> anyhow::Result<()> {}
