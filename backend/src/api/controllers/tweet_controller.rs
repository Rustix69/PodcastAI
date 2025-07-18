use actix_web::{get, web, HttpResponse, Responder};
use crate::api::services::tweet_service;
use crate::api::models::tweet::{ErrorResponse, ContextAdditionResponse};

#[derive(serde::Deserialize)]
pub struct Query { 
    username: String, 
    max: Option<u8> 
}

#[derive(serde::Deserialize)]
pub struct ContextQuery { 
    username: String, 
    max: Option<u8>,
    user_id: Option<String>,
}

#[get("/tweets/original")]
pub async fn get_original_tweets(q: web::Query<Query>) -> impl Responder {
    let max = q.max.unwrap_or(20);
    match tweet_service::fetch_original_tweets(&q.username, max).await {
        Ok(tweets) => HttpResponse::Ok().json(tweets),
        Err(e) => HttpResponse::BadRequest().json(ErrorResponse { error: e }),
    }
}

#[get("/tweets/processed")]
pub async fn get_processed_tweets(q: web::Query<Query>) -> impl Responder {
    let max = q.max.unwrap_or(20);
    match tweet_service::fetch_and_process_tweets(&q.username, max).await {
        Ok(processed_tweets) => HttpResponse::Ok().json(processed_tweets),
        Err(e) => HttpResponse::BadRequest().json(ErrorResponse { error: e }),
    }
}

#[get("/tweets/context-addition")]
pub async fn context_addition(q: web::Query<ContextQuery>) -> impl Responder {
    let max = q.max.unwrap_or(20);
    let user_id = q.user_id.as_deref().unwrap_or("default_user");
    
    match tweet_service::fetch_process_and_add_context(&q.username, max, user_id).await {
        Ok((processed_tweets, context_response)) => {
            HttpResponse::Ok().json(ContextAdditionResponse {
                success: true,
                message: format!(
                    "Successfully processed {} tweets from @{} and added to context processor. {}", 
                    processed_tweets.tweet_count, 
                    processed_tweets.username,
                    context_response.message
                ),
                username: processed_tweets.username,
                tweet_count: processed_tweets.tweet_count,
                context_added: context_response.success,
            })
        },
        Err(e) => HttpResponse::BadRequest().json(ErrorResponse { error: e }),
    }
}