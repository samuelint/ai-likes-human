#[cfg(test)]
#[path = "./completion_test.rs"]
mod completion_test;

use super::{ApiMessageContent, ImageUrl};
use chrono::Utc;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatCompletionMessageDto>,
    pub stream: Option<bool>,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct ChatCompletionMessageDto {
    pub role: String,
    pub content: Vec<ApiMessageContent>,
}

impl ChatCompletionMessageDto {
    pub fn assistant(content: &str) -> Self {
        ChatCompletionMessageDto {
            role: "assistant".to_string(),
            content: vec![ApiMessageContent::text(content)],
        }
    }

    pub fn user(content: &str) -> Self {
        ChatCompletionMessageDto {
            role: "user".to_string(),
            content: vec![ApiMessageContent::text(content)],
        }
    }

    pub fn to_string_content(&self) -> String {
        self.content
            .iter()
            .filter_map(|c| match c {
                ApiMessageContent::Text { text } => Some(text.to_string()),
                _ => None,
            })
            .join("")
    }

    pub fn to_images_url_vec(&self) -> Vec<ImageUrl> {
        self.content
            .iter()
            .filter_map(|c| match c {
                ApiMessageContent::ImageUrl { image_url } => Some(image_url),
                _ => None,
            })
            .cloned()
            .collect()
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

#[derive(Default, Serialize, Deserialize)]
pub struct ChatCompletionDeltaDto {
    pub role: String,
    pub content: String,
}

impl ChatCompletionDeltaDto {
    pub fn new_assistant(content: &str) -> Self {
        ChatCompletionDeltaDto {
            role: "assistant".to_string(),
            content: content.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ChatCompletionChunkChoice {
    pub index: i32,
    pub delta: Option<ChatCompletionDeltaDto>,
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
            delta: Some(ChatCompletionDeltaDto::new_assistant(message)),
            ..ChatCompletionChunkChoice::default()
        };
        ChatCompletionChunkObject {
            choices: vec![choice],
            model: model.to_string(),
            ..ChatCompletionChunkObject::default()
        }
    }
}
