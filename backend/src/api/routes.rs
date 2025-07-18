use actix_web::web;
use crate::api::controllers::tweet_controller;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(tweet_controller::get_original_tweets)
       .service(tweet_controller::get_processed_tweets)
       .service(tweet_controller::context_addition);
}
