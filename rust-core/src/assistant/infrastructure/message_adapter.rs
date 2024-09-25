#[cfg(test)]
#[path = "./message_adapter_test.rs"]
mod message_adapter_test;

use sea_orm::ActiveValue;

use crate::{
    assistant::domain::dto::{CreateThreadMessageDto, MessageContent, ThreadMessageDto},
    entities::message,
    utils::time::current_time_with_timezone,
};

use super::metadata::{deserialize_metadata, serialize_metadata};

impl From<&CreateThreadMessageDto> for message::ActiveModel {
    fn from(item: &CreateThreadMessageDto) -> Self {
        let thread_id: Option<i32> = item.thread_id.clone().map(|id| id.parse().unwrap());
        let run_id: Option<i32> = item.run_id.clone().map(|id| id.parse().unwrap());
        let json_content: String = serde_json::to_string(&item.content).unwrap();
        let json_metadata = item.metadata.clone().map(|m| serialize_metadata(&m));

        message::ActiveModel {
            created_at: ActiveValue::Set(current_time_with_timezone()),
            content: ActiveValue::Set(json_content),
            role: ActiveValue::Set(item.role.to_owned()),
            thread_id: ActiveValue::Set(thread_id),
            run_id: ActiveValue::Set(run_id),
            attachments: ActiveValue::Set(item.attachments.to_owned()),
            metadata: ActiveValue::Set(json_metadata),
            status: ActiveValue::Set(item.status.to_owned()),
            ..Default::default()
        }
    }
}

impl From<message::Model> for ThreadMessageDto {
    fn from(model: message::Model) -> Self {
        let content: Vec<MessageContent> = match serde_json::from_str(&model.content) {
            Ok(content) => content,
            Err(_) => vec![],
        };

        let metadata = model.metadata.map(|m| deserialize_metadata(&m));

        ThreadMessageDto {
            id: model.id.to_string(),
            object: "thread.message".to_string(),
            created_at: model.created_at,
            thread_id: model.thread_id.map(|id| id.to_string()),
            status: model.status,
            role: model.role,
            content: content,
            assistant_id: None,
            run_id: model.run_id.map(|id| id.to_string()),
            metadata,
        }
    }
}
