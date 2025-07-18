use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicMetrics {
    pub retweet_count: u64,
    pub reply_count: u64,
    pub like_count: u64,
    pub quote_count: u64,
    pub bookmark_count: u64,
    pub impression_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tweet {
    pub id: String,
    pub edit_history_tweet_ids: Vec<String>,
    pub created_at: String,
    pub text: String,
    pub public_metrics: PublicMetrics,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TweetMeta {
    pub newest_id: String,
    pub oldest_id: String,
    pub result_count: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TwitterApiResponse {
    pub data: Vec<Tweet>,
    pub meta: TweetMeta,
}

#[derive(Debug, Serialize)]
pub struct ProcessedTweets {
    pub username: String,
    pub tweet_count: usize,
    pub processed_text: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}