use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use std::env;
use regex::Regex;
use crate::api::models::tweet::{Tweet, TwitterApiResponse, ProcessedTweets};

pub async fn fetch_original_tweets(username: &str, max: u8) -> Result<Vec<Tweet>, String> {
    let token = env::var("BEARER_TOKEN").map_err(|_| "Missing BEARER_TOKEN".to_string())?;
    let query = format!("from:{} -is:reply -is:retweet -is:quote", username);
    let url = format!(
        "https://api.x.com/2/tweets/search/recent?query={}&max_results={}&tweet.fields=created_at,public_metrics",
        urlencoding::encode(&query),
        max.clamp(10, 100)
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("API request failed with status {}: {}", status, error_text));
    }

    let api_response: TwitterApiResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse JSON response: {}", e))?;

    if api_response.data.is_empty() {
        return Err("No tweets found".to_string());
    }

    Ok(api_response.data)
}

pub async fn fetch_and_process_tweets(username: &str, max: u8) -> Result<ProcessedTweets, String> {
    let tweets = fetch_original_tweets(username, max).await?;
    
    let processed_text = process_tweets_to_text(&tweets, username);
    
    Ok(ProcessedTweets {
        username: username.to_string(),
        tweet_count: tweets.len(),
        processed_text,
    })
}

fn process_tweets_to_text(tweets: &[Tweet], username: &str) -> String {
    let mut result = format!("Here are the recent tweets from @{} to be made into a podcast:\n\n", username);
    
    for tweet in tweets {
        // Clean the tweet text by removing URLs and extra whitespace
        let cleaned_text = clean_tweet_text(&tweet.text);
        result.push_str(&cleaned_text);
        result.push_str("\n\n");
    }
    
    // Remove the last extra newlines
    result.trim_end().to_string()
}

fn clean_tweet_text(text: &str) -> String {
    // Remove URLs (https://t.co/... links)
    let url_pattern = Regex::new(r"https://t\.co/\w+").unwrap();
    let without_urls = url_pattern.replace_all(text, "").to_string();
    
    // Remove extra whitespace and clean up
    without_urls
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
        .trim()
        .to_string()
}