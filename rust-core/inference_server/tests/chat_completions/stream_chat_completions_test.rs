use crate::test_utils;

use futures::stream::StreamExt;
use serial_test::serial;
use test_utils::{with_default_started_server, OpenaiClient};

#[tokio::test]
#[serial]
async fn test_stream_chat_completions_result() {
    with_default_started_server(|server_url| async move {
        let client = OpenaiClient::new_with_endpoint(server_url);
        let mut stream = client
            .user_stream(
                "openai:gpt-4o-mini",
                "Write a simple advanced usage of Rust in one sentence",
            )
            .await;

        let mut responses = Vec::new();
        while let Some(response) = stream.next().await {
            responses.push(response);
        }
        let response: String = responses.join(" - ");
        assert!(!response.is_empty());
    })
    .await;
}
