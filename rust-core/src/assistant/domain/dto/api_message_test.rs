#[cfg(test)]
mod tests_thread_message_dto {
    use crate::assistant::domain::dto::{MessageContent, ThreadMessageDto};

    #[test]
    fn test_text_content_are_all_join_in_a_string() {
        let message = ThreadMessageDto {
            content: vec![
                MessageContent::text("Hello"),
                MessageContent::text(" World"),
                MessageContent::text("!"),
            ],
            ..ThreadMessageDto::default()
        };

        let result = message.to_string_content();

        assert_eq!(result, "Hello World!");
    }
}

mod test_create_thread_message_dto {
    use claim::assert_ok;

    use crate::assistant::domain::dto::{ApiCreateThreadMessageDto, MessageContent};

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
            MessageContent::Text { text } => text.to_string(),
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
            MessageContent::Text { text } => text.to_string(),
            _ => unreachable!(),
        };

        assert_eq!(content1, "Hi");
    }
}
