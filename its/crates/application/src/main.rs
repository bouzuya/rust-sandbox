mod use_case;

use std::str::FromStr;

use domain::{IssueId, IssueTitle};
use use_case::{issue_management_context_use_case, CreateIssue, IssueManagementContextCommand};

use crate::use_case::FinishIssue;

#[argopt::subcmd(name = "issue-create")]
fn issue_create(#[opt(long = "title")] title: Option<String>) -> anyhow::Result<()> {
    // TODO: unwrap
    let issue_title = IssueTitle::try_from(title.unwrap_or_default()).unwrap();
    let command = IssueManagementContextCommand::CreateIssue(CreateIssue { issue_title });
    let event = issue_management_context_use_case(command)?;
    println!("issue created : {:?}", event);
    Ok(())
}

#[argopt::subcmd(name = "issue-finish")]
fn issue_finish(issue_id: String) -> anyhow::Result<()> {
    let issue_id = IssueId::from_str(issue_id.as_str())?;
    let command = IssueManagementContextCommand::FinishIssue(FinishIssue { issue_id });
    let event = issue_management_context_use_case(command)?;
    println!("issue finished : {:?}", event);
    Ok(())
}

#[argopt::cmd_group(commands = [issue_create, issue_finish])]
fn main() -> anyhow::Result<()> {}
