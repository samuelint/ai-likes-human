#[cfg(test)]
mod tests_thread_message_dto {
    use crate::assistant::domain::dto::{ApiMessageContent, ThreadMessageDto};

    #[test]
    fn test_text_content_are_all_join_in_a_string() {
        let message = ThreadMessageDto {
            content: vec![
                ApiMessageContent::text("Hello"),
                ApiMessageContent::text(" World"),
                ApiMessageContent::text("!"),
            ],
            ..ThreadMessageDto::default()
        };

        let result = message.to_string_content();

        assert_eq!(result, "Hello World!");
    }
}

mod test_create_thread_message_dto {
    use claim::assert_ok;

    use crate::assistant::domain::dto::{ApiCreateThreadMessageDto, ApiMessageContent};

    #[test]
    fn test_message_deserialization_with_string_text_content() {
        let json = r#"{
            "role": "user",
            "content": [
                {
                "type": "text",
                "text": "Salut"
                }
            ]
            }"#;

        let dto = serde_json::from_str::<ApiCreateThreadMessageDto>(json);

        assert_ok!(&dto);

        let dto = dto.unwrap();

        assert_eq!(dto.role, "user");

        let content1 = match &dto.content[0] {
            ApiMessageContent::Text { text } => text.to_string(),
            _ => unreachable!(),
        };

        assert_eq!(content1, "Salut");
    }

    #[test]
    fn test_message_deserialization_with_array_text_content() {
        let json = r#"{
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": {
                        "value": "Hi",
                        "annotations": []
                    }
                }
            ]
            }"#;

        let dto = serde_json::from_str::<ApiCreateThreadMessageDto>(json);

        assert_ok!(&dto);
        let dto = dto.unwrap();
        assert_eq!(dto.role, "user");
        let content1 = match &dto.content[0] {
            ApiMessageContent::Text { text } => text.to_string(),
            _ => unreachable!(),
        };

        assert_eq!(content1, "Hi");
    }
}

mod test_to_db_message_content {
    use crate::assistant::domain::dto::{
        ApiMessageContent, DbMessageContent, ImageUrl, ImageUrlContent, TextContent,
    };

    #[test]
    fn test_text_content_is_same() {
        let message = ApiMessageContent::Text {
            text: TextContent::String("Hello World!".to_string()),
        };

        let result: DbMessageContent = (&message).into();

        let text = match result {
            DbMessageContent::Text { text } => text,
            _ => panic!("Expected Text"),
        };

        assert_eq!(text.value, "Hello World!");
    }

    #[test]
    fn test_annotated_text_content_is_same() {
        let message = ApiMessageContent::Text {
            text: TextContent::Annotated {
                value: "Hello World!".to_string(),
                annotations: vec![],
            },
        };

        let result: DbMessageContent = (&message).into();

        let text = match result {
            DbMessageContent::Text { text } => text,
            _ => panic!("Expected Text"),
        };

        assert_eq!(text.value, "Hello World!");
    }

    #[test]
    fn test_image_url_content_is_same() {
        let message = ApiMessageContent::ImageUrl {
            image_url: ImageUrlContent {
                image_url: ImageUrl::url("https://example.com/img"),
            },
        };

        let result: DbMessageContent = (&message).into();

        let image_url = match result {
            DbMessageContent::ImageUrl { image_url } => image_url.image_url,
            _ => panic!("Expected Image Url"),
        };

        assert_eq!(image_url.url, "https://example.com/img");
    }
}
