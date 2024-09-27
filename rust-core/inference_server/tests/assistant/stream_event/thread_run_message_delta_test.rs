use app_core::assistant::domain::dto::ApiCreateRunDto;

use crate::test_utils::assistant_api::AssistantApiClient;

#[tokio::test]
async fn test_message_delta_events() {
    let client: AssistantApiClient = AssistantApiClient::new().await;
    let thread = client.create_thread_with_prompt("Tell me a joke.").await;
    let chunks = client
        .stream_run_as_chunks_array(
            &thread.id,
            &ApiCreateRunDto {
                ..ApiCreateRunDto::default()
            },
        )
        .await;

    let deltas_data: Vec<_> = chunks
        .iter()
        .filter(|chunk| match chunk.0.as_str() {
            "thread.message.delta" => true,
            _ => false,
        })
        .collect();

    assert!(
        deltas_data.len() > 0,
        "Should have thread.message.delta events"
    );
}
