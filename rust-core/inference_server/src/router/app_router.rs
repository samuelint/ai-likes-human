use std::sync::Arc;

use app_core::{ApiFacade, AppContainer};
use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use tower_http::cors::CorsLayer;

use crate::{
    app_state::ServerState,
    controller::health,
    route::{default_invoke, default_stream},
    trace::with_tracing,
    InvokeFn, StreamFn,
};

use super::openai_v1_router::create_openai_v1_router;

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
pub fn create_router(
    core_container: Arc<AppContainer>,
    parameters: CreateRouterParameters,
) -> Router {
    let state = Arc::new(ServerState {
        core_container: Arc::clone(&core_container),
        api: Arc::new(ApiFacade::new(Arc::clone(&core_container))),
    });

    let router = Router::new()
        .route("/", get(health))
        .route("/health", get(health))
        .nest("/openai/v1", create_openai_v1_router(state))
        .layer(CorsLayer::permissive())
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
