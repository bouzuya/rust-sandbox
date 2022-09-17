use crate::{
    event_store::HasEventStore,
    user_repository::{HasUserRepository, UserRepository},
    user_request_repository::{HasUserRequestRepository, UserRequestRepository},
    worker_repository::{HasWorkerRepository, WorkerName},
};

use super::worker_helper;

pub struct Command;

async fn handle<C: HasUserRepository + HasUserRequestRepository>(
    context: &C,
    event: domain::Event,
) -> worker_helper::Result<()> {
    if let domain::Event::UserRequest(domain::aggregate::user_request::Event::Finished(event)) =
        event
    {
        let user_repository = context.user_repository();
        let user_request_repository = context.user_request_repository();
        let user_request = user_request_repository
            .find(event.user_request_id())
            .await?;
        let user = user_repository.find(event.user_id()).await?;
        match (user_request, user) {
            (None, _) => Err(worker_helper::Error::UserRequestNotFound(
                event.user_request_id(),
            )),
            (_, None) => Err(worker_helper::Error::UserNotFound(event.user_id())),
            (Some(_), Some(user)) => {
                let twitter_user_name = event.user_response().parse()?;
                let at = event.at();
                let mut updated = user.clone();
                updated.update(twitter_user_name, at)?;
                Ok(user_repository.store(Some(user), updated).await?)
            }
        }?;
    }
    Ok(())
}

pub async fn handler<C>(context: &C, _: Command) -> worker_helper::Result<()>
where
    C: HasEventStore + HasUserRepository + HasUserRequestRepository + HasWorkerRepository,
{
    worker_helper::worker(context, WorkerName::UpdateUser, handle).await
}

// TODO: test
