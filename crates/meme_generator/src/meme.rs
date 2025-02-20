use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InfoMemeResponse {
    pub post_link: String,
    pub subreddit: String,
    pub title: String,
    pub url: String,
    pub nsfw: bool,
    pub spoiler: bool,
    pub author: String,
    pub ups: i64,
    pub preview: Vec<String>,
}

#[derive(Debug)]
pub struct MemeResponse {
    pub title: String,
    pub content: Vec<u8>,
}
