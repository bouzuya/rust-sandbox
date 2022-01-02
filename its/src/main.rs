use std::str::FromStr;

use crate::{
    entity::IssueTitle,
    workflow::{create_issue_workflow, CreateIssue},
};

mod entity;
mod workflow;

fn main() {
    let command = CreateIssue::new(
        // TODO: use option
        IssueTitle::from_str("title").unwrap(),
    );
    let event = create_issue_workflow(command);
    println!("issue created : {:?}", event);
}
