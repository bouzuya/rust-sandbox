use async_trait::async_trait;

use query_handler::user_store::{HasUserStore, UserStore};

use ::worker_helper::{
    worker_helper::{self, WorkerDeps},
    worker_repository::WorkerName,
};

pub struct Command;

pub trait Context: WorkerDeps + HasUserStore {}

impl<T: WorkerDeps + HasUserStore> Context for T {}

#[async_trait]
pub trait Has: Context + Sized {
    async fn update_query_user(&self, command: Command) -> worker_helper::Result<()> {
        handler(self, command).await
    }
}

async fn handle<C: Context>(context: &C, event: domain::Event) -> worker_helper::Result<()> {
    if let domain::Event::UserUpdated(event) = event {
        let user_store = context.user_store();
        let before = user_store
            .find_by_twitter_user_id(&event.twitter_user_id().to_string())
            .await?;
        let after = query_handler::user::User {
            user_id: event.user_id().to_string(),
            twitter_user_id: event.twitter_user_id().to_string(),
            twitter_user_name: event.twitter_user_name().to_string(),
        };
        user_store.store(before, after).await?;
    }
    Ok(())
}

pub async fn handler<C: Context>(context: &C, _: Command) -> worker_helper::Result<()> {
    worker_helper::worker(context, WorkerName::UpdateQueryUser, handle).await
}

// TODO: test
