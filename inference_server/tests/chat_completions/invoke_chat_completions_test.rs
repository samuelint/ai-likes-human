use crate::{
    assets,
    test_utils::chat_completion_api::{ChatCompletionApiClient, DEFAULT_LLM_MODEL},
};
use app_core::chat_completion::{ApiMessageContent, ChatCompletionMessageDto};
use inference_server::ApiChatCompletionRequestDto;

#[tokio::test]
async fn test_chat_completions_result() {
    let client = ChatCompletionApiClient::new().await;
    let response = client
        .invoke(&ApiChatCompletionRequestDto {
            model: DEFAULT_LLM_MODEL.to_string(),
            messages: vec![ChatCompletionMessageDto::user("Say hi!")],
            ..ApiChatCompletionRequestDto::default()
        })
        .await;

    let r_content = response.choices[0]
        .message
        .clone()
        .unwrap()
        .to_string_content();
    assert!(!r_content.is_empty(), "content should not be empty");
}

#[tokio::test]
async fn test_chat_completions_result_contains_model() {
    let client = ChatCompletionApiClient::new().await;
    let response = client
        .invoke(&ApiChatCompletionRequestDto {
            model: DEFAULT_LLM_MODEL.to_string(),
            messages: vec![ChatCompletionMessageDto::user("Say hi!")],
            ..ApiChatCompletionRequestDto::default()
        })
        .await;

    assert_eq!(response.model, "openai:gpt-4o-mini");
}

#[tokio::test]
async fn test_chat_completions_result_object_is_chat_completion() {
    let client = ChatCompletionApiClient::new().await;
    let response = client
        .invoke(&ApiChatCompletionRequestDto {
            model: DEFAULT_LLM_MODEL.to_string(),
            messages: vec![ChatCompletionMessageDto::user("Say hi!")],
            ..ApiChatCompletionRequestDto::default()
        })
        .await;

    assert_eq!(response.object, "chat.completion");
}

#[tokio::test]
async fn test_chat_completions_with_image() {
    let client = ChatCompletionApiClient::new().await;
    let pig_image = assets::pig_base64();

    let response = client
        .invoke(&ApiChatCompletionRequestDto {
            model: DEFAULT_LLM_MODEL.to_string(),
            messages: vec![ChatCompletionMessageDto {
                role: "user".to_string(),
                content: vec![
                    ApiMessageContent::image_url(&pig_image),
                    ApiMessageContent::text("What is the image?"),
                ],
            }],
            ..ApiChatCompletionRequestDto::default()
        })
        .await;

    let text_response = response.choices[0]
        .message
        .clone()
        .unwrap()
        .to_string_content();

    assert!(
        text_response.contains("pig"),
        "text response should contain the word 'pig'"
    );
}
