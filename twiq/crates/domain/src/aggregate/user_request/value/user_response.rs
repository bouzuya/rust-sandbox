use crate::aggregate::user::TwitterUserName;

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("invalid body {0}")]
    InvalidBody(String),
    #[error("invalid name {0}")]
    InvalidName(String),
    #[error("invalid status code {0}")]
    InvalidStatusCode(u16),
    #[error("unknown {0}")]
    Unknown(String),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserResponse {
    status_code: u16,
    body: String,
}

impl UserResponse {
    pub fn new(status_code: u16, body: String) -> Self {
        Self { status_code, body }
    }

    pub fn status_code(&self) -> u16 {
        self.status_code
    }

    pub fn body(&self) -> &str {
        &self.body
    }

    pub fn parse(&self) -> Result<TwitterUserName> {
        if self.status_code != 200 {
            return Err(Error::InvalidStatusCode(self.status_code));
        }

        let x: TwitterUserResponse = serde_json::from_str(self.body.as_str())
            .map_err(|e| Error::InvalidBody(e.to_string()))?;
        TwitterUserName::try_from(x.data.name).map_err(|e| Error::InvalidName(e.to_string()))
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct TwitterUserResponse {
    data: TwitterUserResponseData,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct TwitterUserResponseData {
    id: String,
    name: String,
    username: String,
}

#[cfg(test)]
mod tests {
    // TODO: test new
    // TODO: test status_code
    // TODO: test body

    use std::str::FromStr;

    use super::*;

    #[test]
    fn parse_test() -> anyhow::Result<()> {
        let body = r#"
{
  "data": {
    "id": "2244994945",
    "name": "Twitter Dev",
    "username": "TwitterDev"
  }
}"#;
        let user_response = UserResponse::new(200, body.to_owned());
        assert_eq!(
            user_response.parse()?,
            TwitterUserName::from_str("Twitter Dev")?
        );
        Ok(())
    }
}
