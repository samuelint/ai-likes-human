use serde::{Deserialize, Serialize};

use crate::agent::domain::CreateThreadParams;

use super::message::CreateMessageDto;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct CreateThreadDto {
    pub metadata: Option<String>,
    pub messages: Vec<CreateMessageDto>,
}

impl From<CreateThreadDto> for CreateThreadParams {
    fn from(dto: CreateThreadDto) -> Self {
        CreateThreadParams {
            metadata: dto.metadata,
            messages: dto.messages.into_iter().map(|m| m.into()).collect(),
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct ThreadDto {
    pub id: i64,
    pub created_at: String,
    pub metadata: String,
}

#[derive(Default, Serialize, Deserialize)]
pub struct UpdateThreadDto {
    pub metadata: Option<String>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct ThreadMessageDto {
    pub id: i64,
    pub content: String,
    pub role: String,
}
