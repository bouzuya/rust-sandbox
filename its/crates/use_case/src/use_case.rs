mod command;
mod event;
mod issue_block_link_repository;
mod issue_repository;

pub use self::command::*;
pub use self::event::IssueManagementContextEvent;
pub use self::issue_block_link_repository::*;
pub use self::issue_repository::*;
use async_trait::async_trait;
use domain::IssueBlockLinkId;
use domain::IssueUnblocked;
use domain::{
    aggregate::{
        IssueAggregate, IssueAggregateError, IssueAggregateEvent, IssueBlockLinkAggregateError,
        IssueBlockLinkAggregateEvent,
    },
    DomainEvent, IssueBlocked, IssueCreatedV2, IssueDue, IssueFinished, IssueId, IssueNumber,
    IssueTitle, IssueUpdated,
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
}

#[async_trait]
pub trait IssueManagementContextUseCase: HasIssueRepository + HasIssueBlockLinkRepository {
    async fn handle(
        &self,
        command: IssueManagementContextCommand,
    ) -> Result<IssueManagementContextEvent, IssueManagementContextError> {
        match command {
            IssueManagementContextCommand::BlockIssue(command) => {
                let event = self.handle_block_issue(command).await?;
                Ok(IssueManagementContextEvent::from(
                    DomainEvent::IssueBlockLink(IssueBlockLinkAggregateEvent::from(event)),
                ))
            }
            IssueManagementContextCommand::CreateIssue(command) => {
                let event = self.handle_create_issue(command).await?;
                Ok(IssueManagementContextEvent::from(DomainEvent::Issue(
                    IssueAggregateEvent::from(event),
                )))
            }
            IssueManagementContextCommand::FinishIssue(command) => {
                let event = self.handle_finish_issue(command).await?;
                Ok(IssueManagementContextEvent::from(DomainEvent::Issue(
                    IssueAggregateEvent::from(event),
                )))
            }
            IssueManagementContextCommand::UnblockIssue(command) => {
                let event = self.handle_unblock_issue(command).await?;
                Ok(IssueManagementContextEvent::from(
                    DomainEvent::IssueBlockLink(IssueBlockLinkAggregateEvent::from(event)),
                ))
            }
            IssueManagementContextCommand::UpdateIssue(command) => {
                let event = self.handle_update_issue(command).await?;
                Ok(IssueManagementContextEvent::from(DomainEvent::Issue(
                    IssueAggregateEvent::from(event),
                )))
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
    ) -> Result<IssueBlocked, IssueManagementContextError> {
        // io
        let at = Instant::now();
        // TODO: already created
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
        let issue_block_link = issue.block(blocked_issue, at)?;

        // io
        self.issue_block_link_repository()
            .save(&issue_block_link)
            .await?;

        Ok(match issue_block_link.events().first() {
            Some(event) => match event {
                IssueBlockLinkAggregateEvent::Blocked(event) => event.clone(),
                IssueBlockLinkAggregateEvent::Unblocked(_) => unreachable!(),
            },
            None => unreachable!(),
        })
    }

    async fn handle_create_issue(
        &self,
        command: CreateIssue,
    ) -> Result<IssueCreatedV2, IssueManagementContextError> {
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

        let event = created.events().first().unwrap().clone(); // TODO
        if let IssueAggregateEvent::CreatedV2(event) = event {
            Ok(event)
        } else {
            unreachable!()
        }
    }

    async fn handle_finish_issue(
        &self,
        command: FinishIssue,
    ) -> Result<IssueFinished, IssueManagementContextError> {
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

        let event = updated.events().first().unwrap().clone(); // TODO
        if let IssueAggregateEvent::Finished(event) = event {
            Ok(event)
        } else {
            unreachable!()
        }
    }

    async fn handle_unblock_issue(
        &self,
        UnblockIssue {
            issue_block_link_id,
        }: UnblockIssue,
    ) -> Result<IssueUnblocked, IssueManagementContextError> {
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

        Ok(match updated.events().first() {
            Some(event) => match event {
                IssueBlockLinkAggregateEvent::Blocked(_) => unreachable!(),
                IssueBlockLinkAggregateEvent::Unblocked(event) => event.clone(),
            },
            None => unreachable!(),
        })
    }

    async fn handle_update_issue(
        &self,
        command: UpdateIssue,
    ) -> Result<IssueUpdated, IssueManagementContextError> {
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

        let event = updated
            .events()
            .iter()
            .find(|event| matches!(event, IssueAggregateEvent::Updated(_)))
            .unwrap()
            .clone(); // TODO
        if let IssueAggregateEvent::Updated(event) = event {
            Ok(event)
        } else {
            unreachable!()
        }
    }
}

impl<T: HasIssueRepository + HasIssueBlockLinkRepository> IssueManagementContextUseCase for T {}

pub trait HasIssueManagementContextUseCase {
    type IssueManagementContextUseCase: IssueManagementContextUseCase;
    fn issue_management_context_use_case(&self) -> &Self::IssueManagementContextUseCase;
}
