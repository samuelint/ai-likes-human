use crate::test_utils::assistant_api::AssistantApiClient;
use app_core::assistant::domain::dto::{
    message_delta::ThreadMessageDeltaDto, ApiCreateRunDto, ThreadEvent, ThreadEventDto,
};

static LLM_MODEL: &str = "openai:gpt-4o-mini";

// Single Step Sequence

#[tokio::test]
async fn test_stream_run_single_step_is_sequenced_by_events() {
    let client: AssistantApiClient = AssistantApiClient::new().await;
    let thread = client.create_thread_with_prompt("Tell me a joke.").await;
    let chunks = client
        .stream_run_as_chunks_array(
            &thread.id,
            &ApiCreateRunDto {
                model: LLM_MODEL.to_string(),
                ..ApiCreateRunDto::default()
            },
        )
        .await;

    assert!(
        matches!(&chunks[0], ThreadEvent::ThreadRunCreated(_)),
        "Events does not have ThreadRunCreated"
    );
    assert!(
        matches!(&chunks[1], ThreadEvent::ThreadRunQueued(_)),
        "Events does not have ThreadRunQueued"
    );
    assert!(
        matches!(&chunks[2], ThreadEvent::ThreadRunInProgress(_)),
        "Events does not have ThreadRunInProgress"
    );
    assert!(
        matches!(&chunks[3], ThreadEvent::ThreadRunStepCreated(_)),
        "Events does not have ThreadRunStepCreated"
    );
    assert!(
        matches!(&chunks[3], ThreadEvent::ThreadRunStepCreated(_)),
        "Events does not have ThreadRunStepCreated"
    );
    assert!(
        matches!(&chunks[4], ThreadEvent::ThreadRunStepInProgress(_)),
        "Events does not have ThreadRunStepInProgress"
    );
    assert!(
        matches!(&chunks[5], ThreadEvent::ThreadMessageCreated(_)),
        "Events does not have ThreadMessageCreated"
    );
    assert!(
        matches!(&chunks[6], ThreadEvent::ThreadMessageInProgress(_)),
        "Events does not have ThreadMessageInProgress"
    );
    // Deltas
    for i in 7..(chunks.len() - 4) {
        let event = &chunks[i];
        assert!(
            matches!(event, ThreadEvent::ThreadMessageDelta(_)),
            "Events[{}] does not have ThreadMessageDelta",
            i
        );
    }
    // Deltas - END

    let last_item = chunks.get(chunks.len() - 3).unwrap();
    assert!(
        matches!(&last_item, ThreadEvent::ThreadMessageCompleted(_)),
        "Events does not end with ThreadMessageCompleted"
    );

    let last_item = chunks.get(chunks.len() - 2).unwrap();
    assert!(
        matches!(&last_item, ThreadEvent::ThreadRunStepCompleted(_)),
        "Events does not end with ThreadRunStepCompleted"
    );

    let last_chunk = chunks.last().unwrap();
    assert!(
        matches!(&last_chunk, ThreadEvent::ThreadRunCompleted(_)),
        "Events does not end with Done"
    );
}

#[tokio::test]
async fn test_message_delta_events() {
    let client: AssistantApiClient = AssistantApiClient::new().await;
    let thread = client.create_thread_with_prompt("Tell me a joke.").await;
    let chunks = client
        .stream_run_as_chunks_array(
            &thread.id,
            &ApiCreateRunDto {
                model: LLM_MODEL.to_string(),
                ..ApiCreateRunDto::default()
            },
        )
        .await;

    let delta_chunks: Vec<ThreadEventDto<ThreadMessageDeltaDto>> = chunks
        .iter()
        .filter_map(|c| match c {
            ThreadEvent::ThreadMessageDelta(delta) => Some(delta.clone()),
            _ => None,
        })
        .collect();

    for (i, chunk) in delta_chunks.iter().enumerate() {
        assert_eq!(
            chunk.event, "thread.message.delta",
            "delta_chunks[{}] does not have thread.message.delta event",
            i
        );
    }
}

#[tokio::test]
async fn test_thread_run_completed_event_change_run_status() {
    let client: AssistantApiClient = AssistantApiClient::new().await;
    let thread = client.create_thread_with_prompt("Tell me a joke.").await;
    let chunks = client
        .stream_run_as_chunks_array(
            &thread.id,
            &ApiCreateRunDto {
                model: LLM_MODEL.to_string(),
                ..ApiCreateRunDto::default()
            },
        )
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
        .stream_run_as_chunks_array(
            &thread.id,
            &ApiCreateRunDto {
                model: LLM_MODEL.to_string(),
                ..ApiCreateRunDto::default()
            },
        )
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
