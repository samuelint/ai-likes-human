#[cfg(test)]
mod to_langchain_message_type_tests {
    use crate::llm::domain::message_type_adapter::to_langchain_message_type;

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
