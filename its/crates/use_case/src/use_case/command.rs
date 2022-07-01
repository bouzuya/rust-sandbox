pub use super::command_handler::block_issue::BlockIssue;
pub use super::command_handler::create_issue::CreateIssue;
pub use super::command_handler::finish_issue::FinishIssue;
pub use super::command_handler::unblock_issue::UnblockIssue;
pub use super::command_handler::update_issue::UpdateIssue;
pub use super::command_handler::update_issue_description::UpdateIssueDescription;
pub use super::command_handler::update_issue_title::UpdateIssueTitle;

#[derive(Debug)]
pub enum IssueManagementContextCommand {
    BlockIssue(BlockIssue),
    CreateIssue(CreateIssue),
    FinishIssue(FinishIssue),
    UnblockIssue(UnblockIssue),
    UpdateIssue(UpdateIssue),
    UpdateIssueDescription(UpdateIssueDescription),
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

impl From<UpdateIssueTitle> for IssueManagementContextCommand {
    fn from(command: UpdateIssueTitle) -> Self {
        Self::UpdateIssueTitle(command)
    }
}

impl From<UpdateIssueDescription> for IssueManagementContextCommand {
    fn from(command: UpdateIssueDescription) -> Self {
        Self::UpdateIssueDescription(command)
    }
}
