use std::fmt::Debug;

use async_trait::async_trait;
use domain::{aggregate::issue_comment::IssueCommentAggregate, IssueCommentId, Version};
use thiserror::Error;

#[derive(Debug, Eq, PartialEq, Error)]
pub enum Error {
    #[error("unknown: {0}")]
    Unknown(String),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[async_trait]
pub trait IssueCommentRepository {
    async fn find_by_id(
        &self,
        issue_comment_id: &IssueCommentId,
    ) -> Result<Option<IssueCommentAggregate>>;

    async fn find_by_id_and_version(
        &self,
        issue_comment_id: &IssueCommentId,
        version: &Version,
    ) -> Result<Option<IssueCommentAggregate>>;

    async fn save(&self, issue_comment: &IssueCommentAggregate) -> Result<()>;
}

pub trait HasIssueCommentRepository {
    type IssueCommentRepository: IssueCommentRepository + Send + Sync;

    fn issue_comment_repository(&self) -> &Self::IssueCommentRepository;
}
