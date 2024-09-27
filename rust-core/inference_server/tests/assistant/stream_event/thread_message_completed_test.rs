use app_core::assistant::domain::dto::{ApiCreateRunDto, ThreadEvent};

use crate::test_utils::assistant_api::AssistantApiClient;

#[tokio::test]
async fn test_message_completed_event_status_change_to_completed() {
    let client: AssistantApiClient = AssistantApiClient::new().await;
    let thread = client.create_thread_with_prompt("Tell me a joke.").await;
    let chunks = client
        .stream_run_as_chunks_array(&thread.id, &ApiCreateRunDto::default())
        .await;

    let event = chunks
        .iter()
        .find_map(|chunk| match chunk {
            ThreadEvent::ThreadMessageCompleted(event) => Some(event),
            _ => None,
        })
        .expect("ThreadMessageCompleted should be found");

    assert_eq!(event.data.status, "completed");
}
