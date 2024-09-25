#[cfg(test)]
#[path = "./message_delta_update_service_test.rs"]
mod message_delta_update_service_test;

use std::{error::Error, sync::Arc};

use crate::{
    assistant::domain::dto::UpdateThreadMessageDto, chat_completion::ChatCompletionChunkObject,
};

use super::{
    dto::{
        message_delta::{
            MessageContentDelta, MessageDeltaDto, TextDeltaDto, ThreadMessageDeltaDto,
        },
        ThreadMessageDto,
    },
    message_repository::MessageRepository,
};

pub struct MessageDeltaUpdateService {
    message_repository: Arc<dyn MessageRepository>,
}

impl MessageDeltaUpdateService {
    pub fn new(message_repository: Arc<dyn MessageRepository>) -> Self {
        Self { message_repository }
    }

    pub async fn from_chunk(
        &self,
        chunk: &ChatCompletionChunkObject,
        message: &ThreadMessageDto,
    ) -> Result<(ThreadMessageDeltaDto, ThreadMessageDto), Box<dyn Error + Send>> {
        let chunk_content = chunk.to_content_string();
        let new_content = format!("{}{}", message.content, chunk_content);

        let message = self
            .message_repository
            .update(UpdateThreadMessageDto {
                id: message.id.clone(),
                content: Some(new_content.clone()),
                ..UpdateThreadMessageDto::default()
            })
            .await?;

        Ok((
            ThreadMessageDeltaDto {
                delta: MessageDeltaDto {
                    role: message.role.clone(),
                    content: vec![MessageContentDelta::Text(TextDeltaDto {
                        value: Some(new_content),
                        ..TextDeltaDto::default()
                    })],
                },
                ..ThreadMessageDeltaDto::default()
            },
            message,
        ))
    }
}
