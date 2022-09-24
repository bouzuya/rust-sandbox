pub mod user_request_created;
pub mod user_request_finished;
pub mod user_request_started;

pub use self::user_request_created::UserRequestCreated;
pub use self::user_request_finished::UserRequestFinished;
pub use self::user_request_started::UserRequestStarted;

use event_store_core::Event as RawEvent;

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("unknown {0}")]
    Unknown(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Event {
    Created(UserRequestCreated),
    Started(UserRequestStarted),
    Finished(UserRequestFinished),
}

macro_rules! impl_from_and_try_from {
    ($constructor: path, $ty: ty) => {
        impl From<$ty> for Event {
            fn from(value: $ty) -> Self {
                $constructor(value)
            }
        }

        impl TryFrom<Event> for $ty {
            type Error = Error;

            fn try_from(value: Event) -> Result<Self, Self::Error> {
                if let $constructor(value) = value {
                    Ok(value)
                } else {
                    Err(Error::Unknown("try from failed".to_owned()))
                }
            }
        }
    };
}

impl_from_and_try_from!(Event::Created, UserRequestCreated);
impl_from_and_try_from!(Event::Finished, UserRequestFinished);
impl_from_and_try_from!(Event::Started, UserRequestStarted);

impl From<Event> for RawEvent {
    fn from(event: Event) -> Self {
        match event {
            Event::Created(e) => RawEvent::from(e),
            Event::Started(e) => RawEvent::from(e),
            Event::Finished(e) => RawEvent::from(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use core::fmt::Debug;

    pub(in crate::aggregate::user_request::event) fn serde_test<T>(
        o: T,
        s: &str,
    ) -> anyhow::Result<()>
    where
        T: Debug + Eq + serde::de::DeserializeOwned + serde::Serialize,
    {
        let deserialized: T = serde_json::from_str(s)?;
        assert_eq!(deserialized, o);
        assert_eq!(serde_json::to_string_pretty(&o)?, s);
        Ok(())
    }
}
