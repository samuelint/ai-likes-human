#[cfg(test)]
#[path = "./openai_test.rs"]
mod openai_test;

use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OpenAIMessage {
    pub role: String,
    pub content: String,
}

impl OpenAIMessage {
    pub fn new_assistant(content: String) -> Self {
        OpenAIMessage {
            role: "assistant".to_string(),
            content,
        }
    }

    pub fn to_langchain(&self) -> langchain_rust::schemas::Message {
        langchain_rust::schemas::Message {
            content: self.content.clone(),
            message_type: to_langchain_message_type(self.role.clone()),
            id: None,
            tool_calls: None,
            images: None,
        }
    }
}

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

pub fn to_langchain_messages(
    messages: Vec<OpenAIMessage>,
) -> Vec<langchain_rust::schemas::Message> {
    messages
        .iter()
        .map(|message| message.to_langchain())
        .collect()
}

#[derive(Serialize, Deserialize)]
pub struct OpenAIChatCompletionUsage {
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: i32,
}

impl Default for OpenAIChatCompletionUsage {
    fn default() -> Self {
        OpenAIChatCompletionUsage {
            prompt_tokens: -1,
            completion_tokens: -1,
            total_tokens: -1,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct OpenAIChatCompletionChoice {
    pub index: i32,
    pub message: Option<OpenAIMessage>,
    pub finish_reason: Option<String>,
}

impl Default for OpenAIChatCompletionChoice {
    fn default() -> Self {
        OpenAIChatCompletionChoice {
            index: 0,
            message: None,
            finish_reason: None,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct OpenAIChatCompletionObject {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub system_fingerprint: Option<String>,
    pub choices: Vec<OpenAIChatCompletionChoice>,
    pub usage: OpenAIChatCompletionUsage,
}

impl Default for OpenAIChatCompletionObject {
    fn default() -> Self {
        OpenAIChatCompletionObject {
            id: "".to_string(),
            object: "chat.completion".to_string(),
            created: Utc::now().timestamp_millis(),
            model: "".to_string(),
            system_fingerprint: None,
            choices: vec![],
            usage: OpenAIChatCompletionUsage::default(),
        }
    }
}

impl OpenAIChatCompletionObject {
    pub fn new_single_choice(message: OpenAIMessage, model: String) -> Self {
        OpenAIChatCompletionObject {
            choices: vec![OpenAIChatCompletionChoice {
                index: 0,
                message: Some(message),
                finish_reason: Some("stop".to_string()),
            }],
            model,
            ..OpenAIChatCompletionObject::default()
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct OpenAIChatCompletionChunkChoice {
    pub index: i32,
    pub delta: Option<OpenAIMessage>,
    pub finish_reason: Option<String>,
}

impl Default for OpenAIChatCompletionChunkChoice {
    fn default() -> Self {
        OpenAIChatCompletionChunkChoice {
            index: 0,
            delta: None,
            finish_reason: Some("stop".to_string()),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct OpenAIChatCompletionChunkObject {
    pub id: Option<String>,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub system_fingerprint: Option<String>,
    pub choices: Vec<OpenAIChatCompletionChunkChoice>,
    pub usage: OpenAIChatCompletionUsage,
}

impl Default for OpenAIChatCompletionChunkObject {
    fn default() -> Self {
        OpenAIChatCompletionChunkObject {
            id: None,
            object: "chat.completion.chunk".to_string(),
            created: Utc::now().timestamp_millis(),
            model: "".to_string(),
            system_fingerprint: None,
            choices: vec![],
            usage: OpenAIChatCompletionUsage::default(),
        }
    }
}

impl OpenAIChatCompletionChunkObject {
    pub fn new_assistant_chunk(message: String, model: String) -> Self {
        let choice = OpenAIChatCompletionChunkChoice {
            delta: Some(OpenAIMessage::new_assistant(message)),
            ..OpenAIChatCompletionChunkChoice::default()
        };
        OpenAIChatCompletionChunkObject {
            choices: vec![choice],
            model: model,
            ..OpenAIChatCompletionChunkObject::default()
        }
    }
}
