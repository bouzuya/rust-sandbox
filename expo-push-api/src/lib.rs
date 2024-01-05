use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct SendPushNotificationsRequest(Vec<ExpoPushMessage>);

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpoPushMessage {
    pub to: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutable_content: Option<bool>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize)]
#[serde(untagged)]
pub enum SendPushNotificationsResponse {
    Successful { data: Vec<ExpoPushTicket> },
    Error { errors: Vec<RequestError> },
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize)]
pub struct RequestError {
    pub code: String,
    pub message: String,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ExpoPushReceiptId(pub String);

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize)]
#[serde(rename_all = "camelCase", tag = "status")]
pub enum ExpoPushTicket {
    Ok { id: ExpoPushReceiptId },
    Error(ExpoPushErrorReceipt),
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize)]
pub struct ExpoPushErrorReceipt {
    pub message: String,
    pub details: Option<ExpoPushErrorReceiptDetails>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize)]
pub struct ExpoPushErrorReceiptDetails {
    pub errors: Option<ExpoPushErrorReceiptDetailsError>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize)]
pub enum ExpoPushErrorReceiptDetailsError {
    DeviceNotRegistered,
    InvalidCredentials,
    MessageRateExceeded,
    MessageTooBig,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct GetPushNotificationReceiptsRequest {
    pub ids: Vec<ExpoPushReceiptId>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize)]
#[serde(untagged)]
pub enum GetPushNotificationReceiptsResponse {
    Successful {
        data: HashMap<ExpoPushReceiptId, ExpoPushReceipt>,
    },
    Error {
        errors: Vec<RequestError>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize)]
#[serde(rename_all = "camelCase", tag = "status")]
pub enum ExpoPushReceipt {
    Ok,
    Error(ExpoPushErrorReceipt),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expo_push_message() -> anyhow::Result<()> {
        let expo_push_message = ExpoPushMessage {
            to: vec!["ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]".to_string()],
            data: None,
            title: Some("hello".to_string()),
            body: Some("world".to_string()),
            ttl: None,
            expiration: None,
            priority: None,
            subtitle: None,
            sound: None,
            badge: None,
            channel_id: None,
            category_id: None,
            mutable_content: None,
        };
        assert_eq!(
            serde_json::to_string_pretty(&expo_push_message)?,
            r#"{
  "to": [
    "ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]"
  ],
  "title": "hello",
  "body": "world"
}"#
        );
        Ok(())
    }
}
