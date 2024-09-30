#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::assistant::domain::{
        dto::ThreadMessageDto, message::message_repository::MockMessageRepository, thread::ThreadMessageFactory,
    };

    #[tokio::test]
    async fn test_create_thread_message_role_is_assistant() {
        let repository_created_message = ThreadMessageDto {
            role: "assistant".to_string(),
            thread_id: Some("some_thread_id".to_string()),
            run_id: Some("some_run_id".to_string()),
            ..ThreadMessageDto::default()
        };
        let message_repository = message_repository_mocking_create(&repository_created_message);
        let factory = ThreadMessageFactory::new(Arc::new(message_repository));

        let message = factory
            .create_assistant("some_thread_id", "some_run_id")
            .await
            .unwrap();

        assert_eq!(message.role, "assistant");
    }

    #[tokio::test]
    async fn test_create_thread_message_thread_id_is_same_as_provided() {
        let repository_created_message = ThreadMessageDto {
            role: "assistant".to_string(),
            thread_id: Some("some_thread_id".to_string()),
            run_id: Some("some_run_id".to_string()),
            ..ThreadMessageDto::default()
        };
        let message_repository = message_repository_mocking_create(&repository_created_message);
        let factory = ThreadMessageFactory::new(Arc::new(message_repository));

        let message = factory
            .create_assistant("some_thread_id", "some_run_id")
            .await
            .unwrap();

        assert_eq!(message.thread_id.unwrap(), "some_thread_id");
    }

    #[tokio::test]
    async fn test_create_thread_message_run_id_is_same_as_provided() {
        let repository_created_message = ThreadMessageDto {
            role: "assistant".to_string(),
            thread_id: Some("some_thread_id".to_string()),
            run_id: Some("some_run_id".to_string()),
            ..ThreadMessageDto::default()
        };
        let message_repository = message_repository_mocking_create(&repository_created_message);
        let factory = ThreadMessageFactory::new(Arc::new(message_repository));

        let message = factory
            .create_assistant("some_thread_id", "some_run_id")
            .await
            .unwrap();

        assert_eq!(message.run_id.unwrap(), "some_run_id");
    }

    #[tokio::test]
    async fn test_create_thread_message_content_is_empty() {
        let repository_created_message = ThreadMessageDto {
            role: "assistant".to_string(),
            thread_id: Some("some_thread_id".to_string()),
            run_id: Some("some_run_id".to_string()),
            ..ThreadMessageDto::default()
        };
        let message_repository = message_repository_mocking_create(&repository_created_message);
        let factory = ThreadMessageFactory::new(Arc::new(message_repository));

        let message = factory
            .create_assistant("some_thread_id", "some_run_id")
            .await
            .unwrap();

        assert_eq!(message.content.len(), 0);
    }

    fn message_repository_mocking_create(
        repository_created_message: &ThreadMessageDto,
    ) -> MockMessageRepository {
        let mut message_repository = MockMessageRepository::new();
        let input_message_dto = repository_created_message.clone();
        let output_message_dto = ThreadMessageDto {
            id: "123".to_string(),
            ..input_message_dto.clone()
        };

        let thread_id = input_message_dto.thread_id.clone().unwrap();
        let run_id = input_message_dto.run_id.clone().unwrap();
        let role = input_message_dto.role.clone();

        message_repository
            .expect_create()
            .withf_st(move |x| {
                let called_run_id = x.run_id.clone().unwrap();
                let called_thread_id_id = x.thread_id.clone().unwrap();

                called_run_id == run_id && called_thread_id_id == thread_id && x.role == role
            })
            .returning(move |_| {
                let output_message_dto = output_message_dto.clone();
                Box::pin(async { Ok(output_message_dto) })
            });

        message_repository
    }
}
