use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct RetrieveResponse<T> {
    pub complete: Option<u16>,
    pub error: Option<Value>,
    pub list: HashMap<String, T>,
    pub search_meta: Option<RetrieveSearchMetaResponse>,
    pub since: Option<u64>,
    pub status: u16,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct RetrieveSearchMetaResponse {
    search_type: String,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct RetrieveItemResponse {
    pub authors: Option<HashMap<String, RetrieveItemAuthorResponse>>,
    pub domain_metadata: Option<RetrieveItemDomainMetadataResponse>,
    pub excerpt: Option<String>,
    pub favorite: String,
    pub given_title: String,
    pub given_url: String,
    pub has_image: Option<String>,
    pub has_video: Option<String>,
    pub image: Option<RetrieveItemImageResponse>,
    pub images: Option<HashMap<String, RetrieveItemImagesItemResponse>>,
    pub is_article: Option<String>,
    pub is_index: Option<String>,
    pub item_id: String,
    pub lang: Option<String>,
    pub listen_duration_estimate: Option<u16>,
    pub resolved_id: String,
    pub resolved_title: Option<String>,
    pub resolved_url: Option<String>,
    pub sort_id: Option<u64>,
    pub status: String,
    pub tags: Option<HashMap<String, RetrieveItemTagsItemResponse>>,
    pub time_added: Option<String>,
    pub time_favorited: Option<String>,
    pub time_read: Option<String>,
    pub time_to_read: Option<u64>,
    pub time_updated: Option<String>,
    pub top_image_url: Option<String>,
    pub videos: Option<HashMap<String, RetrieveItemVideosItemResponse>>,
    pub word_count: Option<String>,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct RetrieveItemAuthorResponse {
    author_id: String,
    item_id: String,
    name: String,
    url: String,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct RetrieveItemDomainMetadataResponse {
    greyscale_logo: String,
    logo: String,
    name: String,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct RetrieveItemImageResponse {
    height: String,
    item_id: String,
    src: String,
    width: String,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct RetrieveItemImagesItemResponse {
    caption: String,
    credit: String,
    height: String,
    image_id: String,
    item_id: String,
    src: String,
    width: String,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct RetrieveItemTagsItemResponse {
    item_id: String,
    tag: String,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct RetrieveItemVideosItemResponse {
    height: String,
    item_id: String,
    // It seems to be required but is not included in the examples in the document.
    length: Option<String>,
    src: String,
    r#type: String,
    vid: String,
    video_id: String,
    width: String,
}
