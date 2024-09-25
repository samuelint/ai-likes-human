use serde::{Deserialize, Serialize};

use crate::{
    assistant::domain::thread_repository::CreateThreadMessageParams,
    chat_completion::ChatCompletionMessageDto,
};

use super::annotation::MessageAnnotation;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct TextContentDto {
    pub value: String,
    pub annotations: Vec<MessageAnnotation>,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct TextMessageContentDto {
    pub r#type: String,
    pub text: TextContentDto,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct ThreadMessageDto {
    pub id: String,
    pub object: String,
    pub created_at: String,
    pub thread_id: Option<String>,
    pub status: String,
    pub role: String,
    // pub content: Vec<TextMessageContentDto>,
    pub content: String,
    pub assistant_id: Option<String>,
    pub run_id: Option<String>,
    pub metadata: Option<String>,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct UpdateThreadMessageDto {
    pub id: String,
    pub status: Option<String>,
    // pub content: Vec<TextMessageContentDto>,
    pub content: Option<String>,
    pub assistant_id: Option<Option<String>>,
    pub metadata: Option<Option<String>>,
}

impl From<ThreadMessageDto> for ChatCompletionMessageDto {
    fn from(dto: ThreadMessageDto) -> Self {
        ChatCompletionMessageDto {
            content: dto.content,
            role: dto.role,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CreateThreadMessageDto {
    pub content: String,
    pub role: String,
    pub status: String,
    pub thread_id: Option<String>,
    pub run_id: Option<String>,
    pub attachments: Option<String>,
    pub metadata: Option<String>,
}

impl CreateThreadMessageDto {
    pub fn user() -> Self {
        Self {
            role: "user".to_string(),
            ..Self::default()
        }
    }
}

impl Default for CreateThreadMessageDto {
    fn default() -> Self {
        Self {
            content: "".to_string(),
            role: "user".to_string(),
            thread_id: None,
            run_id: None,
            attachments: None,
            status: "in_progress".to_string(),
            metadata: None,
        }
    }
}

impl From<CreateThreadMessageDto> for CreateThreadMessageParams {
    fn from(dto: CreateThreadMessageDto) -> Self {
        CreateThreadMessageParams {
            content: dto.content,
            role: dto.role,
        }
    }
}
