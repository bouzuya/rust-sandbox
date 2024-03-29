pub use super::command_handler::block_issue::BlockIssue;
pub use super::command_handler::create_issue::CreateIssue;
pub use super::command_handler::create_issue_comment::CreateIssueComment;
pub use super::command_handler::delete_issue_comment::DeleteIssueComment;
pub use super::command_handler::finish_issue::FinishIssue;
pub use super::command_handler::unblock_issue::UnblockIssue;
pub use super::command_handler::update_issue::UpdateIssue;
pub use super::command_handler::update_issue_comment::UpdateIssueComment;
pub use super::command_handler::update_issue_description::UpdateIssueDescription;
pub use super::command_handler::update_issue_title::UpdateIssueTitle;

#[derive(Debug, Eq, PartialEq)]
pub enum IssueManagementContextCommand {
    BlockIssue(BlockIssue),
    CreateIssue(CreateIssue),
    CreateIssueComment(CreateIssueComment),
    DeleteIssueComment(DeleteIssueComment),
    FinishIssue(FinishIssue),
    UnblockIssue(UnblockIssue),
    UpdateIssue(UpdateIssue),
    UpdateIssueComment(UpdateIssueComment),
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

impl From<CreateIssueComment> for IssueManagementContextCommand {
    fn from(command: CreateIssueComment) -> Self {
        Self::CreateIssueComment(command)
    }
}

impl From<DeleteIssueComment> for IssueManagementContextCommand {
    fn from(command: DeleteIssueComment) -> Self {
        Self::DeleteIssueComment(command)
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

impl From<UpdateIssueComment> for IssueManagementContextCommand {
    fn from(command: UpdateIssueComment) -> Self {
        Self::UpdateIssueComment(command)
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
