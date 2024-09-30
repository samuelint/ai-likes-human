#[cfg(test)]
#[path = "./dto_test.rs"]
mod dto_test;

use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::llm::domain::message_type_adapter::to_langchain_message_type;

#[derive(Serialize, Deserialize, Clone)]
pub struct ChatCompletionMessageDto {
    pub role: String,
    pub content: String,
}

impl ChatCompletionMessageDto {
    pub fn new_assistant(content: &str) -> Self {
        ChatCompletionMessageDto {
            role: "assistant".to_string(),
            content: content.to_string(),
        }
    }
}

impl From<ChatCompletionMessageDto> for langchain_rust::schemas::Message {
    fn from(message: ChatCompletionMessageDto) -> Self {
        langchain_rust::schemas::Message {
            content: message.content.clone(),
            message_type: to_langchain_message_type(message.role.clone()),
            id: None,
            tool_calls: None,
            images: None,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ChatCompletionUsage {
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: i32,
}

impl Default for ChatCompletionUsage {
    fn default() -> Self {
        ChatCompletionUsage {
            prompt_tokens: -1,
            completion_tokens: -1,
            total_tokens: -1,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ChatCompletionChoice {
    pub index: i32,
    pub message: Option<ChatCompletionMessageDto>,
    pub finish_reason: Option<String>,
}

impl Default for ChatCompletionChoice {
    fn default() -> Self {
        ChatCompletionChoice {
            index: 0,
            message: None,
            finish_reason: None,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ChatCompletionObject {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub system_fingerprint: Option<String>,
    pub choices: Vec<ChatCompletionChoice>,
    pub usage: ChatCompletionUsage,
}

impl Default for ChatCompletionObject {
    fn default() -> Self {
        ChatCompletionObject {
            id: "".to_string(),
            object: "chat.completion".to_string(),
            created: Utc::now().timestamp_millis(),
            model: "".to_string(),
            system_fingerprint: None,
            choices: vec![],
            usage: ChatCompletionUsage::default(),
        }
    }
}

impl ChatCompletionObject {
    pub fn new_single_choice(message: ChatCompletionMessageDto, model: &str) -> Self {
        ChatCompletionObject {
            choices: vec![ChatCompletionChoice {
                index: 0,
                message: Some(message),
                finish_reason: Some("stop".to_string()),
            }],
            model: model.to_string(),
            ..ChatCompletionObject::default()
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ChatCompletionChunkChoice {
    pub index: i32,
    pub delta: Option<ChatCompletionMessageDto>,
    pub finish_reason: Option<String>,
}

impl Default for ChatCompletionChunkChoice {
    fn default() -> Self {
        ChatCompletionChunkChoice {
            index: 0,
            delta: None,
            finish_reason: Some("stop".to_string()),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ChatCompletionChunkObject {
    pub id: Option<String>,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub system_fingerprint: Option<String>,
    pub choices: Vec<ChatCompletionChunkChoice>,
    pub usage: ChatCompletionUsage,
}

impl ChatCompletionChunkObject {
    pub fn to_content_string(&self) -> String {
        self.choices
            .iter()
            .map(|x| x.delta.as_ref().unwrap().content.clone())
            .collect::<Vec<String>>()
            .join("")
    }
}

impl Default for ChatCompletionChunkObject {
    fn default() -> Self {
        ChatCompletionChunkObject {
            id: None,
            object: "chat.completion.chunk".to_string(),
            created: Utc::now().timestamp_millis(),
            model: "".to_string(),
            system_fingerprint: None,
            choices: vec![],
            usage: ChatCompletionUsage::default(),
        }
    }
}

impl ChatCompletionChunkObject {
    pub fn new_assistant_chunk(message: &str, model: &str) -> Self {
        let choice = ChatCompletionChunkChoice {
            delta: Some(ChatCompletionMessageDto::new_assistant(message)),
            ..ChatCompletionChunkChoice::default()
        };
        ChatCompletionChunkObject {
            choices: vec![choice],
            model: model.to_string(),
            ..ChatCompletionChunkObject::default()
        }
    }
}
