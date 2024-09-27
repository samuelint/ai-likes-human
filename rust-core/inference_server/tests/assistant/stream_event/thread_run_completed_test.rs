use app_core::assistant::domain::dto::{ApiCreateRunDto, ThreadEvent};

use crate::test_utils::assistant_api::AssistantApiClient;

#[tokio::test]
async fn test_run_stream_ends_with_thread_run_completed() {
    let client: AssistantApiClient = AssistantApiClient::new().await;
    let chunks = client
        .stream_new_thread_with_prompt_as_chunks_vec("Tell me a joke.")
        .await;
    let chunk = chunks.last().unwrap();

    let chunk = match chunk {
        ThreadEvent::ThreadRunCompleted(event) => event,
        _ => panic!("Expected ThreadRunCompleted event"),
    };

    assert_eq!(chunk.event, "thread.run.completed");
}

#[tokio::test]
async fn test_run_stream_ends_with_thread_run_completed_run_id_is_the_same_as_run_created_event() {
    let client: AssistantApiClient = AssistantApiClient::new().await;
    let chunks = client
        .stream_new_thread_with_prompt_as_chunks_vec("Tell me a joke.")
        .await;
    let thread_run_created_chunk = match chunks
        .iter()
        .find(|&chunk| matches!(chunk, ThreadEvent::ThreadRunCreated(_)))
        .cloned()
        .expect("ThreadRunCompleted should be found")
    {
        ThreadEvent::ThreadRunCreated(event) => event,
        _ => panic!("Expected ThreadRunCompleted event"),
    };

    let chunk = chunks.last().unwrap();
    let chunk = match chunk {
        ThreadEvent::ThreadRunCompleted(event) => event,
        _ => panic!("Expected ThreadRunCompleted event"),
    };

    assert_eq!(chunk.data.id, thread_run_created_chunk.data.id);
}

#[tokio::test]
async fn test_thread_run_completed_event_change_run_status() {
    let client: AssistantApiClient = AssistantApiClient::new().await;
    let thread = client.create_thread_with_prompt("Tell me a joke.").await;
    let chunks = client
        .stream_run_as_chunks_array(&thread.id, &ApiCreateRunDto::default())
        .await;

    let run_completed_event = chunks
        .iter()
        .find_map(|chunk| match chunk {
            ThreadEvent::ThreadRunCompleted(event) => Some(event),
            _ => None,
        })
        .expect("ThreadRunCompleted should be found");

    assert_eq!(run_completed_event.data.status, "completed");
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
        .find_map(|chunk| match chunk {
            ThreadEvent::ThreadRunCompleted(event) => Some(event),
            _ => None,
        })
        .expect("ThreadRunCompleted should be found");
    let run_id = &run_completed_event.data.id;

    let (run, _status) = client.get_run(&thread.id, &run_id).await;
    assert_eq!(run.status, "completed");
}
