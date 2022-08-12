use std::{env, fs::read_to_string};

use anyhow::Context;
use time::{
    format_description::{self, well_known::Iso8601},
    OffsetDateTime,
};

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
    fn parse(self, user_id: &str) -> domain::MyTweet {
        let asctime = format_description::parse(
            "[weekday repr:short case_sensitive:true] [month repr:short case_sensitive:true] [day padding:zero] [hour padding:zero repr:24]:[minute padding:zero]:[second padding:zero] [offset_hour padding:zero sign:mandatory][offset_minute padding:zero] [year padding:none repr:full base:calendar sign:automatic]",
        ).unwrap();
        let tweet = self.tweet;
        let retweet = tweet.full_text.starts_with("RT @");
        let text = if retweet {
            tweet.full_text.trim_start_matches("RT ").to_string()
        } else {
            tweet.full_text
        };
        let author_id = if retweet {
            tweet
                .entities
                .user_mentions
                .iter()
                .find_map(|mention| {
                    if mention.indices[0] == "3" {
                        Some(mention.id_str.to_string())
                    } else {
                        None
                    }
                })
                .unwrap()
        } else {
            user_id.to_string()
        };
        domain::MyTweet {
            id_str: tweet.id_str,
            retweet,
            at: OffsetDateTime::parse(tweet.created_at.as_str(), &asctime)
                .with_context(|| tweet.created_at)
                .unwrap()
                .format(&Iso8601::DEFAULT)
                .unwrap(),
            author_id,
            text,
            entities: domain::MyTweetEntities {
                hashtags: tweet
                    .entities
                    .hashtags
                    .into_iter()
                    .map(|hashtag| domain::MyTweetHashtag {
                        end: hashtag.indices[1].parse::<usize>().unwrap(),
                        start: hashtag.indices[0].parse::<usize>().unwrap(),
                        tag: hashtag.text,
                    })
                    .collect::<Vec<_>>(),
                mentions: tweet
                    .entities
                    .user_mentions
                    .into_iter()
                    .map(|mention| domain::MyTweetMention {
                        end: mention.indices[0].parse::<usize>().unwrap(),
                        start: mention.indices[1].parse::<usize>().unwrap(),
                        username: mention.name,
                    })
                    .collect::<Vec<_>>(),
                urls: tweet
                    .entities
                    .urls
                    .into_iter()
                    .map(|url| domain::MyTweetUrl {
                        display_url: url.display_url,
                        end: url.indices[0].parse::<usize>().unwrap(),
                        expanded_url: url.expanded_url,
                        start: url.indices[1].parse::<usize>().unwrap(),
                        url: url.url,
                    })
                    .collect::<Vec<_>>(),
            },
        }
    }
}

fn main() -> anyhow::Result<()> {
    let args = env::args().collect::<Vec<String>>();
    let file = &args[1];
    let s = read_to_string(file)?;
    let json: Vec<Item> = serde_json::from_str(s.trim_start_matches("window.YTD.tweet.part0 = "))?;
    // TODO: import
    // println!("{}", serde_json::to_string_pretty(&json)?);
    let tweets = json
        .into_iter()
        .take(10)
        .map(|item| item.parse("125962981"))
        .collect::<Vec<_>>();
    println!("{:?}", tweets);
    Ok(())
}
