use app_core::assistant::domain::dto::{
    message_delta::ThreadMessageDeltaDto, ApiCreateRunDto, ThreadEvent, ThreadEventDto,
};

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
