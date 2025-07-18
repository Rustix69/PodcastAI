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

// Alchemyst Context Processor Models
#[derive(Debug, Serialize)]
pub struct ContextDocument {
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct ContextMetadata {
    pub file_name: String,
    pub doc_type: String,
    pub modalities: Vec<String>,
    pub size: u64,
}

#[derive(Debug, Serialize)]
pub struct ContextRequest {
    pub user_id: String,
    pub organization_id: Option<String>,
    pub documents: Vec<ContextDocument>,
    pub source: String,
    pub context_type: String, // "resource", "conversation", "instruction"
    pub scope: String, // "internal", "external"
    pub metadata: ContextMetadata,
}

#[derive(Debug, Deserialize)]
pub struct ContextResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct ContextAdditionResponse {
    pub success: bool,
    pub message: String,
    pub username: String,
    pub tweet_count: usize,
    pub context_added: bool,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_twitter_api_response_deserialization() {
        let json_response = r#"{
            "data": [
                {
                    "id": "1945690992981717364",
                    "edit_history_tweet_ids": [
                        "1945690992981717364"
                    ],
                    "created_at": "2025-07-17T03:44:16.000Z",
                    "text": "People who choose themselves always win no matter how bad the situation gets.",
                    "public_metrics": {
                        "retweet_count": 0,
                        "reply_count": 0,
                        "like_count": 5,
                        "quote_count": 0,
                        "bookmark_count": 1,
                        "impression_count": 224
                    }
                }
            ],
            "meta": {
                "newest_id": "1945690992981717364",
                "oldest_id": "1943621572545167442",
                "result_count": 14
            }
        }"#;

        let response: TwitterApiResponse = serde_json::from_str(json_response)
            .expect("Failed to deserialize Twitter API response");

        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].id, "1945690992981717364");
        assert_eq!(response.data[0].public_metrics.like_count, 5);
        assert_eq!(response.meta.result_count, 14);
    }

    #[test]
    fn test_context_request_serialization() {
        let context_request = ContextRequest {
            user_id: "test_user".to_string(),
            organization_id: None,
            documents: vec![ContextDocument {
                content: "Test content".to_string(),
            }],
            source: "twitter_podcast_ai".to_string(),
            context_type: "resource".to_string(),
            scope: "internal".to_string(),
            metadata: ContextMetadata {
                file_name: "test.txt".to_string(),
                doc_type: "text/plain".to_string(),
                modalities: vec!["text".to_string()],
                size: 12,
            },
        };

        let json = serde_json::to_string(&context_request).expect("Failed to serialize");
        assert!(json.contains("test_user"));
        assert!(json.contains("twitter_podcast_ai"));
        assert!(json.contains("resource"));
    }

    #[test]
    fn test_context_response_deserialization() {
        let json_response = r#"{
            "success": true,
            "message": "Context added successfully"
        }"#;

        let response: ContextResponse = serde_json::from_str(json_response)
            .expect("Failed to deserialize context response");

        assert_eq!(response.success, true);
        assert_eq!(response.message, "Context added successfully");
    }

    #[test]
    fn test_processed_tweets_serialization() {
        let processed = ProcessedTweets {
            username: "testuser".to_string(),
            tweet_count: 5,
            processed_text: "Sample tweet text".to_string(),
        };

        let json = serde_json::to_string(&processed).expect("Failed to serialize");
        assert!(json.contains("testuser"));
        assert!(json.contains("\"tweet_count\":5"));
    }

    #[test]
    fn test_error_response_serialization() {
        let error = ErrorResponse {
            error: "Test error message".to_string(),
        };

        let json = serde_json::to_string(&error).expect("Failed to serialize");
        assert!(json.contains("Test error message"));
    }
}