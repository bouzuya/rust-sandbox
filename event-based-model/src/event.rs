#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(tag = "kind")]
pub enum UserEvent {
    Created(UserCreated),
    Updated(UserUpdated),
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct UserCreated {
    pub at: String,
    pub id: String,
    pub name: String,
    pub version: u32,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct UserUpdated {
    pub at: String,
    pub id: String,
    pub name: String,
    pub version: u32,
}
