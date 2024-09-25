use crate::llm::domain::message_type_adapter::to_langchain_message_type;

use super::dto::ThreadMessageDto;

impl From<ThreadMessageDto> for langchain_rust::schemas::Message {
    fn from(message: ThreadMessageDto) -> Self {
        langchain_rust::schemas::Message {
            content: message.content.clone(),
            message_type: to_langchain_message_type(message.role.clone()),
            id: None,
            tool_calls: None,
            images: None,
        }
    }
}
