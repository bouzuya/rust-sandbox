mod request;
mod response;

use crate::{post, Error};

pub use self::request::*;
pub use self::response::*;

// <https://getpocket.com/developer/docs/v3/retrieve>
pub async fn retrieve_request(request: &RetrieveRequest<'_>) -> Result<RetrieveResponse, Error> {
    post("https://getpocket.com/v3/get", request).await
}
