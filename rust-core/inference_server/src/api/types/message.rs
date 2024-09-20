use app_core::agent::domain::{thread_repository::CreateThreadMessageParams, CreateMessageParams};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct CreateMessageDto {
    pub content: String,
    pub role: String,
    pub attachments: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
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
