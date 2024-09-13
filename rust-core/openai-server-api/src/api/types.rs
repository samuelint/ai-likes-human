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

impl OpenAIChatCompletionChoice {
    pub fn new_stop(index: i32, message: Option<OpenAIMessage>) -> Self {
        OpenAIChatCompletionChoice {
            index,
            message,
            ..OpenAIChatCompletionChoice::default()
        }
    }
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
    pub id: Option<String>,
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
            id: None,
            object: "chat.completion".to_string(),
            created: Utc::now().timestamp_millis(),
            model: "".to_string(),
            system_fingerprint: None,
            choices: vec![],
            usage: OpenAIChatCompletionUsage::default(),
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
