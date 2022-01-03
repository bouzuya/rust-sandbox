use crate::{
    entity::IssueTitle,
    workflow::{create_issue_workflow, CreateIssue},
};

mod entity;
mod workflow;

#[argopt::subcmd(name = "issue-create")]
fn issue_create(#[opt(long = "title")] title: Option<String>) {
    let title = IssueTitle::try_from(title.unwrap_or_default()).unwrap();
    let command = CreateIssue::new(title);
    let event = create_issue_workflow(command);
    println!("issue created : {:?}", event);
}

#[argopt::cmd_group(commands = [issue_create])]
fn main() {}
