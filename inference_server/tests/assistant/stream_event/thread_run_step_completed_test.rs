use app_core::assistant::domain::dto::ThreadEventData;
use pretty_assertions::assert_eq;

use crate::test_utils::assistant_api::AssistantApiClient;

#[tokio::test]
async fn test_run_step_completed_status_is_completed() {
    let client: AssistantApiClient = AssistantApiClient::new().await;
    let chunks = client
        .stream_new_thread_with_prompt_as_chunks_vec("Tell me a joke.")
        .await;

    let event_data = chunks
        .iter()
        .filter_map(|chunk| match chunk.0.as_str() {
            "thread.run.step.completed" => Some(chunk),
            _ => None,
        })
        .find_map(|chunk| match &chunk.1 {
            ThreadEventData::ThreadRunStep(data) => Some(data),
            _ => None,
        })
        .expect("ThreadRunStepCompleted should be found");

    assert_eq!(event_data.status, "completed");
}
