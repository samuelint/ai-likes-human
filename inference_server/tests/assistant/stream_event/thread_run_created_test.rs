use app_core::assistant::domain::dto::ThreadEventData;
use pretty_assertions::assert_eq;

use crate::test_utils::assistant_api::{AssistantApiClient, DEFAULT_LLM_MODEL};

#[tokio::test]
async fn test_run_stream_thread_run_created_chunk_contains_run_data() {
    let client: AssistantApiClient = AssistantApiClient::new().await;
    let chunks = client
        .stream_new_thread_with_prompt_as_chunks_vec("Tell me a joke.")
        .await;
    let chunk = &chunks[1];

    let chunk_data = match &chunk.1 {
        ThreadEventData::ThreadRun(event) => event,
        _ => panic!("Expected ThreadRunCreated event"),
    };

    assert!(chunk_data.id.len() > 0, "Should have a thread id");
    assert!(chunk_data.created_at > 0, "Should have a created at date");
}

#[tokio::test]
async fn test_run_stream_thread_run_created_chunk_contains_run_status_is_queued() {
    let client: AssistantApiClient = AssistantApiClient::new().await;
    let chunks = client
        .stream_new_thread_with_prompt_as_chunks_vec("Tell me a joke.")
        .await;
    let chunk = &chunks[1];

    let chunk_data = match &chunk.1 {
        ThreadEventData::ThreadRun(event) => event,
        _ => panic!("Expected ThreadRunCreated event"),
    };

    assert_eq!(chunk_data.status, "queued");
}

#[tokio::test]
async fn test_run_stream_thread_run_created_chunk_contains_llm_model() {
    let client: AssistantApiClient = AssistantApiClient::new().await;
    let chunks = client
        .stream_new_thread_with_prompt_as_chunks_vec("Tell me a joke.")
        .await;
    let chunk = &chunks[1];

    let chunk_data = match &chunk.1 {
        ThreadEventData::ThreadRun(event) => event,
        _ => panic!("Expected ThreadRunCreated event"),
    };

    assert_eq!(chunk_data.model, DEFAULT_LLM_MODEL);
}
