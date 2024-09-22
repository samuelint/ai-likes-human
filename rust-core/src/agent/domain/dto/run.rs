use serde::{Deserialize, Serialize};

use crate::agent::domain::CreateThreadParams;

use super::CreateThreadDto;

#[derive(Default, Serialize, Deserialize)]
pub struct RunDto {
    pub id: i32,
    pub created_at: String,
    pub assistant_id: String,
    pub thread_id: Option<i32>,
    pub instructions: Option<String>,
    pub model: String,
    pub status: String,
    pub metadata: Option<String>,
    pub temperature: Option<i32>,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct CreateRunDto {
    pub assistant_id: Option<String>,
    pub model: String,
    pub instructions: Option<String>,
    pub temperature: Option<i32>,
    pub metadata: Option<String>,
    pub stream: Option<bool>,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct CreateThreadAndRunDto {
    pub assistant_id: Option<String>,
    pub thread: CreateThreadDto,
    pub model: String,
    pub instructions: Option<String>,
    pub metadata: Option<String>,
    pub temperature: Option<i32>,
    pub stream: Option<bool>,
}

impl From<CreateThreadAndRunDto> for CreateThreadParams {
    fn from(dto: CreateThreadAndRunDto) -> Self {
        CreateThreadParams {
            metadata: dto.thread.metadata,
            messages: dto
                .thread
                .messages
                .iter()
                .map(|m| m.clone().into())
                .collect(),
        }
    }
}

impl From<CreateThreadAndRunDto> for CreateThreadDto {
    fn from(dto: CreateThreadAndRunDto) -> Self {
        dto.thread
    }
}

impl From<CreateThreadAndRunDto> for CreateRunDto {
    fn from(dto: CreateThreadAndRunDto) -> Self {
        CreateRunDto {
            assistant_id: dto.assistant_id,
            model: dto.model,
            instructions: dto.instructions,
            temperature: dto.temperature,
            metadata: dto.metadata,
            stream: dto.stream,
        }
    }
}
