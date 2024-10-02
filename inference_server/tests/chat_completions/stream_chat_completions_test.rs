use crate::test_utils::chat_completion_api::{ChatCompletionApiClient, DEFAULT_LLM_MODEL};
use app_core::chat_completion::ChatCompletionMessageDto;
use inference_server::ApiChatCompletionRequestDto;
use itertools::Itertools;

#[tokio::test]
async fn test_stream_chat_completions_result() {
    let client = ChatCompletionApiClient::new().await;
    let events = client
        .stream_as_events_vec(&ApiChatCompletionRequestDto {
            model: DEFAULT_LLM_MODEL.to_string(),
            messages: vec![ChatCompletionMessageDto::user("Say hi!")],
            ..ApiChatCompletionRequestDto::default()
        })
        .await;

    let text_response = events.iter().map(|e| e.to_content_string()).join("");

    assert!(!text_response.is_empty());
}
