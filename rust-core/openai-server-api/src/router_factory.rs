use axum::{routing::get, Router};

use crate::api::{chat_completions::run_chat_completions, health::health};

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(health))
        .route("/health", get(health))
        .route("/openai/v1/chat/completions", get(run_chat_completions))
}
