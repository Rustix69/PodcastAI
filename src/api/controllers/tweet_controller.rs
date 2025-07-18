use actix_web::{get, web, HttpResponse, Responder};
use crate::api::services::tweet_service;
use crate::api::models::tweet::ErrorResponse;

#[derive(serde::Deserialize)]
pub struct Query { 
    username: String, 
    max: Option<u8> 
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