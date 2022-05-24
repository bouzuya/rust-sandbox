use std::collections::HashMap;

use serde::Deserialize;

use crate::{
    RetrieveItemAuthorResponse, RetrieveItemDomainMetadataResponse, RetrieveItemImageResponse,
    RetrieveItemImagesItemResponse, RetrieveItemTagsItemResponse, RetrieveItemVideosItemResponse,
};

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct RetrieveCompleteItemRawResponse {
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
    pub listen_duration_estimate: u16,
    pub resolved_id: String,
    pub resolved_title: Option<String>,
    pub resolved_url: Option<String>,
    pub sort_id: u64,
    pub status: String,
    pub tags: Option<HashMap<String, RetrieveItemTagsItemResponse>>,
    pub time_added: String,
    pub time_favorited: String,
    pub time_read: String,
    pub time_to_read: Option<u64>,
    pub time_updated: String,
    pub top_image_url: Option<String>,
    pub videos: Option<HashMap<String, RetrieveItemVideosItemResponse>>,
    pub word_count: Option<String>,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub enum RetrieveCompleteItemResponse {
    Resolved(RetrieveCompleteResolvedItemResponse),
    Unresolved(RetrieveCompleteUnresolvedItemResponse),
}

impl From<RetrieveCompleteItemRawResponse> for RetrieveCompleteItemResponse {
    fn from(item: RetrieveCompleteItemRawResponse) -> Self {
        if &item.resolved_id == "0" {
            Self::Unresolved(RetrieveCompleteUnresolvedItemResponse::from(item))
        } else {
            Self::Resolved(RetrieveCompleteResolvedItemResponse::from(item))
        }
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct RetrieveCompleteUnresolvedItemResponse {
    pub favorite: String,
    pub given_title: String,
    pub given_url: String,
    pub item_id: String,
    pub listen_duration_estimate: u16,
    pub resolved_id: String,
    pub sort_id: u64,
    pub status: String,
    pub tags: Option<HashMap<String, RetrieveItemTagsItemResponse>>,
    pub time_added: String,
    pub time_favorited: String,
    pub time_read: String,
    pub time_updated: String,
}

impl From<RetrieveCompleteItemRawResponse> for RetrieveCompleteUnresolvedItemResponse {
    fn from(item: RetrieveCompleteItemRawResponse) -> Self {
        Self {
            favorite: item.favorite,
            given_title: item.given_title,
            given_url: item.given_url,
            item_id: item.item_id,
            listen_duration_estimate: item.listen_duration_estimate,
            resolved_id: item.resolved_id,
            sort_id: item.sort_id,
            status: item.status,
            tags: item.tags,
            time_added: item.time_added,
            time_favorited: item.time_favorited,
            time_read: item.time_read,
            time_updated: item.time_updated,
        }
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct RetrieveCompleteResolvedItemResponse {
    pub authors: Option<HashMap<String, RetrieveItemAuthorResponse>>,
    pub domain_metadata: Option<RetrieveItemDomainMetadataResponse>,
    pub excerpt: String,
    pub favorite: String,
    pub given_title: String,
    pub given_url: String,
    pub has_image: String,
    pub has_video: String,
    pub image: Option<RetrieveItemImageResponse>,
    pub images: Option<HashMap<String, RetrieveItemImagesItemResponse>>,
    pub is_article: String,
    pub is_index: String,
    pub item_id: String,
    pub lang: String,
    pub listen_duration_estimate: u16,
    pub resolved_id: String,
    pub resolved_title: String,
    pub resolved_url: String,
    pub sort_id: u64,
    pub status: String,
    pub tags: Option<HashMap<String, RetrieveItemTagsItemResponse>>,
    pub time_added: String,
    pub time_favorited: String,
    pub time_read: String,
    pub time_to_read: Option<u64>,
    pub time_updated: String,
    pub top_image_url: Option<String>,
    pub videos: Option<HashMap<String, RetrieveItemVideosItemResponse>>,
    pub word_count: String,
}

impl From<RetrieveCompleteItemRawResponse> for RetrieveCompleteResolvedItemResponse {
    fn from(item: RetrieveCompleteItemRawResponse) -> Self {
        Self {
            authors: item.authors,
            domain_metadata: item.domain_metadata,
            excerpt: item.excerpt.expect("excerpt is None"),
            favorite: item.favorite,
            given_title: item.given_title,
            given_url: item.given_url,
            has_image: item.has_image.expect("has_image is None"),
            has_video: item.has_video.expect("has_video is None"),
            image: item.image,
            images: item.images,
            is_article: item.is_article.expect("is_article is None"),
            is_index: item.is_index.expect("is_index is None"),
            item_id: item.item_id,
            lang: item.lang.expect("lang is None"),
            listen_duration_estimate: item.listen_duration_estimate,
            resolved_id: item.resolved_id,
            resolved_title: item.resolved_title.expect("resolved_title is None"),
            resolved_url: item.resolved_url.expect("resolved_url is None"),
            sort_id: item.sort_id,
            status: item.status,
            tags: item.tags,
            time_added: item.time_added,
            time_favorited: item.time_favorited,
            time_read: item.time_read,
            time_to_read: item.time_to_read,
            time_updated: item.time_updated,
            top_image_url: item.top_image_url,
            videos: item.videos,
            word_count: item.word_count.expect("word_count is None"),
        }
    }
}
