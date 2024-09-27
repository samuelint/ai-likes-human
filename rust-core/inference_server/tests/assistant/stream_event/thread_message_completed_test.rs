use app_core::assistant::domain::dto::{ApiCreateRunDto, ThreadEventData};
use pretty_assertions::assert_eq;

use crate::test_utils::assistant_api::AssistantApiClient;

#[tokio::test]
async fn test_message_completed_event_status_change_to_completed() {
    let client: AssistantApiClient = AssistantApiClient::new().await;
    let thread = client.create_thread_with_prompt("Tell me a joke.").await;
    let chunks = client
        .stream_run_as_chunks_array(&thread.id, &ApiCreateRunDto::default())
        .await;

    let data = chunks
        .iter()
        .filter_map(|chunk| match chunk.0.as_str() {
            "thread.message.completed" => Some(chunk),
            _ => None,
        })
        .find_map(|chunk| match &chunk.1 {
            ThreadEventData::ThreadMessage(data) => Some(data),
            _ => None,
        })
        .expect("ThreadMessageCompleted should be found");

    assert_eq!(data.status, "completed");
}
