use std::{fs, path::Path};

use crate::{domain, store::TweetStore};
use anyhow::Context;
use time::{
    format_description::{self, well_known::Iso8601},
    OffsetDateTime,
};
use tracing::instrument;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Item {
    tweet: Tweet,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Tweet {
    retweeted: bool,
    source: String,
    entities: TweetEntities,
    display_text_range: Vec<String>,
    favorite_count: String,
    id_str: String,
    truncated: bool,
    retweet_count: String,
    id: String,
    created_at: String,
    favorited: bool,
    full_text: String,
    lang: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct TweetEntities {
    hashtags: Vec<TweetEntitiesHashTag>,
    media: Option<Vec<TweetEntitiesMedia>>,
    symbols: Vec<TweetEntitiesSymbol>,
    user_mentions: Vec<TweetEntitiesUserMention>,
    urls: Vec<TweetEntitiesUrl>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct TweetEntitiesHashTag {
    text: String,
    indices: Vec<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct TweetEntitiesMedia {
    expanded_url: String,
    source_status_id: Option<String>,
    indices: Vec<String>,
    url: String,
    media_url: String,
    id_str: String,
    video_info: Option<TweetEntitiesMediaVideoInfo>,
    source_user_id: Option<String>,
    additional_media_info: Option<TweetEntitiesMediaAdditionalMediaInfo>,
    id: String,
    media_url_https: String,
    source_user_id_str: Option<String>,
    sizes: TweetEntitiesMediaSizes,
    r#type: String, // "animated_gif" | "photo" | "video" | ...
    source_status_id_str: Option<String>,
    display_url: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct TweetEntitiesMediaAdditionalMediaInfo {
    monetizable: bool,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct TweetEntitiesMediaVideoInfo {
    aspect_ratio: Vec<String>,
    duration_millis: Option<String>,
    variants: Vec<TweetEntitiesMediaVideoInfoVariant>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct TweetEntitiesMediaVideoInfoVariant {
    bitrate: String,
    content_type: String,
    url: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct TweetEntitiesMediaSizes {
    thumb: TweetEntitiesMediaSize,
    small: TweetEntitiesMediaSize,
    medium: TweetEntitiesMediaSize,
    large: TweetEntitiesMediaSize,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct TweetEntitiesMediaSize {
    w: String,
    h: String,
    resize: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct TweetEntitiesSymbol {
    text: String,
    indices: Vec<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct TweetEntitiesUserMention {
    name: String,
    screen_name: String,
    indices: Vec<String>,
    id_str: String,
    id: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct TweetEntitiesUrl {
    url: String,
    expanded_url: String,
    display_url: String,
    indices: Vec<String>,
}

impl Item {
    fn parse(self) -> domain::MyTweet {
        let asctime = format_description::parse(
            "[weekday repr:short case_sensitive:true] [month repr:short case_sensitive:true] [day padding:zero] [hour padding:zero repr:24]:[minute padding:zero]:[second padding:zero] [offset_hour padding:zero sign:mandatory][offset_minute padding:zero] [year padding:none repr:full base:calendar sign:automatic]",
        ).unwrap();
        let tweet = self.tweet;
        let text = tweet.full_text;
        domain::MyTweet {
            id_str: tweet.id_str,
            at: OffsetDateTime::parse(tweet.created_at.as_str(), &asctime)
                .with_context(|| tweet.created_at)
                .unwrap()
                .format(&Iso8601::DEFAULT)
                .unwrap(),
            text,
        }
    }
}

#[instrument(skip_all)]
pub async fn run<P: AsRef<Path>>(store: TweetStore, file: P) -> anyhow::Result<()> {
    let s = fs::read_to_string(file)?;
    let json: Vec<Item> = serde_json::from_str(s.trim_start_matches("window.YTD.tweet.part0 = "))?;

    let mut data = store.read_all().await?;
    for tweet in json.into_iter().map(|item| item.parse()) {
        data.insert(tweet.id_str.clone(), tweet);
    }
    store.write_all(&data).await?;
    Ok(())
}
