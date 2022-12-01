use async_trait::async_trait;

use crate::user_store::{HasUserStore, UserStore};

use ::worker_helper::{
    worker_helper::{self, WorkerDeps},
    worker_repository::WorkerName,
};

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    // #[error("user_aggregate {0}")]
    // UserAggregate(#[from] domain::aggregate::user::Error),
    // #[error("user_repository {0}")]
    // UserRepository(#[from] crate::user_repository::Error),
    // #[error("unknown {0}")]
    // Unknown(String),
    #[error("user_store {0}")]
    UserStore(#[from] crate::user_store::Error),
    #[error("worker_helper {0}")]
    WorkerHelper(#[from] worker_helper::Error),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub struct Command;

pub trait Context: WorkerDeps + HasUserStore {}

impl<T: WorkerDeps + HasUserStore> Context for T {}

#[async_trait]
pub trait Has: Context + Sized {
    async fn update_query_user(&self, command: Command) -> Result<()> {
        handler(self, command).await
    }
}

async fn handle<C: Context>(
    context: &C,
    event: domain::Event,
) -> Result<(), Box<dyn std::error::Error>> {
    if let domain::Event::UserUpdated(event) = event {
        let user_store = context.user_store();
        let before = user_store
            .find_by_twitter_user_id(&event.twitter_user_id().to_string())
            .await?;
        let after = crate::user::User {
            user_id: event.user_id().to_string(),
            twitter_user_id: event.twitter_user_id().to_string(),
            twitter_user_name: event.twitter_user_name().to_string(),
        };
        user_store.store(before, after).await?;
    }
    Ok(())
}

pub async fn handler<C: Context>(context: &C, _: Command) -> Result<()> {
    Ok(worker_helper::worker(context, WorkerName::UpdateQueryUser, handle).await?)
}

// TODO: test
