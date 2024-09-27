use app_core::assistant::domain::dto::ApiCreateRunDto;

use crate::test_utils::assistant_api::AssistantApiClient;
use pretty_assertions::assert_eq;

#[tokio::test]
async fn test_thread_messages_after_a_run_are_listed() {
    let client: AssistantApiClient = AssistantApiClient::new().await;
    let thread = client.create_thread_with_prompt("Tell me a joke.").await;
    let _ = client
        .stream_run_as_chunks_array(&thread.id, &ApiCreateRunDto::default())
        .await;

    let messages = client.list_thread_messages(&thread.id).await.0;

    assert_eq!(
        messages.len(),
        2,
        "Should have 2 messages. User and Assistant"
    );
}

#[tokio::test]
async fn test_thread_run_after_a_run_is_listed() {
    let client: AssistantApiClient = AssistantApiClient::new().await;
    let thread = client.create_thread_with_prompt("Tell me a joke.").await;
    let _ = client
        .stream_run_as_chunks_array(&thread.id, &ApiCreateRunDto::default())
        .await;

    let runs = client.list_thread_runs(&thread.id).await.0;

    assert_eq!(runs.len(), 1, "Should have a run.");
}
