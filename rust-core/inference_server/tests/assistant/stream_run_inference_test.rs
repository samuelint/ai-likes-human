use crate::test_utils::assistant_api::AssistantApiClient;
use app_core::assistant::domain::dto::{
    ApiCreateRunDto, ApiCreateThreadDto, ApiCreateThreadMessageDto, MessageContent, ThreadEvent,
};
use futures::{Stream, StreamExt};

static LLM_MODEL: &str = "openai:gpt-4o-mini";

async fn create_run_from_existing_thread_stream(
    prompt: &str,
) -> impl Stream<Item = Result<ThreadEvent, axum::Error>> {
    let client: AssistantApiClient = AssistantApiClient::new().await;

    let thread = client
        .create_thread(&ApiCreateThreadDto {
            messages: vec![ApiCreateThreadMessageDto {
                role: "user".to_string(),
                content: vec![MessageContent::text(prompt)],
                ..ApiCreateThreadMessageDto::default()
            }],
            ..ApiCreateThreadDto::default()
        })
        .await
        .0;

    client
        .stream_run(
            &thread.id,
            &ApiCreateRunDto {
                model: LLM_MODEL.to_string(),
                stream: Some(true),
                ..ApiCreateRunDto::default()
            },
        )
        .await
}

async fn stream_as_chunks_array(prompt: &str) -> Vec<ThreadEvent> {
    let mut stream = create_run_from_existing_thread_stream(prompt).await;

    let mut responses = Vec::new();
    while let Some(chunk) = stream.next().await {
        let response = chunk.unwrap();
        responses.push(response);
    }

    responses
}

// Single Step Sequence

#[tokio::test]
async fn test_stream_run_single_step_is_sequenced_by_events() {
    let chunks = stream_as_chunks_array("Tell me a joke.").await;

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
