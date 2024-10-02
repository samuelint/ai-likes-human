use std::sync::Arc;

use axum::{
    routing::{delete, get, post},
    Router,
};
use tower_http::limit::RequestBodyLimitLayer;

use crate::{
    app_state::ServerState,
    controller::{
        create_run_and_execute, create_thread, create_thread_create_run_and_execute,
        create_thread_message, delete_thread, delete_thread_message, find_thread,
        find_thread_message, find_thread_run, list_thread_messages, list_thread_runs, list_threads,
        run_chat_completions, update_thread,
    },
};

pub fn create_openai_v1_router(state: Arc<ServerState>) -> Router {
    let thread_route = create_thread_router(Arc::clone(&state));

    Router::new()
        .route("/chat/completions", post(run_chat_completions))
        .nest("/threads", thread_route)
        .with_state(Arc::clone(&state))
}

fn create_thread_router(state: Arc<ServerState>) -> Router<Arc<ServerState>> {
    Router::new()
        .route("/", post(create_thread))
        .route("/runs", post(create_thread_create_run_and_execute))
        .route("/", get(list_threads))
        .route("/:thread_id", get(find_thread))
        .route("/:thread_id", post(update_thread))
        .route("/:thread_id", delete(delete_thread))
        .route("/:thread_id/messages", get(list_thread_messages))
        .route("/:thread_id/messages/:message_id", get(find_thread_message))
        .route(
            "/:thread_id/messages/:message_id",
            delete(delete_thread_message),
        )
        .route(
            "/:thread_id/messages",
            post(create_thread_message).layer(RequestBodyLimitLayer::new(1024 * 1024 * 100)), // 100 MB
        )
        .route("/:thread_id/runs", post(create_run_and_execute))
        .route("/:thread_id/runs/:run_id", get(find_thread_run))
        .route("/:thread_id/runs", get(list_thread_runs))
        .with_state(Arc::clone(&state))
}
