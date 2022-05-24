mod complete_response;
mod request;
mod response;
mod simple_response;

use std::collections::HashMap;

use crate::{post, Error};

pub use self::complete_response::*;
pub use self::request::*;
pub use self::response::*;
pub use self::simple_response::*;

// <https://getpocket.com/developer/docs/v3/retrieve>
pub async fn retrieve_request(
    request: &RetrieveRequest<'_>,
) -> Result<RetrieveResponse<RetrieveItemResponse>, Error> {
    post("https://getpocket.com/v3/get", request).await
}

// <https://getpocket.com/developer/docs/v3/retrieve>
pub async fn retrieve_simple_request(
    request: &RetrieveRequest<'_>,
) -> Result<RetrieveResponse<RetrieveSimpleItemResponse>, Error> {
    if request.detail_type != Some(RetrieveRequestDetailType::Simple) {
        return Err(Error::InvalidRequest);
    }
    let response: RetrieveResponse<RetrieveSimpleItemRawResponse> =
        post("https://getpocket.com/v3/get", request).await?;
    Ok(RetrieveResponse {
        complete: response.complete,
        error: response.error,
        list: response
            .list
            .into_iter()
            .map(|(item_id, item)| (item_id, RetrieveSimpleItemResponse::from(item)))
            .collect::<HashMap<String, RetrieveSimpleItemResponse>>(),
        search_meta: response.search_meta,
        since: response.since,
        status: response.status,
    })
}

// <https://getpocket.com/developer/docs/v3/retrieve>
pub async fn retrieve_complete_request(
    request: &RetrieveRequest<'_>,
) -> Result<RetrieveResponse<RetrieveCompleteItemResponse>, Error> {
    if request.detail_type != Some(RetrieveRequestDetailType::Complete) {
        return Err(Error::InvalidRequest);
    }
    let response: RetrieveResponse<RetrieveCompleteItemRawResponse> =
        post("https://getpocket.com/v3/get", request).await?;
    Ok(RetrieveResponse {
        complete: response.complete,
        error: response.error,
        list: response
            .list
            .into_iter()
            .map(|(item_id, item)| (item_id, RetrieveCompleteItemResponse::from(item)))
            .collect::<HashMap<String, RetrieveCompleteItemResponse>>(),
        search_meta: response.search_meta,
        since: response.since,
        status: response.status,
    })
}
