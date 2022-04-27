//! DomainEvent
//!
//! ドメインイベント (domain event) は……
//!
//! - あるドメインで発生するイベントを表したものである
//! - 集約イベントの集合である
//!   - 集約をまたがる形で発生するイベントもありそうだが割り切っている
//! - 他のコンテキストに提供するデータのソースになる
//!   - use_case crate で不要なデータを落としたものになる想定
//! - 永続化に使用される
//! - domain crate 以外に対して文字列との相互変換を提供する
mod event_dto;
mod event_id;
mod issue_blocked;
mod issue_created;
mod issue_created_v2;
mod issue_finished;
mod issue_unblocked;
mod issue_updated;

use std::fmt::Display;
use std::str::FromStr;

use thiserror::Error;
use ulid::Ulid;

use self::event_dto::*;
use self::event_id::EventId;
use crate::aggregate::IssueAggregateEvent;
use crate::aggregate::IssueBlockLinkAggregateEvent;
use crate::Version;

pub use self::issue_blocked::*;
pub use self::issue_created::*;
pub use self::issue_created_v2::*;
pub use self::issue_finished::*;
pub use self::issue_unblocked::*;
pub use self::issue_updated::*;

trait DomainEventBase {
    fn id(&self) -> EventId;
    fn aggregate_id(&self) -> Ulid;
    fn aggregate_version(&self) -> Version;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DomainEvent {
    Issue(IssueAggregateEvent),
    IssueBlockLink(IssueBlockLinkAggregateEvent),
}

impl Display for DomainEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dto = EventDto::from(self.clone());
        let s = serde_json::to_string(&dto).map_err(|_| std::fmt::Error)?;
        write!(f, "{}", s)
    }
}

#[derive(Debug, Error)]
#[error("ParseDomainEventError")]
pub struct ParseDomainEventError;

impl FromStr for DomainEvent {
    type Err = ParseDomainEventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str::<'_, EventDto>(s)
            .map_err(|_| ParseDomainEventError)
            .and_then(|dto| DomainEvent::try_from(dto).map_err(|_| ParseDomainEventError))
            .map_err(|_| ParseDomainEventError)
    }
}

impl From<IssueAggregateEvent> for DomainEvent {
    fn from(event: IssueAggregateEvent) -> Self {
        Self::Issue(event)
    }
}

impl From<IssueBlockLinkAggregateEvent> for DomainEvent {
    fn from(event: IssueBlockLinkAggregateEvent) -> Self {
        Self::IssueBlockLink(event)
    }
}

impl DomainEvent {
    pub fn issue(self) -> Option<IssueAggregateEvent> {
        if let Self::Issue(event) = self {
            Some(event)
        } else {
            None
        }
    }

    pub fn issue_block_link(self) -> Option<IssueBlockLinkAggregateEvent> {
        if let Self::IssueBlockLink(event) = self {
            Some(event)
        } else {
            None
        }
    }
}
