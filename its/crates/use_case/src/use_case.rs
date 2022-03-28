mod command;
mod event;
mod issue_block_link_repository;
mod issue_repository;

pub use self::command::*;
pub use self::event::IssueManagementContextEvent;
pub use self::issue_block_link_repository::*;
pub use self::issue_repository::*;
use async_trait::async_trait;
use domain::{
    aggregate::{IssueAggregate, IssueAggregateError, IssueBlockLinkAggregateError},
    DomainEvent, IssueBlockLinkId, IssueDue, IssueId, IssueNumber, IssueTitle,
    ParseIssueBlockLinkError,
};
use limited_date_time::Instant;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IssueManagementContextError {
    #[error("IssueAggregate")]
    IssueAggregate(#[from] IssueAggregateError),
    #[error("IssueBlockLinkAggregate")]
    IssueBlockLinkAggregate(#[from] IssueBlockLinkAggregateError),
    #[error("IssueBlockLinkNotFound")]
    IssueBlockLinkNotFound(IssueBlockLinkId),
    #[error("IssueBlockLinkRepository")]
    IssueBlockLinkRepository(#[from] IssueBlockLinkRepositoryError),
    #[error("IssueNotFound")]
    IssueNotFound(IssueId),
    #[error("IssueRepository")]
    IssueRepository(#[from] IssueRepositoryError),
    #[error("InvalidIssueBlockLinkId")]
    InvalidIssueBlockLinkId(#[from] ParseIssueBlockLinkError),
}

#[async_trait]
pub trait IssueManagementContextUseCase: HasIssueRepository + HasIssueBlockLinkRepository {
    async fn handle(
        &self,
        command: IssueManagementContextCommand,
    ) -> Result<Vec<IssueManagementContextEvent>, IssueManagementContextError> {
        match command {
            IssueManagementContextCommand::BlockIssue(command) => {
                self.handle_block_issue(command).await
            }
            IssueManagementContextCommand::CreateIssue(command) => {
                self.handle_create_issue(command).await
            }
            IssueManagementContextCommand::FinishIssue(command) => {
                self.handle_finish_issue(command).await
            }
            IssueManagementContextCommand::UnblockIssue(command) => {
                self.handle_unblock_issue(command).await
            }
            IssueManagementContextCommand::UpdateIssue(command) => {
                self.handle_update_issue(command).await
            }
        }
    }

    fn block_issue(&self, issue_id: IssueId, blocked_issue_id: IssueId) -> BlockIssue {
        BlockIssue {
            issue_id,
            blocked_issue_id,
        }
    }

    fn create_issue(&self, issue_title: IssueTitle, issue_due: Option<IssueDue>) -> CreateIssue {
        CreateIssue {
            issue_title,
            issue_due,
        }
    }

    fn finish_issue(&self, issue_id: IssueId) -> FinishIssue {
        FinishIssue { issue_id }
    }

    fn unblock_issue(&self, issue_block_link_id: IssueBlockLinkId) -> UnblockIssue {
        UnblockIssue {
            issue_block_link_id,
        }
    }

    fn update_issue(&self, issue_id: IssueId, issue_due: Option<IssueDue>) -> UpdateIssue {
        UpdateIssue {
            issue_id,
            issue_due,
        }
    }

    async fn handle_block_issue(
        &self,
        BlockIssue {
            issue_id,
            blocked_issue_id,
        }: BlockIssue,
    ) -> Result<Vec<IssueManagementContextEvent>, IssueManagementContextError> {
        // io
        let at = Instant::now();
        let issue_block_link_id =
            IssueBlockLinkId::new(issue_id.clone(), blocked_issue_id.clone())?;
        let issue_block_link = match self
            .issue_block_link_repository()
            .find_by_id(&issue_block_link_id)
            .await?
        {
            Some(_) => todo!("already created"),
            None => {
                // io
                let issue = self
                    .issue_repository()
                    .find_by_id(&issue_id)
                    .await?
                    .ok_or(IssueManagementContextError::IssueNotFound(issue_id))?;
                let blocked_issue = self
                    .issue_repository()
                    .find_by_id(&blocked_issue_id)
                    .await?
                    .ok_or(IssueManagementContextError::IssueNotFound(blocked_issue_id))?;

                // pure
                issue.block(blocked_issue, at)?
            }
        };

        // io
        self.issue_block_link_repository()
            .save(&issue_block_link)
            .await?;

        Ok(issue_block_link
            .events()
            .iter()
            .cloned()
            .map(DomainEvent::from)
            .map(IssueManagementContextEvent::from)
            .collect::<Vec<IssueManagementContextEvent>>())
    }

    async fn handle_create_issue(
        &self,
        command: CreateIssue,
    ) -> Result<Vec<IssueManagementContextEvent>, IssueManagementContextError> {
        // io
        let issue_number = self
            .issue_repository()
            .last_created()
            .await?
            .map(|issue| issue.id().issue_number().next_number())
            .unwrap_or_else(IssueNumber::start_number);
        let at = Instant::now();

        // pure
        let created =
            IssueAggregate::new(at, issue_number, command.issue_title, command.issue_due)?;

        // io
        self.issue_repository().save(&created).await?;

        Ok(created
            .events()
            .iter()
            .cloned()
            .map(DomainEvent::from)
            .map(IssueManagementContextEvent::from)
            .collect::<Vec<IssueManagementContextEvent>>())
    }

    async fn handle_finish_issue(
        &self,
        command: FinishIssue,
    ) -> Result<Vec<IssueManagementContextEvent>, IssueManagementContextError> {
        // io
        let issue = self
            .issue_repository()
            .find_by_id(&command.issue_id)
            .await?
            .ok_or(IssueManagementContextError::IssueNotFound(command.issue_id))?;
        let at = Instant::now();

        // pure
        let updated = issue.finish(at)?;

        // io
        self.issue_repository().save(&updated).await?;

        Ok(updated
            .events()
            .iter()
            .cloned()
            .map(DomainEvent::from)
            .map(IssueManagementContextEvent::from)
            .collect::<Vec<IssueManagementContextEvent>>())
    }

    async fn handle_unblock_issue(
        &self,
        UnblockIssue {
            issue_block_link_id,
        }: UnblockIssue,
    ) -> Result<Vec<IssueManagementContextEvent>, IssueManagementContextError> {
        // io
        let at = Instant::now();
        let issue_block_link = self
            .issue_block_link_repository()
            .find_by_id(&issue_block_link_id)
            .await?
            .ok_or(IssueManagementContextError::IssueBlockLinkNotFound(
                issue_block_link_id,
            ))?;

        // pure
        let updated = issue_block_link.unblock(at)?;

        // io
        self.issue_block_link_repository().save(&updated).await?;

        Ok(updated
            .events()
            .iter()
            .cloned()
            .map(DomainEvent::from)
            .map(IssueManagementContextEvent::from)
            .collect::<Vec<IssueManagementContextEvent>>())
    }

    async fn handle_update_issue(
        &self,
        command: UpdateIssue,
    ) -> Result<Vec<IssueManagementContextEvent>, IssueManagementContextError> {
        // io
        let issue = self
            .issue_repository()
            .find_by_id(&command.issue_id)
            .await?
            .ok_or(IssueManagementContextError::IssueNotFound(command.issue_id))?;
        let issue_due = command.issue_due;
        let at = Instant::now();

        // pure
        let updated = issue.update(issue_due, at)?;

        // io
        self.issue_repository().save(&updated).await?;

        Ok(updated
            .events()
            .iter()
            .cloned()
            .map(DomainEvent::from)
            .map(IssueManagementContextEvent::from)
            .collect::<Vec<IssueManagementContextEvent>>())
    }
}

impl<T: HasIssueRepository + HasIssueBlockLinkRepository> IssueManagementContextUseCase for T {}

pub trait HasIssueManagementContextUseCase {
    type IssueManagementContextUseCase: IssueManagementContextUseCase;
    fn issue_management_context_use_case(&self) -> &Self::IssueManagementContextUseCase;
}
