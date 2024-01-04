use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct SendPushNotificationsRequest(Vec<ExpoPushMessage>);

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpoPushMessage {
    pub to: Vec<String>,
    pub data: Option<serde_json::Value>,
    pub title: Option<String>,
    pub body: Option<String>,
    pub ttl: Option<i64>,
    pub expiration: Option<i64>,
    pub priority: Option<String>,
    pub subtitle: Option<String>,
    pub sound: Option<String>,
    pub badge: Option<i64>,
    pub channel_id: Option<String>,
    pub category_id: Option<String>,
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
    message: String,
    details: Option<serde_json::Value>,
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
