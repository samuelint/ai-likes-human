use serde::{Deserialize, Serialize};

use crate::{
    agent::domain::{thread_repository::CreateThreadMessageParams, CreateMessageParams},
    entities::message,
};

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct AnnotationDto {
    pub value: String,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct TextContentDto {
    pub value: String,
    pub annotations: Option<AnnotationDto>,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct TextMessageContentDto {
    pub r#type: String,
    pub text: TextContentDto,
}

#[derive(Default, Serialize, Deserialize, Clone)]
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

impl From<message::Model> for ThreadMessageDto {
    fn from(model: message::Model) -> Self {
        ThreadMessageDto {
            id: model.id.to_string(),
            object: "thread.message".to_string(),
            created_at: model.created_at,
            thread_id: model.thread_id.map(|id| id.to_string()),
            status: "completed".to_string(),
            role: model.role,
            content: model.content,
            assistant_id: None,
            run_id: model.run_id.map(|id| id.to_string()),
            metadata: model.metadata,
        }
    }
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct CreateMessageDto {
    pub content: String,
    pub role: String,
    pub attachments: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
}

impl CreateMessageDto {
    pub fn user() -> Self {
        Self {
            role: "user".to_string(),
            ..Self::default()
        }
    }
}

impl From<CreateMessageDto> for CreateThreadMessageParams {
    fn from(dto: CreateMessageDto) -> Self {
        CreateThreadMessageParams {
            content: dto.content,
            role: dto.role,
        }
    }
}

impl From<CreateMessageDto> for CreateMessageParams {
    fn from(dto: CreateMessageDto) -> Self {
        CreateMessageParams {
            content: dto.content,
            role: dto.role,
            thread_id: None,
            run_id: None,
            attachments: dto.attachments.as_ref().map(|v| v.to_string()),
            metadata: dto.metadata.as_ref().map(|v| v.to_string()),
        }
    }
}
