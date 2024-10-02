use crate::test_utils::assistant_api::AssistantApiClient;
use app_core::{
    assistant::domain::dto::{ApiCreateThreadDto, ApiCreateThreadMessageDto},
    chat_completion::{ApiMessageContent, ApiTextContent},
};

#[tokio::test]
async fn test_message_with_string_text_is_fetch_annotated() {
    let client = AssistantApiClient::new().await;

    let (thread, _) = client.create_thread(&ApiCreateThreadDto::default()).await;
    let (message, _) = client
        .create_message(
            &thread.id,
            &ApiCreateThreadMessageDto {
                content: vec![ApiMessageContent::Text {
                    text: ApiTextContent::String("Say Hello!".to_string()),
                }],
                ..ApiCreateThreadMessageDto::user()
            },
        )
        .await;
    let (fetched_message, _) = client.get_thread_message(&thread.id, &message.id).await;
    let fetched_message = fetched_message.unwrap();
    let content = &fetched_message.content[0];

    let value = match content {
        ApiMessageContent::Text { text, .. } => match text {
            ApiTextContent::Annotated { value, .. } => value,
            _ => panic!("Expected Text Annotated"),
        },
        _ => panic!("Expected Text MessageContent"),
    };

    assert_eq!(value, "Say Hello!");
}
