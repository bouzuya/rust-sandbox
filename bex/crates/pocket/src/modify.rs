use serde::{Deserialize, Serialize};

use crate::{request::post, Error};

#[derive(Debug, Serialize)]
pub struct ModifyRequest<'a> {
    pub consumer_key: &'a str,
    pub access_token: &'a str,
    pub actions: Vec<ModifyRequestAction<'a>>,
}

#[derive(Debug, Serialize)]
#[serde(tag = "action")]
pub enum ModifyRequestAction<'a> {
    #[serde(rename = "archive")]
    Archive {
        item_id: &'a str,
        #[serde(skip_serializing_if = "Option::is_none")]
        time: Option<&'a str>,
    },
    // TODO:
}

#[derive(Debug, Deserialize)]
pub struct ModifyResponse {
    action_results: Vec<bool>,
    status: u16,
}

// <https://getpocket.com/developer/docs/v3/modify>
pub async fn modify_request(request: &ModifyRequest<'_>) -> Result<ModifyResponse, Error> {
    post("https://getpocket.com/v3/send", request).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_test() -> anyhow::Result<()> {
        let request = ModifyRequest {
            consumer_key: "consumer_key1",
            access_token: "access_token1",
            actions: vec![ModifyRequestAction::Archive {
                item_id: "229279689",
                time: Some("1348853312"),
            }],
        };
        assert_eq!(
            serde_json::to_string_pretty(&request)?,
            r#"{
  "consumer_key": "consumer_key1",
  "access_token": "access_token1",
  "actions": [
    {
      "action": "archive",
      "item_id": "229279689",
      "time": "1348853312"
    }
  ]
}"#
        );
        Ok(())
    }
}
