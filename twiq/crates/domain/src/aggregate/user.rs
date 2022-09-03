mod event;

use self::event::{Event, UserCreated, UserFetchRequested, UserUpdated};

#[derive(Debug, thiserror::Error)]
#[error("error")]
pub struct Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub struct User {
    events: Vec<Event>,
}

impl User {
    pub fn create() -> Self {
        // TODO
        Self {
            events: vec![Event::Created(UserCreated)],
        }
    }

    pub fn fetch_request(&mut self) -> Result<()> {
        // TODO
        self.events.push(Event::FetchRequested(UserFetchRequested));
        Ok(())
    }

    pub fn update(&mut self) -> Result<()> {
        // TODO
        self.events.push(Event::Updated(UserUpdated));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_test() {
        // TODO
    }

    #[test]
    fn fetch_request_test() {
        // TODO
    }

    #[test]
    fn update_test() {
        // TODO
    }
}
