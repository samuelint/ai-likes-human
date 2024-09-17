mod test_utils;

use serial_test::serial;
use std::sync::Arc;
use test_utils::{with_invoke_fn_server, OpenaiClient};

#[tokio::test]
#[serial]
async fn test_chat_completions_result() {
    with_invoke_fn_server(
        Arc::new(|_messages| Ok("It is something".to_string())),
        |server_url| async move {
            let client = OpenaiClient::new(server_url);

            let result = client.user_invoke("gpt-4o-mini", "What is bitcoin?").await;

            let r_content = result.choices[0].message.content.as_slice();
            assert!(!r_content.is_empty());
            assert!(r_content.contains(&String::from("It is something")));
        },
    )
    .await;
}

#[tokio::test]
#[serial]
async fn test_chat_completions_result_contains_model() {
    with_invoke_fn_server(
        Arc::new(|_messages| Ok("It is something".to_string())),
        |server_url| async move {
            let client = OpenaiClient::new(server_url);

            let result = client.user_invoke("gpt-4o-mini", "What is bitcoin?").await;

            let response_model = result.model;
            assert!(!response_model.is_empty());
            assert_eq!(response_model, "gpt-4o-mini");
        },
    )
    .await;
}

#[tokio::test]
#[serial]
async fn test_chat_completions_result_object_is_chat_completion() {
    with_invoke_fn_server(
        Arc::new(|_messages| Ok("It is something".to_string())),
        |server_url| async move {
            let client = OpenaiClient::new(server_url);

            let result = client.user_invoke("gpt-4o-mini", "What is bitcoin?").await;

            let response_model = result.object;
            assert!(!response_model.is_empty());
            assert_eq!(response_model, "chat.completion");
        },
    )
    .await;
}
