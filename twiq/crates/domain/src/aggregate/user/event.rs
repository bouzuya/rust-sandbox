pub struct UserCreated;

pub struct UserFetchRequested;

pub struct UserUpdated;

pub enum Event {
    Created(UserCreated),
    Updated(UserUpdated),
    FetchRequested(UserFetchRequested),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_test() -> anyhow::Result<()> {
        // TODO
        Ok(())
    }

    #[test]
    fn user_created_test() -> anyhow::Result<()> {
        // TODO
        // let deserialized: Created = serde_json::from_str(
        //     r#"{"id":"id1","at":"at1","user_id":"user_id1","twitter_user_id":"twitter_user_id1"}"#,
        // )?;
        // assert_eq!(
        //     deserialized,
        //     Created {
        //         id: "id1".to_owned(),
        //         at: "at1".to_owned(),
        //         user_id: "user_id1".to_owned(),
        //         twitter_user_id: "twitter_user_id1".to_owned()
        //     }
        // );
        Ok(())
    }

    #[test]
    fn user_fetch_requested_test() -> anyhow::Result<()> {
        // TODO
        Ok(())
    }

    #[test]
    fn user_updated_test() -> anyhow::Result<()> {
        // TODO
        Ok(())
    }
}
