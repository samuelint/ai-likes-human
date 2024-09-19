use std::net::SocketAddr;
use std::sync::Arc;

use tokio::net::TcpListener;

use crate::route::{default_invoke, default_stream};
use crate::router_factory::{create_router, CreateRouterParameters};
use crate::trace::configure_tracing;
use crate::{InvokeFn, StreamFn};

pub struct ServeParameters {
    pub port: u16,
    pub invoke_fn: Arc<InvokeFn>,
    pub stream_fn: Arc<StreamFn>,
    pub use_trace: bool, // Traces must be disabled for integration tests. https://github.com/tokio-rs/console/issues/505#issuecomment-1935805256
}

impl Default for ServeParameters {
    fn default() -> Self {
        ServeParameters {
            port: 1234,
            invoke_fn: Arc::new(default_invoke),
            stream_fn: Arc::new(default_stream),
            use_trace: false,
        }
    }
}

pub async fn serve(parameters: ServeParameters) {
    if parameters.use_trace {
        configure_tracing();
    }

    let router = create_router(CreateRouterParameters {
        use_trace: parameters.use_trace,
        invoke_fn: parameters.invoke_fn,
        stream_fn: parameters.stream_fn,
        ..CreateRouterParameters::default()
    });

    let addr = SocketAddr::from(([127, 0, 0, 1], parameters.port));
    let listener = TcpListener::bind(addr).await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
}
