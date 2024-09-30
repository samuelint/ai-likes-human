use crate::{
    assistant::domain::dto::{ApiCreateThreadDto, DbCreateThreadDto, ThreadDto},
    entities::thread,
    utils::time::TimeBuilder,
};

use super::metadata::{deserialize_metadata, serialize_metadata};

impl From<ApiCreateThreadDto> for DbCreateThreadDto {
    fn from(dto: ApiCreateThreadDto) -> Self {
        DbCreateThreadDto {
            metadata: dto.metadata,
            messages: dto.messages.into_iter().map(|m| (&m).into()).collect(),
        }
    }
}

impl From<thread::Model> for ThreadDto {
    fn from(model: thread::Model) -> Self {
        ThreadDto {
            id: model.id.to_string(),
            created_at: TimeBuilder::from_string(&model.created_at).into(),
            metadata: deserialize_metadata(&model.metadata),
        }
    }
}

impl From<ThreadDto> for thread::Model {
    fn from(dto: ThreadDto) -> Self {
        thread::Model {
            id: dto.id.parse().expect("thread id should be a number"),
            created_at: TimeBuilder::from_i64(dto.created_at).into(),
            metadata: serialize_metadata(&dto.metadata),
        }
    }
}
