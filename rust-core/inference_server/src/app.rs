use std::net::SocketAddr;
use std::sync::Arc;

use app_core::{CoreConfiguration, AppContainer};
use tokio::net::TcpListener;

use crate::router::{create_router, CreateRouterParameters};
use crate::trace::configure_tracing;

pub struct ServeParameters {
    pub port: u16,
    pub use_trace: bool, // Traces must be disabled for integration tests. https://github.com/tokio-rs/console/issues/505#issuecomment-1935805256
    pub database_url: String,
}

impl Default for ServeParameters {
    fn default() -> Self {
        ServeParameters {
            port: 1234,
            use_trace: false,
            database_url: String::from("sqlite::memory:"),
        }
    }
}

pub async fn serve(parameters: ServeParameters) {
    if parameters.use_trace {
        configure_tracing();
    }
    let config = CoreConfiguration {
        database_url: parameters.database_url,
        ..CoreConfiguration::default()
    };
    let container = AppContainer::new(config).await.unwrap();

    let router = create_router(
        Arc::new(container),
        CreateRouterParameters {
            use_trace: parameters.use_trace,
            ..CreateRouterParameters::default()
        },
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], parameters.port));
    let listener = TcpListener::bind(addr).await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
}
