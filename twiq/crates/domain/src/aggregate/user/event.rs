#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Created {
    id: String,
    at: String,
    user_id: String,
    twitter_user_id: String,
}

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct FetchRequested {
    id: String,
    at: String,
    user_id: String,
    twitter_user_id: String,
}

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct FetchResultReceived {
    id: String,
    at: String,
    user_id: String,
    twitter_user_id: String,
    status_code: u16,
    response_body: String,
}

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Event {
    Created(Created),
    FetchRequested(FetchRequested),
    FetchResultReceived(FetchResultReceived),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn created_test() -> anyhow::Result<()> {
        let deserialized: Created = serde_json::from_str(
            r#"{"id":"id1","at":"at1","user_id":"user_id1","twitter_user_id":"twitter_user_id1"}"#,
        )?;
        assert_eq!(
            deserialized,
            Created {
                id: "id1".to_owned(),
                at: "at1".to_owned(),
                user_id: "user_id1".to_owned(),
                twitter_user_id: "twitter_user_id1".to_owned()
            }
        );
        Ok(())
    }

    // TODO: fetch_requested_test
    // TODO: fetch_result_received_test
}
