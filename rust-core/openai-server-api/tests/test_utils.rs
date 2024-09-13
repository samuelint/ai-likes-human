use std::future::IntoFuture;

use openai_server_api::{serve, ServeParameters};

pub async fn with_started_server<F>(f: F)
where
    F: IntoFuture,
{
    let server = tokio::spawn(async move {
        serve(ServeParameters { port: 1234 }).await;
    });

    tokio::time::timeout(std::time::Duration::from_secs(10), f)
        .await
        .unwrap();

    server.abort();
}
