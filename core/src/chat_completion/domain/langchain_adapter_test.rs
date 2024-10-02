#[cfg(test)]
mod openai_message_tests {
    use crate::chat_completion::{ApiMessageContent, ChatCompletionMessageDto, ImageUrl};
    use langchain_rust::schemas::MessageType;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_image_url_and_text_content_are_contained_langchain_message() {
        let message = ChatCompletionMessageDto {
            role: "assistant".to_string(),
            content: vec![
                ApiMessageContent::image_url("data:image/jpeg;base64,23434234234234"),
                ApiMessageContent::text("What is in this image?"),
            ],
            ..ChatCompletionMessageDto::default()
        };

        let result: langchain_rust::schemas::Message = message.into();

        let images = result.images.unwrap();

        assert_eq!(result.content, "What is in this image?");
        assert_eq!(images[0].image_url, "data:image/jpeg;base64,23434234234234");
    }

    #[test]
    fn test_image_url_content_is_contained_langchain_message() {
        let message = ChatCompletionMessageDto {
            role: "assistant".to_string(),
            content: vec![ApiMessageContent::image_url(
                "data:image/jpeg;base64,23434234234234",
            )],
            ..ChatCompletionMessageDto::default()
        };

        let result: langchain_rust::schemas::Message = message.into();

        let images = result.images.unwrap();

        assert_eq!(images.len(), 1);
        assert_eq!(images[0].image_url, "data:image/jpeg;base64,23434234234234");
    }

    #[test]
    fn test_image_url_details_is_contained_langchain_message() {
        let message = ChatCompletionMessageDto {
            role: "assistant".to_string(),
            content: vec![ApiMessageContent::ImageUrl {
                image_url: ImageUrl {
                    url: "data:image/jpeg;base64,23434234234234".to_string(),
                    details: Some("high".to_string()),
                },
            }],
            ..ChatCompletionMessageDto::default()
        };

        let result: langchain_rust::schemas::Message = message.into();

        let images = result.images.unwrap();

        assert_eq!(images.len(), 1);
        assert_eq!(images[0].detail.clone().unwrap(), "high");
    }

    #[test]
    fn test_multiple_text_content_are_join_in_single_content() {
        let message = ChatCompletionMessageDto {
            role: "assistant".to_string(),
            content: vec![
                ApiMessageContent::text("hello"),
                ApiMessageContent::text(" "),
                ApiMessageContent::text("world"),
            ],
            ..ChatCompletionMessageDto::default()
        };

        let result: langchain_rust::schemas::Message = message.into();

        assert_eq!(result.content, "hello world");
    }

    #[test]
    fn test_text_content_is_contained_langchain_message() {
        let result: langchain_rust::schemas::Message =
            ChatCompletionMessageDto::assistant("Hello").into();

        assert_eq!(result.content, "Hello");
    }

    #[test]
    fn test_message_type_is_contained_in_langchain_message() {
        let message = ChatCompletionMessageDto {
            role: "assistant".to_string(),
            ..ChatCompletionMessageDto::default()
        };

        let result: langchain_rust::schemas::Message = message.into();

        assert_eq!(result.message_type, MessageType::AIMessage);
    }
}
