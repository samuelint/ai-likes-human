use app_core::assistant::domain::dto::ThreadEvent;
use pretty_assertions::assert_eq;

use crate::test_utils::assistant_api::AssistantApiClient;

#[tokio::test]
async fn test_run_step_completed_status_is_completed() {
    let client: AssistantApiClient = AssistantApiClient::new().await;
    let chunks = client
        .stream_new_thread_with_prompt_as_chunks_vec("Tell me a joke.")
        .await;

    let event = chunks
        .iter()
        .find_map(|chunk| match chunk {
            ThreadEvent::ThreadRunStepCompleted(event) => Some(event),
            _ => None,
        })
        .expect("ThreadRunStepCompleted should be found");

    assert_eq!(event.data.status, "completed");
}
