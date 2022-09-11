pub mod user_request_created;

pub use self::user_request_created::UserRequestCreated;

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct UserRequestStarted;

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct UserRequestFinished;

pub enum Event {
    Created(UserRequestCreated),
    Started(UserRequestStarted),
    Finished(UserRequestFinished),
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

    // TODO: test
}
