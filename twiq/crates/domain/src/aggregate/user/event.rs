pub mod user_created;
pub mod user_requested;
pub mod user_updated;

pub use self::user_created::UserCreated;
pub use self::user_requested::UserRequested;
pub use self::user_updated::UserUpdated;

use event_store_core::Event as RawEvent;

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("unknown {0}")]
    Unknown(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Event {
    Created(UserCreated),
    Requested(UserRequested),
    Updated(UserUpdated),
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

impl_from_and_try_from!(Event::Created, UserCreated);
impl_from_and_try_from!(Event::Requested, UserRequested);
impl_from_and_try_from!(Event::Updated, UserUpdated);

impl From<Event> for RawEvent {
    fn from(event: Event) -> Self {
        match event {
            Event::Created(e) => RawEvent::from(e),
            Event::Requested(e) => RawEvent::from(e),
            Event::Updated(e) => RawEvent::from(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use core::fmt::Debug;

    pub(in crate::aggregate::user::event) fn serde_test<T>(o: T, s: &str) -> anyhow::Result<()>
    where
        T: Debug + Eq + serde::de::DeserializeOwned + serde::Serialize,
    {
        let deserialized: T = serde_json::from_str(s)?;
        assert_eq!(deserialized, o);
        assert_eq!(serde_json::to_string_pretty(&o)?, s);
        Ok(())
    }
}
