use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct SendPushNotificationsRequest(Vec<Message>);

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct Message {
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
    Successful { data: Vec<PushTicket> },
    Error { errors: Vec<RequestError> },
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize)]
pub struct RequestError {
    pub code: String,
    pub message: String,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ReceiptId(pub String);

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize)]
#[serde(rename_all = "camelCase", tag = "status")]
pub enum PushTicket {
    Ok {
        id: ReceiptId,
    },
    Error {
        message: Option<String>,
        details: Option<serde_json::Value>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct GetPushNotificationReceiptsRequest {
    pub ids: Vec<ReceiptId>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize)]
#[serde(untagged)]
pub enum GetPushNotificationReceiptsResponse {
    Successful {
        data: HashMap<ReceiptId, PushReceipt>,
    },
    Error {
        errors: Vec<RequestError>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize)]
#[serde(rename_all = "camelCase", tag = "status")]
pub enum PushReceipt {
    Ok,
    Error {
        message: Option<String>,
        details: Option<serde_json::Value>,
    },
}
