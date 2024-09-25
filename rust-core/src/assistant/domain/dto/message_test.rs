#[cfg(test)]
mod tests_thread_message_dto {
    use crate::assistant::domain::dto::{MessageContent, ThreadMessageDto};

    #[test]
    fn test_text_content_are_all_join_in_a_string() {
        let message = ThreadMessageDto {
            content: vec![
                MessageContent::new_text_content("Hello"),
                MessageContent::new_text_content(" World"),
                MessageContent::new_text_content("!"),
            ],
            ..ThreadMessageDto::default()
        };

        let result = message.to_string_content();

        assert_eq!(result, "Hello World!");
    }
}
