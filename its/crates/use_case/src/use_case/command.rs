use domain::{IssueBlockLinkId, IssueDue, IssueId, IssueResolution, IssueTitle};

#[derive(Debug)]
pub enum IssueManagementContextCommand {
    BlockIssue(BlockIssue),
    CreateIssue(CreateIssue),
    FinishIssue(FinishIssue),
    UnblockIssue(UnblockIssue),
    UpdateIssue(UpdateIssue),
    UpdateIssueTitle(UpdateIssueTitle),
}

impl From<BlockIssue> for IssueManagementContextCommand {
    fn from(command: BlockIssue) -> Self {
        Self::BlockIssue(command)
    }
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

impl From<UnblockIssue> for IssueManagementContextCommand {
    fn from(command: UnblockIssue) -> Self {
        Self::UnblockIssue(command)
    }
}

impl From<UpdateIssue> for IssueManagementContextCommand {
    fn from(command: UpdateIssue) -> Self {
        Self::UpdateIssue(command)
    }
}

#[derive(Debug)]
pub struct BlockIssue {
    pub issue_id: IssueId,
    pub blocked_issue_id: IssueId,
}

#[derive(Debug)]
pub struct CreateIssue {
    pub issue_title: IssueTitle,
    pub issue_due: Option<IssueDue>,
}

#[derive(Debug)]
pub struct FinishIssue {
    pub issue_id: IssueId,
    pub resolution: Option<IssueResolution>,
}

#[derive(Debug)]
pub struct UnblockIssue {
    pub issue_block_link_id: IssueBlockLinkId,
}

#[derive(Debug)]
pub struct UpdateIssue {
    pub issue_id: IssueId,
    pub issue_due: Option<IssueDue>,
}

#[derive(Debug)]
pub struct UpdateIssueTitle {
    pub issue_id: IssueId,
    pub issue_title: IssueTitle,
}
