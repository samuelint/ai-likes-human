use crate::test_utils;
use serial_test::serial;
use test_utils::{with_default_started_server, OpenaiClient};

#[tokio::test]
#[serial]
async fn test_chat_completions_result() {
    with_default_started_server(|server_url| async move {
        let client = OpenaiClient::new_with_endpoint(server_url);

        let result = client.user_invoke("openai:gpt-4o-mini", "Say hi!").await;

        let r_content = result.choices[0].message.content.as_slice();
        assert!(!r_content.is_empty(), "content should not be empty");
    })
    .await;
}

#[tokio::test]
#[serial]
async fn test_chat_completions_result_contains_model() {
    with_default_started_server(|server_url| async move {
        let client = OpenaiClient::new_with_endpoint(server_url);

        let result = client.user_invoke("openai:gpt-4o-mini", "Say hi!").await;

        let response_model = result.model;
        assert!(!response_model.is_empty(), "model should not be empty");
        assert_eq!(response_model, "openai:gpt-4o-mini");
    })
    .await;
}

#[tokio::test]
#[serial]
async fn test_chat_completions_result_object_is_chat_completion() {
    with_default_started_server(|server_url| async move {
        let client = OpenaiClient::new_with_endpoint(server_url);

        let result = client.user_invoke("openai:gpt-4o-mini", "Say hi!").await;

        let response_model = result.object;
        assert!(!response_model.is_empty(), "object should not be empty");
        assert_eq!(response_model, "chat.completion");
    })
    .await;
}
