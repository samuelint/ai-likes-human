use std::sync::Arc;

use app_core::ApiFacade;

#[derive(Clone)]
pub struct ServerState {
    pub core_container: Arc<app_core::AppContainer>,
    pub api: Arc<ApiFacade>,
}
