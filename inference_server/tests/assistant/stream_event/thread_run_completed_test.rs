use app_core::assistant::domain::dto::{ApiCreateRunDto, ThreadEventData};
use pretty_assertions::assert_eq;

use crate::test_utils::{assistant_api::AssistantApiClient, messages_builder::MessagesBuilder};

#[tokio::test]
async fn test_run_stream_ends_with_thread_run_completed_run_id_is_the_same_as_run_created_event() {
    let client: AssistantApiClient = AssistantApiClient::new().await;
    let chunks = client
        .stream_new_thread_as_chunks_vec(MessagesBuilder::from_text("Tell me a joke.").into())
        .await;
    let run_created_data = chunks
        .iter()
        .find_map(|chunk| match &chunk.1 {
            ThreadEventData::ThreadRun(e) => Some(e),
            _ => None,
        })
        .expect("ThreadRunCompleted should be found");

    let chunk = chunks.last().unwrap();
    let run_completed_data = match &chunk.1 {
        ThreadEventData::ThreadRun(event) => event,
        _ => panic!("Expected ThreadRunCompleted event"),
    };

    assert_eq!(run_created_data.id, run_completed_data.id);
}

#[tokio::test]
async fn test_thread_run_completed_event_change_run_status() {
    let client: AssistantApiClient = AssistantApiClient::new().await;
    let thread = client.create_thread_with_prompt("Tell me a joke.").await;
    let chunks = client
        .stream_run_as_chunks_array(&thread.id, &ApiCreateRunDto::default())
        .await;

    let run_completed_data = chunks
        .iter()
        .filter_map(|chunk| match chunk.0.as_str() {
            "thread.run.completed" => Some(chunk),
            _ => None,
        })
        .find_map(|chunk| match &chunk.1 {
            ThreadEventData::ThreadRun(event) => Some(event),
            _ => None,
        })
        .expect("ThreadRunCompleted should be found");

    assert_eq!(run_completed_data.status, "completed");
}

#[tokio::test]
async fn test_final_thread_run_status_is_completed() {
    let client: AssistantApiClient = AssistantApiClient::new().await;
    let thread = client.create_thread_with_prompt("Tell me a joke.").await;
    let chunks = client
        .stream_run_as_chunks_array(&thread.id, &ApiCreateRunDto::default())
        .await;

    let run_completed_event = chunks
        .iter()
        .filter_map(|chunk| match chunk.0.as_str() {
            "thread.run.completed" => Some(chunk),
            _ => None,
        })
        .find_map(|chunk| match &chunk.1 {
            ThreadEventData::ThreadRun(event) => Some(event),
            _ => None,
        })
        .expect("ThreadRunCompleted should be found");
    let run_id = &run_completed_event.id;

    let (run, _status) = client.get_run(&thread.id, &run_id).await;
    assert_eq!(run.status, "completed");
}
