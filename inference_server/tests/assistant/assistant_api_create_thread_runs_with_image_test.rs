use crate::{
    assets,
    test_utils::{assistant_api::AssistantApiClient, messages_builder::MessagesBuilder},
};

#[tokio::test]
async fn test_run_stream_with_image_url() {
    let client: AssistantApiClient = AssistantApiClient::new().await;
    let pig_image = assets::pig_base64();
    let messages = MessagesBuilder::new()
        .add_image_url(&pig_image)
        .add_text("What animal is in the image?")
        .build();
    let text_response = client.stream_new_thread_as_text(messages).await;

    assert!(
        text_response.contains("pig"),
        "text response should contain the word 'pig'"
    );
}
