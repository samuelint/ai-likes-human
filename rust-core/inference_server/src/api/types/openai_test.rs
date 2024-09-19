#[cfg(test)]
mod openai_message_tests {
    use crate::openai::OpenAIMessage;

    #[test]
    fn test_create_assistant_message() {
        let result = OpenAIMessage::new_assistant("Hello".to_string());

        assert!(result.role == "assistant");
    }

    #[test]
    fn test_create_message_string_content() {
        let result = OpenAIMessage::new_assistant("Hello".to_string());

        assert!(result.content == "Hello");
    }
}

mod chat_completion_object_tests {
    use chrono::Utc;

    use crate::openai::{OpenAIChatCompletionObject, OpenAIMessage};

    #[test]
    fn test_openaichat_completion_object() {
        let result = OpenAIChatCompletionObject::default();

        assert!(result.object == "chat.completion");
    }

    #[test]
    fn test_openaichat_completion_object_id_is_none_by_default() {
        let result = OpenAIChatCompletionObject::default();

        assert!(result.id == None);
    }

    #[test]
    fn test_openaichat_completion_created_is_now() {
        let result = OpenAIChatCompletionObject::default();

        let now = Utc::now().timestamp_millis();
        let delta = (result.created - now).abs();
        assert!(delta < 100);
    }

    #[test]
    fn test_openaichat_completion_object_single_choice() {
        let message = OpenAIMessage::new_assistant("Hello".to_string());
        let result = OpenAIChatCompletionObject::new_single_choice(message);

        let choice1 = &result.choices[0];
        assert!(choice1.index == 0);
        assert!(choice1.message.is_some());
        assert!(choice1.message.as_ref().unwrap().content == "Hello");
        assert!(choice1.message.as_ref().unwrap().role == "assistant");
    }

    #[test]
    fn test_openaichat_completion_object_single_choice_finish_reason_is_stop() {
        let message = OpenAIMessage::new_assistant("Hello".to_string());
        let result = OpenAIChatCompletionObject::new_single_choice(message);

        let choice1 = &result.choices[0];
        assert!(choice1.finish_reason.as_ref().unwrap() == "stop");
    }
}

mod openai_chat_completion_chunk_object {
    use chrono::Utc;

    use crate::openai::OpenAIChatCompletionChunkObject;

    #[test]
    fn test_openai_chat_completion_chunk_object_id_is_none_by_default() {
        let result = OpenAIChatCompletionChunkObject::default();

        assert!(result.id == None);
    }

    #[test]
    fn test_openai_chat_completion_chunk_object_object() {
        let result = OpenAIChatCompletionChunkObject::default();

        assert!(result.object == "chat.completion.chunk");
    }

    #[test]
    fn test_openai_chat_completion_chunk_object_created_is_now() {
        let result = OpenAIChatCompletionChunkObject::default();

        let now = Utc::now().timestamp_millis();
        let delta = (result.created - now).abs();
        assert!(delta < 100);
    }
}
