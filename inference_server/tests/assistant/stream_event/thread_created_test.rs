use app_core::assistant::domain::dto::ThreadEventData;

use crate::test_utils::assistant_api::AssistantApiClient;

#[tokio::test]
async fn test_run_stream_thread_created_contains_new_thread_data() {
    let client: AssistantApiClient = AssistantApiClient::new().await;
    let chunks = client
        .stream_new_thread_with_prompt_as_chunks_vec("Tell me a joke.")
        .await;
    let chunk = &chunks[0];

    let data = match &chunk.1 {
        ThreadEventData::Thread(event) => event,
        _ => panic!("Expected ThreadCreated event"),
    };

    assert!(data.id.len() > 0, "Should have a thread id");
    assert!(data.created_at > 0, "Should have a created at date");
}
