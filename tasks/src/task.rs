#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Task {
    pub done: bool,
    pub id: usize,
    pub text: String,
}
