#[cfg(test)]
mod openai_message_tests {
    use crate::agent::domain::dto::chat_completion::ChatCompletionMessageDto;

    #[test]
    fn test_create_assistant_message() {
        let result = ChatCompletionMessageDto::new_assistant("Hello".to_string());

        assert!(result.role == "assistant");
    }

    #[test]
    fn test_create_message_string_content() {
        let result = ChatCompletionMessageDto::new_assistant("Hello".to_string());

        assert!(result.content == "Hello");
    }

    #[test]
    fn test_conversion_to_langchain_message_have_content() {
        let result: langchain_rust::schemas::Message =
            ChatCompletionMessageDto::new_assistant("Hello".to_string()).into();

        assert!(result.content == "Hello");
    }
}

mod chat_completion_object_tests {
    use chrono::Utc;

    use crate::agent::domain::dto::chat_completion::{
        ChatCompletionMessageDto, ChatCompletionObject,
    };

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
        let message = ChatCompletionMessageDto::new_assistant("Hello".to_string());
        let result = ChatCompletionObject::new_single_choice(message, "".to_string());

        let choice1 = &result.choices[0];
        assert!(choice1.index == 0);
        assert!(choice1.message.is_some());
        assert!(choice1.message.as_ref().unwrap().content == "Hello");
        assert!(choice1.message.as_ref().unwrap().role == "assistant");
    }

    #[test]
    fn test_openaichat_completion_object_single_choice_finish_reason_is_stop() {
        let message = ChatCompletionMessageDto::new_assistant("Hello".to_string());
        let result = ChatCompletionObject::new_single_choice(message, "".to_string());

        let choice1 = &result.choices[0];
        assert!(choice1.finish_reason.as_ref().unwrap() == "stop");
    }
}

mod openai_chat_completion_chunk_object {
    use chrono::Utc;

    use crate::agent::domain::dto::chat_completion::ChatCompletionChunkObject;

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
