use app_core::assistant::domain::dto::ThreadEvent;

use crate::test_utils::assistant_api::AssistantApiClient;

#[tokio::test]
async fn test_run_stream_starts_with_thread_created() {
    let client: AssistantApiClient = AssistantApiClient::new().await;
    let chunks = client
        .stream_new_thread_with_prompt_as_chunks_vec("Tell me a joke.")
        .await;
    let chunk = &chunks[0];

    let chunk = match chunk {
        ThreadEvent::ThreadCreated(event) => event,
        _ => panic!("Expected ThreadCreated event"),
    };

    assert_eq!(chunk.event, "thread.created");
}

#[tokio::test]
async fn test_run_stream_thread_created_contains_new_thread_data() {
    let client: AssistantApiClient = AssistantApiClient::new().await;
    let chunks = client
        .stream_new_thread_with_prompt_as_chunks_vec("Tell me a joke.")
        .await;
    let chunk = &chunks[0];

    let chunk = match chunk {
        ThreadEvent::ThreadCreated(event) => event,
        _ => panic!("Expected ThreadCreated event"),
    };

    assert!(chunk.data.id.len() > 0, "Should have a thread id");
    assert!(chunk.data.created_at > 0, "Should have a created at date");
}

#[tokio::test]
async fn test_run_stream_second_chunk_is_thread_run_created() {
    let client: AssistantApiClient = AssistantApiClient::new().await;
    let chunks = client
        .stream_new_thread_with_prompt_as_chunks_vec("Tell me a joke.")
        .await;
    let chunk = &chunks[1];

    let chunk = match chunk {
        ThreadEvent::ThreadRunCreated(event) => event,
        _ => panic!("Expected ThreadRunCreated event"),
    };

    assert_eq!(chunk.event, "thread.run.created");
}
