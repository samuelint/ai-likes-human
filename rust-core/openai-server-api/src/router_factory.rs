use axum::{routing::get, Router};

use crate::api::chat_completions::run_chat_completions;

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/chat/completions", get(run_chat_completions))
}

async fn root() -> &'static str {
    "Hello, World!"
}
