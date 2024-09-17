mod test_utils;

use async_stream::try_stream;
use futures::stream::StreamExt;
use openai_server_api::StreamData;
use serial_test::serial;
use std::sync::Arc;
use test_utils::{with_stream_fn_server, OpenaiClient};

#[tokio::test]
#[serial]
async fn test_stream_chat_completions_result() {
    with_stream_fn_server(
        Arc::new(|_messages| {
            let stream = try_stream! {
                yield StreamData::new(
                    serde_json::json!({}),
                    "Hello world".to_string(),
                    );
            };

            Box::pin(stream)
        }),
        |server_url| async move {
            let client = OpenaiClient::new(server_url);
            let mut stream = client
                .user_stream(
                    "gpt-4o-mini",
                    "Write a simple advanced usage of Rust in one sentence",
                )
                .await;

            let mut responses = Vec::new();
            while let Some(response) = stream.next().await {
                responses.push(response);
            }
            let response: String = responses.join(" - ");
            assert!(!response.is_empty());
            assert!(response.contains(&String::from("Hello world")));
        },
    )
    .await;
}
