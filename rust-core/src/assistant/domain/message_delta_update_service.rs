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
        MessageContent, TextContentBlock, ThreadMessageDto,
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
        let new_content_text = self.get_new_text_content(chunk, message);
        let new_content = self.create_updated_text_content(&message.content, &new_content_text);

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
                        value: Some(new_content_text),
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
        existing_content: &Vec<MessageContent>,
        new_content: &str,
    ) -> Vec<MessageContent> {
        let new_text_block = TextContentBlock::new(&new_content);

        let mut content: Vec<MessageContent> = existing_content
            .iter()
            .filter_map(|c| match c {
                MessageContent::TextContentBlock(_) => None,
                c => Some(c.clone()),
            })
            .collect();

        content.push(MessageContent::TextContentBlock(new_text_block));

        content
    }
}
