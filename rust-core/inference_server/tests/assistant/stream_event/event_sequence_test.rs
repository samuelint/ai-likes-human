use crate::test_utils::assistant_api::AssistantApiClient;
use app_core::assistant::domain::dto::{ApiCreateRunDto, ThreadEvent};

#[tokio::test]
async fn test_stream_new_thread_run_simple_sequence() {
    let client: AssistantApiClient = AssistantApiClient::new().await;
    let chunks = client
        .stream_new_thread_with_prompt_as_chunks_vec("Tell me a joke.")
        .await;

    assert!(
        matches!(&chunks[0], ThreadEvent::ThreadCreated(_)),
        "Events does not have ThreadCreated"
    );
    assert!(
        matches!(&chunks[1], ThreadEvent::ThreadRunCreated(_)),
        "Events does not have ThreadRunCreated"
    );
    assert!(
        matches!(&chunks[2], ThreadEvent::ThreadRunQueued(_)),
        "Events does not have ThreadRunQueued"
    );
    assert!(
        matches!(&chunks[3], ThreadEvent::ThreadRunInProgress(_)),
        "Events does not have ThreadRunInProgress"
    );
    assert!(
        matches!(&chunks[4], ThreadEvent::ThreadRunStepCreated(_)),
        "Events does not have ThreadRunStepCreated"
    );
    assert!(
        matches!(&chunks[4], ThreadEvent::ThreadRunStepCreated(_)),
        "Events does not have ThreadRunStepCreated"
    );
    assert!(
        matches!(&chunks[5], ThreadEvent::ThreadRunStepInProgress(_)),
        "Events does not have ThreadRunStepInProgress"
    );
    assert!(
        matches!(&chunks[6], ThreadEvent::ThreadMessageCreated(_)),
        "Events does not have ThreadMessageCreated"
    );
    assert!(
        matches!(&chunks[7], ThreadEvent::ThreadMessageInProgress(_)),
        "Events does not have ThreadMessageInProgress"
    );
    // Deltas
    for i in 8..(chunks.len() - 4) {
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
async fn test_stream_new_run_on_existign_thread_simple_sequence() {
    let client: AssistantApiClient = AssistantApiClient::new().await;
    let thread = client.create_thread_with_prompt("Tell me a joke.").await;
    let chunks = client
        .stream_run_as_chunks_array(&thread.id, &ApiCreateRunDto::default())
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
