#[cfg(test)]
#[path = "./message_type_adapter_test.rs"]
mod message_type_adapter_test;

pub fn to_langchain_message_type(role: String) -> langchain_rust::schemas::MessageType {
    match role.as_str() {
        "assistant" => langchain_rust::schemas::MessageType::AIMessage,
        "ai" => langchain_rust::schemas::MessageType::AIMessage,
        "human" => langchain_rust::schemas::MessageType::HumanMessage,
        "user" => langchain_rust::schemas::MessageType::HumanMessage,
        "tool" => langchain_rust::schemas::MessageType::ToolMessage,
        "system" => langchain_rust::schemas::MessageType::SystemMessage,
        _ => langchain_rust::schemas::MessageType::HumanMessage,
    }
}
