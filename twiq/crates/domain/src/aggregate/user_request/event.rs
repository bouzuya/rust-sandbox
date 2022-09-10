#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct UserRequestCreated;

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct UserRequestStarted;

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct UserRequestFinished;

pub enum Event {
    Created(UserRequestCreated),
    Started(UserRequestStarted),
    Finished(UserRequestFinished),
}

// TODO: test
