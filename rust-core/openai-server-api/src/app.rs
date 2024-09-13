use std::net::SocketAddr;

use tokio::net::TcpListener;

use crate::router_factory::create_router;
use crate::trace::configure_tracing;

pub struct ServeParameters {
    pub port: u16,
}
pub async fn serve(parameters: ServeParameters) {
    configure_tracing();
    let router = create_router();

    let addr = SocketAddr::from(([127, 0, 0, 1], parameters.port));
    let listener = TcpListener::bind(addr).await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router.to_owned()).await.unwrap();
}
