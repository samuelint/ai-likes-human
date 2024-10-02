use crate::test_utils::{assistant_api::AssistantApiClient, messages_builder::MessagesBuilder};
use app_core::assistant::domain::dto::ApiCreateRunDto;
use pretty_assertions::assert_eq;

#[tokio::test]
async fn test_stream_new_thread_run_simple_sequence() {
    let client: AssistantApiClient = AssistantApiClient::new().await;
    let chunks = client
        .stream_new_thread_as_chunks_vec(MessagesBuilder::from_text("Tell me a joke.").into())
        .await;

    assert_eq!(&chunks[0].0, "thread.created");
    assert_eq!(&chunks[1].0, "thread.run.created");
    assert_eq!(&chunks[2].0, "thread.run.queued");
    assert_eq!(&chunks[3].0, "thread.run.in_progress");
    assert_eq!(&chunks[4].0, "thread.run.step.created");
    assert_eq!(&chunks[5].0, "thread.run.step.in_progress");
    assert_eq!(&chunks[6].0, "thread.message.created");
    assert_eq!(&chunks[7].0, "thread.message.in_progress");

    // Deltas
    for i in 8..(chunks.len() - 4) {
        let event = &chunks[i];
        assert_eq!(event.0, "thread.message.delta");
    }
    // Deltas - END

    let before_before_last_item = chunks.get(chunks.len() - 3).unwrap();
    assert_eq!(&before_before_last_item.0, "thread.message.completed");

    let before_last_chunk = chunks.get(chunks.len() - 2).unwrap();
    assert_eq!(&before_last_chunk.0, "thread.run.step.completed");

    let last_chunk = chunks.get(chunks.len() - 1).unwrap();
    assert_eq!(&last_chunk.0, "thread.run.completed");
}

#[tokio::test]
async fn test_stream_new_run_on_existing_thread_simple_sequence() {
    let client: AssistantApiClient = AssistantApiClient::new().await;
    let thread = client.create_thread_with_prompt("Tell me a joke.").await;

    let chunks = client
        .stream_run_as_chunks_array(&thread.id, &ApiCreateRunDto::default())
        .await;

    assert_eq!(&chunks[0].0, "thread.run.created");
    assert_eq!(&chunks[1].0, "thread.run.queued");
    assert_eq!(&chunks[2].0, "thread.run.in_progress");
    assert_eq!(&chunks[3].0, "thread.run.step.created");
    assert_eq!(&chunks[4].0, "thread.run.step.in_progress");
    assert_eq!(&chunks[5].0, "thread.message.created");
    assert_eq!(&chunks[6].0, "thread.message.in_progress");

    // Deltas
    for i in 7..(chunks.len() - 4) {
        let event = &chunks[i];
        assert_eq!(event.0, "thread.message.delta");
    }
    // Deltas - END

    let before_before_last_item = chunks.get(chunks.len() - 3).unwrap();
    assert_eq!(&before_before_last_item.0, "thread.message.completed");

    let before_last_chunk = chunks.get(chunks.len() - 2).unwrap();
    assert_eq!(&before_last_chunk.0, "thread.run.step.completed");

    let last_chunk = chunks.get(chunks.len() - 1).unwrap();
    assert_eq!(&last_chunk.0, "thread.run.completed");
}

#[tokio::test]
async fn test_stream_on_thread_with_message_created_after_thread_creation() {
    let client: AssistantApiClient = AssistantApiClient::new().await;
    let (thread, _) = client.create_empty_thread().await;
    let _ = client
        .create_user_message_from_prompt(&thread.id, "Tell me a joke.")
        .await;

    let chunks = client
        .stream_run_as_chunks_array(&thread.id, &ApiCreateRunDto::default())
        .await;

    let last_chunk = chunks.get(chunks.len() - 1).unwrap();
    assert_eq!(&last_chunk.0, "thread.run.completed");
}
