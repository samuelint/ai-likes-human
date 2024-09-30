use std::sync::Arc;

use app_core::AppContainer;
use inference_server::{create_router, CreateRouterParameters};

#[allow(dead_code)]
pub async fn create_test_router() -> axum::Router {
    let container = AppContainer::new_in_memory().await.unwrap();
    create_router(Arc::new(container), CreateRouterParameters::default())
}
