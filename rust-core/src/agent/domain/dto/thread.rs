use serde::{Deserialize, Serialize};

use crate::{agent::domain::CreateThreadParams, entities::thread};

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

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct ThreadDto {
    pub id: String,
    pub created_at: String,
    pub metadata: String,
}

impl From<thread::Model> for ThreadDto {
    fn from(model: thread::Model) -> Self {
        ThreadDto {
            id: model.id.to_string(),
            created_at: model.created_at.to_string(),
            metadata: model.metadata,
        }
    }
}

impl From<ThreadDto> for thread::Model {
    fn from(dto: ThreadDto) -> Self {
        thread::Model {
            id: dto.id.parse().expect("thread id should be a number"),
            created_at: dto.created_at,
            metadata: dto.metadata,
        }
    }
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct UpdateThreadDto {
    pub metadata: Option<String>,
}
