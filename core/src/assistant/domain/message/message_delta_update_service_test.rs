#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        assistant::domain::{
            dto::{
                message_delta::MessageDeltaContent, ApiMessageContent, DbUpdateThreadMessageDto,
                ThreadMessageDto,
            },
            message::{message_repository::MockMessageRepository, MessageDeltaUpdateService},
        },
        chat_completion::{
            ChatCompletionChunkChoice, ChatCompletionChunkObject, ChatCompletionMessageDto,
        },
    };

    #[tokio::test]
    async fn test_updated_message_from_chunk_contains_full_message_in_single_text_block() {
        // Given new chunk adding "World! to existing message"
        let thread_id = "some_thread_id";
        let message_id = "some_message_id";
        let expected_update_message = DbUpdateThreadMessageDto {
            content: Some(vec![ApiMessageContent::text("Hello World!")]),
            ..DbUpdateThreadMessageDto::default()
        };
        let existing_message = ThreadMessageDto {
            id: message_id.to_string(),
            thread_id: Some(thread_id.to_string()),
            content: vec![ApiMessageContent::text("Hello ")],
            ..ThreadMessageDto::default()
        };
        let chunk = ChatCompletionChunkObject {
            choices: vec![ChatCompletionChunkChoice {
                delta: Some(ChatCompletionMessageDto {
                    role: "assistant".to_string(),
                    content: "World!".to_string(),
                }),
                ..ChatCompletionChunkChoice::default()
            }],
            ..ChatCompletionChunkObject::default()
        };
        let repository = message_repository_mocking_update(&thread_id, &expected_update_message);
        let instance = MessageDeltaUpdateService::new(Arc::new(repository));

        // When updating message already containing "Hello" from chunk
        let (_delta, message) = instance
            .from_chunk(&chunk, &existing_message)
            .await
            .unwrap();

        // Then message should contain a single Text Block Content
        let text_blocks = message
            .content
            .iter()
            .filter_map(|c| match c {
                ApiMessageContent::Text { text } => Some(text),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(text_blocks.len(), 1);

        // Then message Text Block Content should contain "Hello World!"
        let text_block = text_blocks[0];

        assert_eq!(text_block.to_string(), "Hello World!");
    }

    #[tokio::test]
    async fn test_delta_contains_only_chunk_content() {
        let thread_id = "some_thread_id";
        let message_id = "some_message_id";
        let expected_update_message = DbUpdateThreadMessageDto {
            content: Some(vec![ApiMessageContent::text("Hello World!")]),
            ..DbUpdateThreadMessageDto::default()
        };
        let existing_message = ThreadMessageDto {
            id: message_id.to_string(),
            thread_id: Some(thread_id.to_string()),
            content: vec![ApiMessageContent::text("Hello ")],
            ..ThreadMessageDto::default()
        };
        let chunk = ChatCompletionChunkObject {
            choices: vec![ChatCompletionChunkChoice {
                delta: Some(ChatCompletionMessageDto {
                    role: "assistant".to_string(),
                    content: "World!".to_string(),
                }),
                ..ChatCompletionChunkChoice::default()
            }],
            ..ChatCompletionChunkObject::default()
        };

        let repository = message_repository_mocking_update(&thread_id, &expected_update_message);
        let instance = MessageDeltaUpdateService::new(Arc::new(repository));

        let (delta, _message) = instance
            .from_chunk(&chunk, &existing_message)
            .await
            .unwrap();

        assert!(delta.delta.content.len() > 0);
        let message_content = &delta.delta.content[0];

        let text_delta = match &message_content {
            MessageDeltaContent::Text { text, .. } => text.to_string(),
            _ => panic!("Expected Text MessageContentDelta"),
        };

        assert_eq!(text_delta, "World!");
    }

    #[tokio::test]
    async fn test_updated_message_chunk_role_is_same_as_message() {
        let thread_id = "some_thread_id";
        let message_id = "some_message_id";
        let expected_update_message = DbUpdateThreadMessageDto {
            content: Some(vec![ApiMessageContent::text("Hello World!")]),
            ..DbUpdateThreadMessageDto::default()
        };
        let existing_message = ThreadMessageDto {
            id: message_id.to_string(),
            thread_id: Some(thread_id.to_string()),
            content: vec![ApiMessageContent::text("Hello ")],
            role: "assistant".to_string(),
            ..ThreadMessageDto::default()
        };
        let chunk = ChatCompletionChunkObject {
            choices: vec![ChatCompletionChunkChoice {
                delta: Some(ChatCompletionMessageDto {
                    role: "assistant".to_string(),
                    content: "World!".to_string(),
                }),
                ..ChatCompletionChunkChoice::default()
            }],
            ..ChatCompletionChunkObject::default()
        };

        let repository = message_repository_mocking_update(&thread_id, &expected_update_message);
        let instance = MessageDeltaUpdateService::new(Arc::new(repository));

        let (delta, _message) = instance
            .from_chunk(&chunk, &existing_message)
            .await
            .unwrap();

        assert_eq!(delta.delta.role, "assistant");
    }

    #[tokio::test]
    async fn test_delta_message_id_is_same_as_message() {
        let thread_id = "some_thread_id";
        let message_id = "some_message_id";
        let expected_update_message = DbUpdateThreadMessageDto {
            content: Some(vec![ApiMessageContent::text("Hello World!")]),
            ..DbUpdateThreadMessageDto::default()
        };
        let existing_message = ThreadMessageDto {
            id: message_id.to_string(),
            thread_id: Some(thread_id.to_string()),
            content: vec![ApiMessageContent::text("Hello ")],
            role: "assistant".to_string(),
            ..ThreadMessageDto::default()
        };
        let chunk = ChatCompletionChunkObject {
            choices: vec![ChatCompletionChunkChoice {
                delta: Some(ChatCompletionMessageDto {
                    role: "assistant".to_string(),
                    content: "World!".to_string(),
                }),
                ..ChatCompletionChunkChoice::default()
            }],
            ..ChatCompletionChunkObject::default()
        };

        let repository = message_repository_mocking_update(&thread_id, &expected_update_message);
        let instance = MessageDeltaUpdateService::new(Arc::new(repository));

        let (delta, _message) = instance
            .from_chunk(&chunk, &existing_message)
            .await
            .unwrap();

        assert_eq!(delta.id, message_id);
    }

    fn message_repository_mocking_update(
        thread_id: &str,
        update_message_dto: &DbUpdateThreadMessageDto,
    ) -> MockMessageRepository {
        let mut message_repository = MockMessageRepository::new();
        let thread_id = thread_id.to_string();
        let update_message_dto = update_message_dto.clone();
        let update_message_dto_with = update_message_dto.clone();

        message_repository
            .expect_update()
            .withf_st(move |_id, dto| *dto == update_message_dto_with)
            .returning(move |id, dto| {
                let thread_id = thread_id.clone();
                let status = dto.status.clone().unwrap_or("".to_string());
                let id = id.to_string();

                Box::pin(async {
                    Ok(ThreadMessageDto {
                        id: id,
                        thread_id: Some(thread_id),
                        status: status,
                        role: "assistant".to_string(),
                        content: vec![ApiMessageContent::text("Hello World!")],
                        ..ThreadMessageDto::default()
                    })
                })
            });

        message_repository
    }
}
