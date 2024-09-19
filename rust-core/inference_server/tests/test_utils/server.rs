use std::future::IntoFuture;

use hyper::StatusCode;
use inference_server::{serve, ServeParameters};

pub async fn with_started_server<F, Fut>(parameters: ServeParameters, f: F)
where
    F: Fn(String) -> Fut,
    Fut: IntoFuture<Output = ()>,
{
    let port = parameters.port;
    let server = tokio::spawn(async {
        serve(parameters).await;
    });

    wait_for_server_to_be_up_for(&format!("http://localhost:{}", port), 5).await;

    let server_url = format!("http://localhost:{}/openai/v1", port);
    let f = f(server_url);

    tokio::time::timeout(std::time::Duration::from_secs(15), f)
        .await
        .unwrap();

    server.abort();
}

pub async fn is_server_up(server_url: &str) -> bool {
    let response = reqwest::get(server_url).await;
    match response {
        Ok(response) => response.status() == StatusCode::OK,
        Err(_) => false,
    }
}

pub async fn wait_for_server_to_be_up_for(server_url: &str, duration_sec: u64) {
    let start = std::time::Instant::now();
    while !is_server_up(&server_url).await && start.elapsed().as_secs() < duration_sec {
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
}

#[allow(dead_code)]
pub async fn with_default_started_server<F, Fut>(f: F)
where
    F: Fn(String) -> Fut,
    Fut: IntoFuture<Output = ()>,
{
    with_started_server(
        ServeParameters {
            use_trace: false,
            ..ServeParameters::default()
        },
        f,
    )
    .await;
}
