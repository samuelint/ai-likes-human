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

    #[test]
    fn test_conversion_to_langchain_message_have_content() {
        let result = OpenAIMessage::new_assistant("Hello".to_string()).to_langchain();

        assert!(result.content == "Hello");
    }
}

#[cfg(test)]
mod to_langchain_message_type_tests {
    use crate::openai::to_langchain_message_type;

    #[test]
    fn test_assistant_is_ai_message() {
        let result = to_langchain_message_type("assistant".to_string());

        assert!(result == langchain_rust::schemas::MessageType::AIMessage);
    }

    #[test]
    fn test_ai_is_ai_message() {
        let result = to_langchain_message_type("ai".to_string());

        assert!(result == langchain_rust::schemas::MessageType::AIMessage);
    }

    #[test]
    fn test_human_is_human_message() {
        let result = to_langchain_message_type("human".to_string());

        assert!(result == langchain_rust::schemas::MessageType::HumanMessage);
    }

    #[test]
    fn test_user_is_human_message() {
        let result = to_langchain_message_type("user".to_string());

        assert!(result == langchain_rust::schemas::MessageType::HumanMessage);
    }

    #[test]
    fn test_tool_is_tool_message() {
        let result = to_langchain_message_type("tool".to_string());

        assert!(result == langchain_rust::schemas::MessageType::ToolMessage);
    }

    #[test]
    fn test_system_is_system_message() {
        let result = to_langchain_message_type("system".to_string());

        assert!(result == langchain_rust::schemas::MessageType::SystemMessage);
    }

    #[test]
    fn test_unknown_is_human_message() {
        let result = to_langchain_message_type("dsf".to_string());

        assert!(result == langchain_rust::schemas::MessageType::HumanMessage);
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
    fn test_openaichat_completion_object_id_is_empty_string_by_default() {
        let result = OpenAIChatCompletionObject::default();

        assert!(result.id == "");
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
        let result = OpenAIChatCompletionObject::new_single_choice(message, "".to_string());

        let choice1 = &result.choices[0];
        assert!(choice1.index == 0);
        assert!(choice1.message.is_some());
        assert!(choice1.message.as_ref().unwrap().content == "Hello");
        assert!(choice1.message.as_ref().unwrap().role == "assistant");
    }

    #[test]
    fn test_openaichat_completion_object_single_choice_finish_reason_is_stop() {
        let message = OpenAIMessage::new_assistant("Hello".to_string());
        let result = OpenAIChatCompletionObject::new_single_choice(message, "".to_string());

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
