use domain::{IssueDue, IssueId, IssueTitle};

#[derive(Debug)]
pub enum IssueManagementContextCommand {
    CreateIssue(CreateIssue),
    FinishIssue(FinishIssue),
    UpdateIssue(UpdateIssue),
}

impl From<CreateIssue> for IssueManagementContextCommand {
    fn from(command: CreateIssue) -> Self {
        Self::CreateIssue(command)
    }
}

impl From<FinishIssue> for IssueManagementContextCommand {
    fn from(command: FinishIssue) -> Self {
        Self::FinishIssue(command)
    }
}

impl From<UpdateIssue> for IssueManagementContextCommand {
    fn from(command: UpdateIssue) -> Self {
        Self::UpdateIssue(command)
    }
}

#[derive(Debug)]
pub struct CreateIssue {
    pub issue_title: IssueTitle,
    pub issue_due: Option<IssueDue>,
}

#[derive(Debug)]
pub struct FinishIssue {
    pub issue_id: IssueId,
}

#[derive(Debug)]
pub struct UpdateIssue {
    pub issue_id: IssueId,
    pub issue_due: Option<IssueDue>,
}
