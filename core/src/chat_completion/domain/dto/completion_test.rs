#[cfg(test)]
mod openai_message_tests {
    use crate::chat_completion::{domain::dto::ChatCompletionMessageDto, ApiMessageContent};

    #[test]
    fn test_create_assistant_message() {
        let result = ChatCompletionMessageDto::assistant("Hello");

        assert!(result.role == "assistant");
    }

    #[test]
    fn test_create_message_string_content() {
        let result = ChatCompletionMessageDto::assistant("Hello");

        assert!(result.to_string_content() == "Hello");
    }

    #[test]
    fn test_text_content_are_all_join_in_a_string() {
        let message = ChatCompletionMessageDto {
            content: vec![
                ApiMessageContent::text("Hello"),
                ApiMessageContent::text(" World"),
                ApiMessageContent::text("!"),
            ],
            ..ChatCompletionMessageDto::default()
        };

        let result = message.to_string_content();

        assert_eq!(result, "Hello World!");
    }
}

mod chat_completion_object_tests {
    use chrono::Utc;

    use crate::chat_completion::domain::dto::{ChatCompletionMessageDto, ChatCompletionObject};

    #[test]
    fn test_openaichat_completion_object() {
        let result = ChatCompletionObject::default();

        assert!(result.object == "chat.completion");
    }

    #[test]
    fn test_openaichat_completion_object_id_is_empty_string_by_default() {
        let result = ChatCompletionObject::default();

        assert!(result.id == "");
    }

    #[test]
    fn test_openaichat_completion_created_is_now() {
        let result = ChatCompletionObject::default();

        let now = Utc::now().timestamp_millis();
        let delta = (result.created - now).abs();
        assert!(delta < 100);
    }

    #[test]
    fn test_openaichat_completion_object_single_choice() {
        let message = ChatCompletionMessageDto::assistant("Hello");
        let result = ChatCompletionObject::new_single_choice(message, "");

        let choice1 = &result.choices[0];
        assert!(choice1.index == 0);
        assert!(choice1.message.is_some());
        assert!(choice1.message.as_ref().unwrap().to_string_content() == "Hello");
        assert!(choice1.message.as_ref().unwrap().role == "assistant");
    }

    #[test]
    fn test_openaichat_completion_object_single_choice_finish_reason_is_stop() {
        let message = ChatCompletionMessageDto::assistant("Hello");
        let result = ChatCompletionObject::new_single_choice(message, "");

        let choice1 = &result.choices[0];
        assert!(choice1.finish_reason.as_ref().unwrap() == "stop");
    }
}

mod openai_chat_completion_chunk_object {
    use chrono::Utc;

    use crate::chat_completion::domain::dto::ChatCompletionChunkObject;

    #[test]
    fn test_openai_chat_completion_chunk_object_id_is_none_by_default() {
        let result = ChatCompletionChunkObject::default();

        assert!(result.id == None);
    }

    #[test]
    fn test_openai_chat_completion_chunk_object_object() {
        let result = ChatCompletionChunkObject::default();

        assert!(result.object == "chat.completion.chunk");
    }

    #[test]
    fn test_openai_chat_completion_chunk_object_created_is_now() {
        let result = ChatCompletionChunkObject::default();

        let now = Utc::now().timestamp_millis();
        let delta = (result.created - now).abs();
        assert!(delta < 100);
    }
}
