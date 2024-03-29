mod command;
mod command_handler;
mod event;
pub mod issue_block_link_repository;
pub mod issue_comment_repository;
pub mod issue_repository;

pub use self::command::*;
pub use self::event::IssueManagementContextEvent;
pub use self::issue_block_link_repository::*;
use self::issue_comment_repository::HasIssueCommentRepository;
pub use self::issue_comment_repository::IssueCommentRepository;
pub use self::issue_repository::*;
use async_trait::async_trait;
use domain::{IssueBlockLinkId, IssueId};

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IssueAggregate")]
    IssueAggregate(#[from] domain::aggregate::issue::Error),
    #[error("IssueBlockLinkAggregate")]
    IssueBlockLinkAggregate(#[from] domain::aggregate::issue_block_link::Error),
    #[error("IssueBlockLinkNotFound")]
    IssueBlockLinkNotFound(IssueBlockLinkId),
    #[error("IssueBlockLinkRepository")]
    IssueBlockLinkRepository(#[from] IssueBlockLinkRepositoryError),
    #[error("IssueNotFound")]
    IssueNotFound(IssueId),
    #[error("IssueRepository")]
    IssueRepository(#[from] crate::use_case::issue_repository::Error),
    #[error("InvalidIssueBlockLinkId")]
    InvalidIssueBlockLinkId(#[from] domain::issue_block_link_id::Error),
    #[error("block issue {0}")]
    BlockIssue(#[from] command_handler::block_issue::Error),
    #[error("create issue {0}")]
    CreateIssue(#[from] command_handler::create_issue::Error),
    #[error("create issue comment {0}")]
    CreateIssueComment(#[from] command_handler::create_issue_comment::Error),
    #[error("delete issue comment {0}")]
    DeleteIssueComment(#[from] command_handler::delete_issue_comment::Error),
    #[error("finish issue {0}")]
    FinishIssue(#[from] command_handler::finish_issue::Error),
    #[error("unblock issue {0}")]
    UnblockIssue(#[from] command_handler::unblock_issue::Error),
    #[error("update issue {0}")]
    UpdateIssue(#[from] command_handler::update_issue::Error),
    #[error("update issue comment {0}")]
    UpdateIssueComment(#[from] command_handler::update_issue_comment::Error),
    #[error("update issue description {0}")]
    UpdateIssueDescription(#[from] command_handler::update_issue_description::Error),
    #[error("update issue title {0}")]
    UpdateIssueTitle(#[from] command_handler::update_issue_title::Error),
}

#[async_trait]
pub trait IssueManagementContextUseCase:
    HasIssueRepository + HasIssueBlockLinkRepository + HasIssueCommentRepository
{
    async fn handle<C: Into<IssueManagementContextCommand> + Send>(
        &self,
        command: C,
    ) -> Result<IssueManagementContextEvent> {
        use IssueManagementContextCommand::*;
        match command.into() {
            BlockIssue(command) => {
                Ok(command_handler::block_issue::block_issue(self, command).await?)
            }
            CreateIssue(command) => {
                Ok(command_handler::create_issue::create_issue(self, command).await?)
            }
            CreateIssueComment(command) => Ok(
                command_handler::create_issue_comment::create_issue_comment(self, command).await?,
            ),
            DeleteIssueComment(command) => Ok(
                command_handler::delete_issue_comment::delete_issue_comment(self, command).await?,
            ),
            FinishIssue(command) => {
                Ok(command_handler::finish_issue::finish_issue(self, command).await?)
            }
            UnblockIssue(command) => {
                Ok(command_handler::unblock_issue::unblock_issue(self, command).await?)
            }
            UpdateIssue(command) => {
                Ok(command_handler::update_issue::update_issue(self, command).await?)
            }
            UpdateIssueComment(command) => Ok(
                command_handler::update_issue_comment::update_issue_comment(self, command).await?,
            ),
            UpdateIssueTitle(command) => {
                Ok(command_handler::update_issue_title::update_issue_title(self, command).await?)
            }
            UpdateIssueDescription(command) => Ok(
                command_handler::update_issue_description::update_issue_description(self, command)
                    .await?,
            ),
        }
    }
}

impl<T: HasIssueRepository + HasIssueBlockLinkRepository + HasIssueCommentRepository>
    IssueManagementContextUseCase for T
{
}

pub trait HasIssueManagementContextUseCase {
    type IssueManagementContextUseCase: IssueManagementContextUseCase;
    fn issue_management_context_use_case(&self) -> &Self::IssueManagementContextUseCase;
}
