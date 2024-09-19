use std::{future::IntoFuture, sync::Arc};

use inference_server::{serve, InvokeFn, ServeParameters, StreamFn};

pub async fn with_started_server<F, Fut>(parameters: ServeParameters, f: F)
where
    F: Fn(String) -> Fut,
    Fut: IntoFuture<Output = ()>,
{
    let port = parameters.port;
    let server = tokio::spawn(async {
        serve(parameters).await;
    });

    let server_url = format!("http://localhost:{}/openai/v1", port);
    let f = f(server_url);

    tokio::time::timeout(std::time::Duration::from_secs(5), f)
        .await
        .unwrap();

    server.abort();
}

#[allow(dead_code)]
pub async fn with_invoke_fn_server<F, Fut>(invoke_fn: Arc<InvokeFn>, f: F)
where
    F: Fn(String) -> Fut,
    Fut: IntoFuture<Output = ()>,
{
    with_started_server(
        ServeParameters {
            invoke_fn: invoke_fn,
            use_trace: false,
            ..ServeParameters::default()
        },
        f,
    )
    .await;
}

#[allow(dead_code)]
pub async fn with_stream_fn_server<F, Fut>(stream_fn: Arc<StreamFn>, f: F)
where
    F: Fn(String) -> Fut,
    Fut: IntoFuture<Output = ()>,
{
    with_started_server(
        ServeParameters {
            stream_fn: stream_fn,
            use_trace: false,
            ..ServeParameters::default()
        },
        f,
    )
    .await;
}
