use crate::{assistant::domain::dto::RunDto, entities::run, utils::time::TimeBuilder};

use super::metadata::{deserialize_metadata, serialize_metadata};

impl From<run::Model> for RunDto {
    fn from(model: run::Model) -> Self {
        RunDto {
            id: model.id.to_string(),
            created_at: TimeBuilder::from_string(&model.created_at).into(),
            assistant_id: model.assistant_id,
            thread_id: model.thread_id.map(|id| id.to_string()),
            instructions: model.instructions,
            model: model.model,
            status: model.status,
            metadata: model.metadata.map(|m| deserialize_metadata(&m)),
            temperature: model.temperature,
        }
    }
}

impl From<RunDto> for run::Model {
    fn from(dto: RunDto) -> Self {
        run::Model {
            id: dto.id.parse().unwrap(),
            created_at: TimeBuilder::from_i64(dto.created_at).into(),
            assistant_id: dto.assistant_id,
            thread_id: dto.thread_id.map(|id| id.parse().unwrap()),
            instructions: dto.instructions,
            model: dto.model,
            status: dto.status,
            metadata: dto.metadata.map(|m| serialize_metadata(&m)),
            temperature: dto.temperature,
        }
    }
}
