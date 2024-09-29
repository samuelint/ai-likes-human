#[cfg(test)]
#[path = "./message_delta_update_service_test.rs"]
mod message_delta_update_service_test;

use std::{error::Error, sync::Arc};

use crate::{
    assistant::domain::dto::{
        message_delta::{
            MessageContentDelta, MessageDeltaDto, TextDeltaDto, ThreadMessageDeltaDto,
        },
        DbUpdateThreadMessageDto, ApiMessageContent, ThreadMessageDto,
    },
    chat_completion::ChatCompletionChunkObject,
};

use super::message_repository::MessageRepository;

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
        let new_content_text = self.get_new_text_content(chunk, message);
        let new_content = self.create_updated_text_content(&message.content, &new_content_text);

        let message = self
            .message_repository
            .update(
                &message.id,
                &DbUpdateThreadMessageDto {
                    content: Some(new_content.clone()),
                    ..DbUpdateThreadMessageDto::default()
                },
            )
            .await?;

        Ok((
            ThreadMessageDeltaDto {
                id: message.id.clone(),
                delta: MessageDeltaDto {
                    role: message.role.clone(),
                    content: vec![MessageContentDelta::Text(TextDeltaDto {
                        value: Some(chunk_content),
                        ..TextDeltaDto::default()
                    })],
                },
                ..ThreadMessageDeltaDto::default()
            },
            message,
        ))
    }

    fn get_new_text_content(
        &self,
        chunk: &ChatCompletionChunkObject,
        message: &ThreadMessageDto,
    ) -> String {
        let chunk_content = chunk.to_content_string();
        let existing_content = message.to_string_content();

        format!("{existing_content}{chunk_content}")
    }

    fn create_updated_text_content(
        &self,
        existing_content: &Vec<ApiMessageContent>,
        new_content: &str,
    ) -> Vec<ApiMessageContent> {
        let mut content: Vec<ApiMessageContent> = existing_content
            .iter()
            .filter_map(|c| match c {
                ApiMessageContent::Text { .. } => None,
                c => Some(c.clone()),
            })
            .collect();

        content.push(ApiMessageContent::text(new_content));

        content
    }
}
