mod command;
mod command_handler;
mod event;
mod issue_block_link_repository;
mod issue_repository;

pub use self::command::*;
pub use self::event::IssueManagementContextEvent;
pub use self::issue_block_link_repository::*;
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
    IssueRepository(#[from] IssueRepositoryError),
    #[error("InvalidIssueBlockLinkId")]
    InvalidIssueBlockLinkId(#[from] domain::issue_block_link_id::Error),
    #[error("block issue {0}")]
    BlockIssue(#[from] command_handler::block_issue::Error),
    #[error("create issue {0}")]
    CreateIssue(#[from] command_handler::create_issue::Error),
    #[error("finish issue {0}")]
    FinishIssue(#[from] command_handler::finish_issue::Error),
    #[error("unblock issue {0}")]
    UnblockIssue(#[from] command_handler::unblock_issue::Error),
    #[error("update issue {0}")]
    UpdateIssue(#[from] command_handler::update_issue::Error),
    #[error("update issue description {0}")]
    UpdateIssueDescription(#[from] command_handler::update_issue_description::Error),
    #[error("update issue title {0}")]
    UpdateIssueTitle(#[from] command_handler::update_issue_title::Error),
}

#[async_trait]
pub trait IssueManagementContextUseCase: HasIssueRepository + HasIssueBlockLinkRepository {
    async fn handle<C: Into<IssueManagementContextCommand> + Send>(
        &self,
        command: C,
    ) -> Result<Vec<IssueManagementContextEvent>> {
        match command.into() {
            IssueManagementContextCommand::BlockIssue(command) => {
                Ok(command_handler::block_issue::block_issue(self, command).await?)
            }
            IssueManagementContextCommand::CreateIssue(command) => {
                Ok(command_handler::create_issue::create_issue(self, command).await?)
            }
            IssueManagementContextCommand::FinishIssue(command) => {
                Ok(command_handler::finish_issue::finish_issue(self, command).await?)
            }
            IssueManagementContextCommand::UnblockIssue(command) => {
                Ok(command_handler::unblock_issue::unblock_issue(self, command).await?)
            }
            IssueManagementContextCommand::UpdateIssue(command) => {
                Ok(command_handler::update_issue::update_issue(self, command).await?)
            }
            IssueManagementContextCommand::UpdateIssueTitle(command) => {
                Ok(command_handler::update_issue_title::update_issue_title(self, command).await?)
            }
            IssueManagementContextCommand::UpdateIssueDescription(command) => Ok(
                command_handler::update_issue_description::update_issue_description(self, command)
                    .await?,
            ),
        }
    }
}

impl<T: HasIssueRepository + HasIssueBlockLinkRepository> IssueManagementContextUseCase for T {}

pub trait HasIssueManagementContextUseCase {
    type IssueManagementContextUseCase: IssueManagementContextUseCase;
    fn issue_management_context_use_case(&self) -> &Self::IssueManagementContextUseCase;
}
