use crate::{
    assistant::domain::{
        dto::{CreateThreadDto, ThreadDto},
        CreateThreadParams,
    },
    entities::thread,
};

impl From<CreateThreadDto> for CreateThreadParams {
    fn from(dto: CreateThreadDto) -> Self {
        CreateThreadParams {
            metadata: dto.metadata,
            messages: dto.messages.into_iter().map(|m| m.into()).collect(),
        }
    }
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
