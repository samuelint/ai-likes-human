use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};

use crate::{
    api::{chat_completions::run_chat_completions, health::health},
    trace::with_tracing,
};

pub fn create_router() -> Router {
    let router = Router::new()
        .route("/", get(health))
        .route("/health", get(health))
        .route("/openai/v1/chat/completions", post(run_chat_completions))
        .fallback(fallback);

    with_tracing(router)
}

async fn fallback() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "")
}
