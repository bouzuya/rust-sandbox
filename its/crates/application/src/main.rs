use std::str::FromStr;

use adapter_fs::FsIssueRepository;
use domain::{IssueDue, IssueId, IssueTitle};
use use_case::{
    issue_management_context_use_case, CreateIssue, FinishIssue, IssueManagementContextCommand,
    UpdateIssue,
};

#[argopt::subcmd(name = "issue-create")]
fn issue_create(
    #[opt(long = "title")] title: Option<String>,
    #[opt(long = "due")] due: Option<String>,
) -> anyhow::Result<()> {
    let issue_repository = FsIssueRepository::default();

    let issue_title = IssueTitle::try_from(title.unwrap_or_default())?;
    let issue_due = due.map(|s| IssueDue::from_str(s.as_str())).transpose()?;
    let command = IssueManagementContextCommand::CreateIssue(CreateIssue {
        issue_title,
        issue_due,
    });
    let event = issue_management_context_use_case(issue_repository, command)?;
    println!("issue created : {:?}", event);
    Ok(())
}

#[argopt::subcmd(name = "issue-finish")]
fn issue_finish(issue_id: String) -> anyhow::Result<()> {
    let issue_repository = FsIssueRepository::default();

    let issue_id = IssueId::from_str(issue_id.as_str())?;
    let command = IssueManagementContextCommand::FinishIssue(FinishIssue { issue_id });
    let event = issue_management_context_use_case(issue_repository, command)?;
    println!("issue finished : {:?}", event);
    Ok(())
}

#[argopt::subcmd(name = "issue-update")]
fn issue_update(issue_id: String, #[opt(long = "due")] due: Option<String>) -> anyhow::Result<()> {
    let issue_repository = FsIssueRepository::default();

    let issue_id = IssueId::from_str(issue_id.as_str())?;
    let issue_due = due.map(|s| IssueDue::from_str(s.as_str())).transpose()?;
    let command = IssueManagementContextCommand::UpdateIssue(UpdateIssue {
        issue_id,
        issue_due,
    });
    let event = issue_management_context_use_case(issue_repository, command)?;
    println!("issue updated : {:?}", event);
    Ok(())
}

#[argopt::cmd_group(commands = [issue_create, issue_finish, issue_update])]
fn main() -> anyhow::Result<()> {}
