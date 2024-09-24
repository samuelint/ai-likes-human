use serde::{Deserialize, Serialize};

use crate::{agent::domain::CreateThreadParams, entities::run};

use super::CreateThreadDto;

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct RunDto {
    pub id: String,
    pub created_at: String,
    pub assistant_id: String,
    pub thread_id: Option<String>,
    pub instructions: Option<String>,
    pub model: String,
    pub status: String,
    pub metadata: Option<String>,
    pub temperature: Option<i32>,
}

impl From<run::Model> for RunDto {
    fn from(model: run::Model) -> Self {
        RunDto {
            id: model.id.to_string(),
            created_at: model.created_at,
            assistant_id: model.assistant_id,
            thread_id: model.thread_id.map(|id| id.to_string()),
            instructions: model.instructions,
            model: model.model,
            status: model.status,
            metadata: model.metadata,
            temperature: model.temperature,
        }
    }
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
