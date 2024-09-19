use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};

use crate::{
    api::health::health,
    app_state::ServerState,
    openai_v1_router::create_openai_v1_router,
    route::{default_invoke, default_stream},
    trace::with_tracing,
    InvokeFn, StreamFn,
};

pub struct CreateRouterParameters {
    pub invoke_fn: Arc<InvokeFn>,
    pub stream_fn: Arc<StreamFn>,
    pub use_trace: bool,
}

impl Default for CreateRouterParameters {
    fn default() -> Self {
        CreateRouterParameters {
            invoke_fn: Arc::new(default_invoke),
            stream_fn: Arc::new(default_stream),
            use_trace: true,
        }
    }
}

#[allow(dead_code)]
pub fn create_router(parameters: CreateRouterParameters) -> Router {
    let state = Arc::new(ServerState {
        invoke_fn: parameters.invoke_fn,
        stream_fn: parameters.stream_fn,
    });

    let openai_router = create_openai_v1_router(state);

    let router = Router::new()
        .route("/", get(health))
        .route("/health", get(health))
        .nest("/openai/v1", openai_router)
        .fallback(fallback);

    if parameters.use_trace {
        with_tracing(router)
    } else {
        router
    }
}

async fn fallback() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "")
}
