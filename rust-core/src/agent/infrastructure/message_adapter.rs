use sea_orm::ActiveValue;

use crate::{
    agent::domain::CreateMessageParams, entities::message, utils::time::current_time_with_timezone,
};

impl From<&CreateMessageParams> for message::ActiveModel {
    fn from(item: &CreateMessageParams) -> Self {
        let thread_id: Option<i32> = item.thread_id.clone().map(|id| id.parse().unwrap());
        let run_id: Option<i32> = item.run_id.clone().map(|id| id.parse().unwrap());
        message::ActiveModel {
            created_at: ActiveValue::Set(current_time_with_timezone()),
            content: ActiveValue::Set(item.content.to_owned()),
            role: ActiveValue::Set(item.role.to_owned()),
            thread_id: ActiveValue::Set(thread_id),
            run_id: ActiveValue::Set(run_id),
            attachments: ActiveValue::Set(item.attachments.to_owned()),
            metadata: ActiveValue::Set(item.metadata.to_owned()),
            ..Default::default()
        }
    }
}
