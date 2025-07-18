use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use std::env;
use regex::Regex;
use crate::api::models::tweet::{Tweet, TwitterApiResponse, ProcessedTweets, ContextRequest, ContextDocument, ContextMetadata, ContextResponse};

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

pub async fn send_to_context_processor(
    processed_tweets: &ProcessedTweets,
    user_id: &str
) -> Result<ContextResponse, String> {
    let alchemyst_api_key = env::var("ALCHEMYST_API_KEY")
        .map_err(|_| "Missing ALCHEMYST_API_KEY".to_string())?;
    let alchemyst_base_url = env::var("ALCHEMYST_BASE_URL")
        .unwrap_or_else(|_| "https://api.alchemyst.ai".to_string());
    
    let url = format!("{}/api/v1/context/add", alchemyst_base_url);
    
    let context_request = ContextRequest {
        user_id: user_id.to_string(),
        organization_id: None,
        documents: vec![ContextDocument {
            content: processed_tweets.processed_text.clone(),
        }],
        source: "twitter_podcast_ai".to_string(),
        context_type: "resource".to_string(),
        scope: "internal".to_string(),
        metadata: ContextMetadata {
            file_name: format!("{}_tweets.txt", processed_tweets.username),
            doc_type: "text/plain".to_string(),
            modalities: vec!["text".to_string()],
            size: processed_tweets.processed_text.len() as u64,
        },
    };

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .header(AUTHORIZATION, format!("Bearer {}", alchemyst_api_key))
        .header(CONTENT_TYPE, "application/json")
        .json(&context_request)
        .send()
        .await
        .map_err(|e| format!("Context API request failed: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("Context API failed with status {}: {}", status, error_text));
    }

    let context_response: ContextResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse context API response: {}", e))?;

    Ok(context_response)
}

pub async fn fetch_process_and_add_context(
    username: &str, 
    max: u8, 
    user_id: &str
) -> Result<(ProcessedTweets, ContextResponse), String> {
    // Step 1: Fetch and process tweets
    let processed_tweets = fetch_and_process_tweets(username, max).await?;
    
    // Step 2: Send to context processor
    let context_response = send_to_context_processor(&processed_tweets, user_id).await?;
    
    Ok((processed_tweets, context_response))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::models::tweet::{Tweet, PublicMetrics};

    #[test]
    fn test_clean_tweet_text() {
        let tweet_with_url = "Building a great app! Check it out: https://t.co/abc123def  ";
        let cleaned = clean_tweet_text(tweet_with_url);
        assert_eq!(cleaned, "Building a great app! Check it out:");

        let tweet_without_url = "People who choose themselves always win no matter how bad the situation gets.";
        let cleaned = clean_tweet_text(tweet_without_url);
        assert_eq!(cleaned, "People who choose themselves always win no matter how bad the situation gets.");
    }

    #[test]
    fn test_clean_tweet_text_multiple_urls() {
        let tweet_with_multiple_urls = "Check this out https://t.co/abc123 and also this https://t.co/def456 amazing!";
        let cleaned = clean_tweet_text(tweet_with_multiple_urls);
        assert_eq!(cleaned, "Check this out and also this amazing!");
    }

    #[test]
    fn test_clean_tweet_text_only_url() {
        let tweet_only_url = "https://t.co/abc123def";
        let cleaned = clean_tweet_text(tweet_only_url);
        assert_eq!(cleaned, "");
    }

    #[test]
    fn test_clean_tweet_text_excessive_whitespace() {
        let tweet_with_spaces = "Too    many   spaces    here   !";
        let cleaned = clean_tweet_text(tweet_with_spaces);
        assert_eq!(cleaned, "Too many spaces here !");
    }

    #[test]
    fn test_clean_tweet_text_empty() {
        let empty_tweet = "";
        let cleaned = clean_tweet_text(empty_tweet);
        assert_eq!(cleaned, "");
    }

    #[test]
    fn test_clean_tweet_text_with_emojis() {
        let tweet_with_emojis = "LFG 🚀 Hope so Gold will respect my levels. Otherwise C gaye guru. https://t.co/8n3oK3Ia4Z";
        let cleaned = clean_tweet_text(tweet_with_emojis);
        assert_eq!(cleaned, "LFG 🚀 Hope so Gold will respect my levels. Otherwise C gaye guru.");
    }

    #[test]
    fn test_process_tweets_to_text() {
        let tweets = vec![
            Tweet {
                id: "1".to_string(),
                edit_history_tweet_ids: vec!["1".to_string()],
                created_at: "2025-01-01T00:00:00.000Z".to_string(),
                text: "First tweet https://t.co/abc123".to_string(),
                public_metrics: PublicMetrics {
                    retweet_count: 0,
                    reply_count: 0,
                    like_count: 5,
                    quote_count: 0,
                    bookmark_count: 1,
                    impression_count: 100,
                },
            },
            Tweet {
                id: "2".to_string(),
                edit_history_tweet_ids: vec!["2".to_string()],
                created_at: "2025-01-02T00:00:00.000Z".to_string(),
                text: "Second tweet".to_string(),
                public_metrics: PublicMetrics {
                    retweet_count: 1,
                    reply_count: 2,
                    like_count: 10,
                    quote_count: 0,
                    bookmark_count: 0,
                    impression_count: 200,
                },
            },
        ];

        let result = process_tweets_to_text(&tweets, "testuser");
        let expected = "Here are the recent tweets from @testuser to be made into a podcast:\n\nFirst tweet\n\nSecond tweet";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_process_tweets_to_text_empty_list() {
        let tweets: Vec<Tweet> = vec![];
        let result = process_tweets_to_text(&tweets, "testuser");
        let expected = "Here are the recent tweets from @testuser to be made into a podcast:";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_process_tweets_to_text_single_tweet() {
        let tweets = vec![
            Tweet {
                id: "1".to_string(),
                edit_history_tweet_ids: vec!["1".to_string()],
                created_at: "2025-01-01T00:00:00.000Z".to_string(),
                text: "Only tweet https://t.co/test123".to_string(),
                public_metrics: PublicMetrics {
                    retweet_count: 0,
                    reply_count: 0,
                    like_count: 1,
                    quote_count: 0,
                    bookmark_count: 0,
                    impression_count: 50,
                },
            },
        ];

        let result = process_tweets_to_text(&tweets, "singleuser");
        let expected = "Here are the recent tweets from @singleuser to be made into a podcast:\n\nOnly tweet";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_context_request_creation() {
        let processed_tweets = ProcessedTweets {
            username: "testuser".to_string(),
            tweet_count: 2,
            processed_text: "Test tweet content".to_string(),
        };

        // This would be used in send_to_context_processor function
        let context_request = ContextRequest {
            user_id: "test_user_123".to_string(),
            organization_id: None,
            documents: vec![ContextDocument {
                content: processed_tweets.processed_text.clone(),
            }],
            source: "twitter_podcast_ai".to_string(),
            context_type: "resource".to_string(),
            scope: "internal".to_string(),
            metadata: ContextMetadata {
                file_name: format!("{}_tweets.txt", processed_tweets.username),
                doc_type: "text/plain".to_string(),
                modalities: vec!["text".to_string()],
                size: processed_tweets.processed_text.len() as u64,
            },
        };

        assert_eq!(context_request.user_id, "test_user_123");
        assert_eq!(context_request.source, "twitter_podcast_ai");
        assert_eq!(context_request.context_type, "resource");
        assert_eq!(context_request.scope, "internal");
        assert_eq!(context_request.documents.len(), 1);
        assert_eq!(context_request.documents[0].content, "Test tweet content");
        assert_eq!(context_request.metadata.file_name, "testuser_tweets.txt");
        assert_eq!(context_request.metadata.doc_type, "text/plain");
        assert_eq!(context_request.metadata.size, 18); // "Test tweet content".len()
    }

    #[test]
    fn test_context_request_with_organization() {
        let processed_tweets = ProcessedTweets {
            username: "corpuser".to_string(),
            tweet_count: 1,
            processed_text: "Corporate tweet".to_string(),
        };

        let context_request = ContextRequest {
            user_id: "corp_user_456".to_string(),
            organization_id: Some("org_123".to_string()),
            documents: vec![ContextDocument {
                content: processed_tweets.processed_text.clone(),
            }],
            source: "twitter_podcast_ai".to_string(),
            context_type: "conversation".to_string(),
            scope: "external".to_string(),
            metadata: ContextMetadata {
                file_name: format!("{}_tweets.txt", processed_tweets.username),
                doc_type: "text/plain".to_string(),
                modalities: vec!["text".to_string()],
                size: processed_tweets.processed_text.len() as u64,
            },
        };

        assert_eq!(context_request.organization_id, Some("org_123".to_string()));
        assert_eq!(context_request.context_type, "conversation");
        assert_eq!(context_request.scope, "external");
    }

    #[test]
    fn test_context_metadata_size_calculation() {
        let short_text = "Hi";
        let long_text = "This is a much longer text that should have a significantly larger size value when calculated";
        
        assert_eq!(short_text.len(), 2);
        assert_eq!(long_text.len(), 93);
        
        // Verify our size calculation is accurate
        let metadata_short = ContextMetadata {
            file_name: "test.txt".to_string(),
            doc_type: "text/plain".to_string(),
            modalities: vec!["text".to_string()],
            size: short_text.len() as u64,
        };
        
        let metadata_long = ContextMetadata {
            file_name: "test.txt".to_string(),
            doc_type: "text/plain".to_string(),
            modalities: vec!["text".to_string()],
            size: long_text.len() as u64,
        };
        
        assert_eq!(metadata_short.size, 2);
        assert_eq!(metadata_long.size, 93);
    }
}