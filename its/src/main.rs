use crate::workflow::{create_issue_workflow, CreateIssue};

mod entity;
mod workflow;

fn main() {
    let command = CreateIssue {};
    let event = create_issue_workflow(command);
    println!("issue created : {:?}", event);
}
