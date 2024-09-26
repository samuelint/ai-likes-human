use serde::{Deserialize, Serialize};

use super::{ApiCreateThreadDto, DbCreateThreadDto, Metadata};

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct RunDto {
    pub id: String,
    pub created_at: i64,
    pub assistant_id: String,
    pub thread_id: Option<String>,
    pub instructions: Option<String>,
    pub model: String,
    pub status: String,
    pub metadata: Option<Metadata>,
    pub temperature: Option<i32>,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct ApiCreateRunDto {
    pub assistant_id: Option<String>,
    pub model: String,
    pub instructions: Option<String>,
    pub temperature: Option<i32>,
    pub metadata: Option<Metadata>,
    pub stream: Option<bool>,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct ApiCreateThreadAndRunDto {
    pub assistant_id: Option<String>,
    pub thread: ApiCreateThreadDto,
    pub model: String,
    pub instructions: Option<String>,
    pub metadata: Option<Metadata>,
    pub temperature: Option<i32>,
    pub stream: Option<bool>,
}

impl From<&ApiCreateThreadAndRunDto> for DbCreateThreadDto {
    fn from(dto: &ApiCreateThreadAndRunDto) -> Self {
        DbCreateThreadDto {
            metadata: dto.thread.metadata.clone(),
            messages: dto.thread.messages.iter().map(|m| m.into()).collect(),
        }
    }
}

impl From<&ApiCreateThreadAndRunDto> for ApiCreateThreadDto {
    fn from(dto: &ApiCreateThreadAndRunDto) -> Self {
        dto.thread.clone()
    }
}

impl From<&ApiCreateThreadAndRunDto> for ApiCreateRunDto {
    fn from(dto: &ApiCreateThreadAndRunDto) -> Self {
        ApiCreateRunDto {
            assistant_id: dto.assistant_id.clone(),
            model: dto.model.clone(),
            instructions: dto.instructions.clone(),
            temperature: dto.temperature,
            metadata: dto.metadata.clone(),
            stream: dto.stream,
        }
    }
}
