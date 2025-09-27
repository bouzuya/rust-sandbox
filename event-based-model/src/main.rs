use crate::aggregates::User;
use crate::aggregates::UserError;

use crate::value_objects::UserName;
use crate::value_objects::UserNameError;

mod aggregates;
mod event;
mod value_objects;

#[derive(Debug, thiserror::Error)]
enum AppError {
    #[error("create")]
    Create(#[source] UserError),
    #[error("from events")]
    FromEvents(#[source] UserError),
    #[error("update")]
    Update(#[source] UserError),
    #[error("user name")]
    UserName(#[source] UserNameError),
}

fn main() {
    sample1().unwrap();
}

fn sample1() -> Result<(), AppError> {
    let name1 = UserName::try_from("Alice".to_owned()).map_err(AppError::UserName)?;
    let (created, create_events) = User::create(name1).map_err(AppError::Create)?;
    let name2 = UserName::try_from("Bob".to_owned()).map_err(AppError::UserName)?;
    let (updated, update_events) = created.update(name2).map_err(AppError::Update)?;

    let replayed = User::from_events(create_events.into_iter().chain(update_events))
        .map_err(AppError::FromEvents)?;
    assert_eq!(updated, replayed);
    Ok(())
}
